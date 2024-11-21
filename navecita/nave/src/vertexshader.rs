use nalgebra_glm::{cross, dot, Mat3, Vec4};

use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::Vec3;
use crate::uniform::Uniforms;
use crate::normal_map::{NormalMap, with_normal_map};

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  // Transform position
  let position = Vec4::new(
    vertex.position.x,
    vertex.position.y,
    vertex.position.z,
    1.0
  );
  let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

  // Perform perspective division
  let w = transformed.w;
  let ndc_position = Vec4::new(
    transformed.x / w,
    transformed.y / w,
    transformed.z / w,
    1.0
  );

  // apply viewport matrix
  let screen_position = uniforms.viewport_matrix * ndc_position;

  // Transform normal
  let model_mat3 = Mat3::new(
    uniforms.model_matrix[0], uniforms.model_matrix[1], uniforms.model_matrix[2],
    uniforms.model_matrix[4], uniforms.model_matrix[5], uniforms.model_matrix[6],
    uniforms.model_matrix[8], uniforms.model_matrix[9], uniforms.model_matrix[10]
  );
  let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

  let transformed_normal = normal_matrix * vertex.normal;

  // Create a new Vertex with transformed attributes
  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
    transformed_normal,
  }
}

pub fn calculate_lighting(fragment: &Fragment) -> f32 {
  // Sample the normal map and transform to world space
  let normal_from_map = with_normal_map(|normal_map: &NormalMap| {
      normal_map.sample(fragment.tex_coords.x, fragment.tex_coords.y)
  });
  
  // Combine the normal from the map with the surface normal
  let modified_normal = (fragment.normal + normal_from_map).normalize();
  
  // Calculate lighting with the modified normal
  let light_dir = Vec3::new(0.0, 0.0, 1.0);
  dot(&modified_normal, &light_dir).max(0.0)
}


pub fn calculate_tangent_lighting(fragment: &Fragment) -> f32 {
  // Sample the normal map (comes in tangent space)
  let tangent_normal = with_normal_map(|normal_map: &NormalMap| {
      normal_map.sample(fragment.tex_coords.x, fragment.tex_coords.y)
  });
  
  // Calculate TBN matrix
  let normal = fragment.normal.normalize();
  
  // Calculate tangent and bitangent
  // This is a simple way to get tangent vectors - ideally these would come from the mesh data
  let tangent = if normal.y.abs() < 0.999 {
      cross(&Vec3::new(0.0, 1.0, 0.0), &normal).normalize()
  } else {
      cross(&Vec3::new(0.0, 0.0, 1.0), &normal).normalize()
  };
  let bitangent = cross(&normal, &tangent).normalize();
  
  // Create TBN matrix to transform from tangent space to world space
  let tbn = Mat3::new(
      tangent.x, bitangent.x, normal.x,
      tangent.y, bitangent.y, normal.y,
      tangent.z, bitangent.z, normal.z,
  );
  
  // Transform normal from tangent space to world space
  let world_normal = (tbn * tangent_normal).normalize();
  
  // Calculate lighting with the transformed normal
  let light_dir = Vec3::new(0.0, 0.0, 1.0);
  dot(&world_normal, &light_dir).max(0.0)
}