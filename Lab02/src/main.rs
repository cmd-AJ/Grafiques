use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
mod framebuffer;
use framebuffer::Framebuffer;
mod bmp;

mod game_life;
use game_life::GameOfLife;

fn render(framebuffer: &mut Framebuffer, game: &GameOfLife) {
    framebuffer.set_foreground_color(0xFFDDDD);
    for y in 0..game.height {
        for x in 0..game.width {
            if game.grid[y][x] {
                framebuffer.point(x as usize, y as usize);
            }
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 200;
    let framebuffer_height = 100;

    let frame_delay = Duration::from_millis(36);

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut game = GameOfLife::new(framebuffer_width, framebuffer_height);

    // Manually define initial states for various patterns using set_pattern method
    game.set_pattern("glider", 1, 1);
    game.set_pattern("glider", 2, 1);
    game.set_pattern("glider", 3, 1);

    game.set_pattern("glider", 11, 1);
    game.set_pattern("glider", 12, 2);
    game.set_pattern("glider", 13, 1);
    game.set_pattern("glider", 180, 20);
    game.set_pattern("block", 10, 10);
    game.set_pattern("blinker", 20, c20);
    game.set_pattern("glider", 50, 6);
    game.set_pattern("pulsar", 50, 50);
    game.set_pattern("penta-decathlon", 110, 60);
    game.set_pattern("penta-decathlon", 132, 10);
    game.set_pattern("penta-decathlon", 180, 60);
    game.set_pattern("glider", 100, 30);
    game.set_pattern("glider", 75, 0);
    game.set_pattern("beehive", 70, 10);
    game.set_pattern("toad", 70, 50);
    game.set_pattern("hwss", 110, 50);
    game.set_pattern("hwss", 150, 40);
    game.set_pattern("pulsar", 180, 80);
    game.set_pattern("pulsar", 140, 60);
    game.set_pattern("puff", 180, 10);
    game.set_pattern("boat", 50, 10);
    game.set_pattern("glider", 1, 88);
    game.set_pattern("blinker", 5, 88);
    game.set_pattern("glider", 8, 88);
    game.set_pattern("beacon", 150, 10);
    game.set_pattern("lwss", 150, 10);
    game.set_pattern("mwss", 150, 40);
    game.set_pattern("penta-decathlon", 50, 60);
    game.set_pattern("mwss", 100, 11);
    game.set_pattern("hwss", 160, 70);
    game.set_pattern("mwss", 100, 10);
    game.set_pattern("glider", 170, 40);
    game.set_pattern("glider", 100, 100);
    game.set_pattern("blinker", 80, 20);
    game.set_pattern("glider", 170, 80);
    game.set_pattern("pulsar", 100, 30);
    game.set_pattern("pulsar", 0, 50);
    game.set_pattern("puff", 26, 30);
    game.set_pattern("tub", 195, 85);
    game.set_pattern("block", 40, 95);
    game.set_pattern("block", 80, 95);
    game.set_pattern("block", 100, 95);
    game.set_pattern("glider", 150, 1);
    game.set_pattern("toad", 130, 1);
    game.set_pattern("glider", 135, 86);
    game.set_pattern("puff", 190, 30);
    game.set_pattern("penta-decathlon", 180, 60);  
    game.set_pattern("hwss", 150,90);
    game.set_pattern("glider", 190, 1);
    game.set_pattern("glider", 185, 1);
    game.set_pattern("glider", 188, 1);
  
    let mut window = Window::new(
        "Rust Graphics - Conway's Game of Life",
        window_width, window_height, WindowOptions::default()
    ).unwrap();

    let mut running = false;

    while window.is_open() {
        // Listen to inputs
        if window.is_key_down(Key::Enter) {
            break;
        }

        if window.is_key_down(Key::Space) {
            framebuffer.render_buffer("output.bmp").unwrap();
        }

        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            game.clear();
            // Redefine initial states if needed
            game.set_pattern("glider", 1, 1);
            game.set_pattern("glider", 180, 20);
            game.set_pattern("block", 10, 10);
            game.set_pattern("blinker", 20, 20);
            game.set_pattern("glider", 50, 6);
            game.set_pattern("pulsar", 50, 50);
            game.set_pattern("penta-decathlon", 110, 60);
            game.set_pattern("penta-decathlon", 180, 60);
            game.set_pattern("glider", 100, 30);
            game.set_pattern("glider", 75, 0);
            game.set_pattern("beehive", 70, 10);
            game.set_pattern("toad", 70, 50);
            game.set_pattern("hwss", 110, 50);
            game.set_pattern("hwss", 150, 40);
            game.set_pattern("pulsar", 180, 80);
            game.set_pattern("pulsar", 140, 60);
            game.set_pattern("puff", 180, 10);
            game.set_pattern("boat", 50, 10);
            game.set_pattern("glider", 1, 88);
            game.set_pattern("blinker", 5, 88);
            game.set_pattern("glider", 8, 88);
            game.set_pattern("beacon", 150, 10);
            game.set_pattern("lwss", 150, 10);
            game.set_pattern("mwss", 150, 40);
            game.set_pattern("penta-decathlon", 50, 60);
            game.set_pattern("mwss", 100, 11);
            game.set_pattern("hwss", 160, 70);
            game.set_pattern("mwss", 100, 10);
            game.set_pattern("glider", 170, 40);
            game.set_pattern("glider", 100, 100);
            game.set_pattern("blinker", 80, 20);
            game.set_pattern("blinker", 60, 30);
            game.set_pattern("glider", 170, 80);
            game.set_pattern("pulsar", 100, 30);
            game.set_pattern("pulsar", 0, 50);
            game.set_pattern("puff", 26, 30);
            game.set_pattern("tub", 195, 85);
            game.set_pattern("block", 40, 95);
            game.set_pattern("block", 80, 95);
            game.set_pattern("block", 100, 95);
            game.set_pattern("glider", 150, 1);
            game.set_pattern("toad", 130, 1);
            game.set_pattern("glider", 135, 86);
            game.set_pattern("puff", 190, 30);
            game.set_pattern("penta-decathlon", 180, 60);  
            game.set_pattern("hwss", 150,90);
            game.set_pattern("glider", 190, 1);
            game.set_pattern("glider", 185, 1);
            game.set_pattern("glider", 188, 1);
            game.set_pattern("penta-decathlon", 72, 50); 
            
            
        }

        if window.is_key_pressed(Key::P, minifb::KeyRepeat::No) {
            running = !running;
        }

        // Update the game if running
        if running {
            game.update();
        }

        // Clear the framebuffer
        framebuffer.set_background_color(0x333355);
        framebuffer.clear();

        // Render the game
        render(&mut framebuffer, &game);

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
