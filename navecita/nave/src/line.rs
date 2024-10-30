
use crate::vertex::Vertex;
use crate::fragment::Fragment;

pub fn line(a: &Vertex, b: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    // Extract integer coordinates from the vertices
    let x0 = a.position.x as i32;
    let y0 = a.position.y as i32;
    let x1 = b.position.x as i32;
    let y1 = b.position.y as i32;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy; // Initialize error value

    let mut current_x = x0;
    let mut current_y = y0;

    loop {
        // Calculate the depth of the fragment as a linear interpolation between the two vertices
        let t = if (x1 - x0) != 0 {
            (current_x - x0) as f32 / (x1 - x0) as f32
        } else {
            (current_y - y0) as f32 / (y1 - y0) as f32
        };
        let depth = a.position.z * (1.0 - t) + b.position.z * t;

        // Interpolate color between the two vertices
        let color = a.color.lerp(&b.color, t);

        // Create a fragment at the current position
        let fragment = Fragment::new(current_x as f32, current_y as f32, color, depth);
        fragments.push(fragment);

        if current_x == x1 && current_y == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            current_x += sx;
        }
        if e2 <= dx {
            err += dx;
            current_y += sy;
        }
    }

    fragments
}
