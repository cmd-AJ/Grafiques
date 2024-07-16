use nalgebra_glm::Vec3;
use crate::framebuffer::{Framebuffer, Vecline};

#[derive(Clone)]
pub struct Edge {
    y_max: isize,
    x_min: f32,
    inv_slope: f32,
}

fn build_edge_table(vertices: &Vec<Vec3>) -> Vec<Vec<Edge>> {
    let mut edge_table = vec![Vec::new(); vertices.len()];

    for i in 0..vertices.len() {
        let start = &vertices[i];
        let end = &vertices[(i + 1) % vertices.len()];

        if start.y == end.y {
            continue;
        }

        let (y_min, y_max, x_at_y_min, inv_slope) = if start.y < end.y {
            (start.y as isize, end.y as isize, start.x as f32, (end.x - start.x) / (end.y - start.y))
        } else {
            (end.y as isize, start.y as isize, end.x as f32, (start.x - end.x) / (start.y - end.y))
        };

        edge_table[y_min as usize].push(Edge {
            y_max,
            x_min: x_at_y_min,
            inv_slope,
        });
    }

    edge_table
}

fn fill_polygon(framebuffer: &mut Framebuffer, vertices: Vec<Vec3>, color: u32) {
    let edge_table = build_edge_table(&vertices);
    let mut active_edge_table = Vec::new();

    for y in 0..framebuffer.height as isize {
        active_edge_table.retain(|edge| edge.y_max > y);

        for edge in &edge_table[y as usize] {
            active_edge_table.push(edge.clone());
        }

        active_edge_table.sort_by(|a, b| a.x_min.partial_cmp(&b.x_min).unwrap());

        let mut fill = false;
        let mut x_prev = 0.0;

        for edge in &active_edge_table {
            if fill {
                for x in x_prev as isize..edge.x_min as isize {
                    framebuffer.set_pixel(x, y, color);
                }
            }

            fill = !fill;
            x_prev = edge.x_min;
        }

        for edge in &mut active_edge_table {
            edge.x_min += edge.inv_slope;
        }
    }
}
