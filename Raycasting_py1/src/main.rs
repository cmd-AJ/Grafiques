use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec2;
use std::{f32::consts::PI, time::Duration};
mod framebuffer;
use framebuffer::Framebuffer;
mod render;
use render::load_maze;
mod player;
use player::{Player, process_events};
mod castray;
use castray::{cast_ray, Intersect};
mod controller;


fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char){
    for x in xo..xo + block_size{
        for y in yo..yo + block_size{
            if cell != ' ' {
                framebuffer.point(x, y)
            }

            if cell == 'g' {
                framebuffer.set_foreground_color(0xFF1155);
                framebuffer.point(x, y);
                framebuffer.set_foreground_color(0xFF00FF);
            }

            if cell == '|' || cell == '-' {
                framebuffer.set_foreground_color(0x00FF00);
                framebuffer.point(x, y);
                framebuffer.set_foreground_color(0xFF00FF);
            }

            if cell == 'p' {
                framebuffer.set_foreground_color(0x000000);
                framebuffer.point(x, y);
                framebuffer.set_foreground_color(0xFF00FF);
            }

            

        }
    }
}


fn render2d(framebuffer: &mut Framebuffer, player: &Player){
    let maze = load_maze("./labterinto.txt");
    let block_size = 100;

    for row in 0..maze.len(){
        for col in 0..maze[row].len(){
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col])
        }
    }


    framebuffer.point(player.pos.x as usize, player.pos.y as usize);
    cast_ray(framebuffer, &maze, player, player.a, block_size, true);
    
}


fn main() {
    let window_width = 1300;
    let window_height = 900;

    let framebuffer_width = 1300;
    let framebuffer_height = 900;

    let frame_delay = Duration::from_millis(0);

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

  
    let mut window = Window::new(
        "VG and LCalibre",
        window_width, window_height, WindowOptions::default()
    ).unwrap();

    window.set_position(100, 100);
    window.update();

    framebuffer.set_background_color(0x000000);
    let mut player = Player{
        pos: Vec2::new(150.0,150.0),
        a:PI/3.0
    };

    while window.is_open() {
        // Listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }
        process_events(&window, &mut player);
        framebuffer.clear();
        render2d(&mut framebuffer, &player);

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }

    
}

