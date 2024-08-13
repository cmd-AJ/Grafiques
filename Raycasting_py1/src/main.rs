use gilrs::Gilrs;
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{distance, Vec2};
use std::{f32::consts::PI, sync::Arc, time::Duration};
mod framebuffer;
use framebuffer::Framebuffer;
mod render;
use render::load_maze;
mod player;
use player::{Player, process_events};
mod castray;
use castray::{cast_ray, Intersect};
use once_cell::sync::Lazy;
mod texture;
use texture::Texture;
mod music;
use music::AudioPlayer;
use rodio::{Decoder, OutputStream, Sink};
use std::{time::Instant}; // Add this import


static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/cornfields.jpg")));
static JB1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/JB1.png")));
static PLAYER: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/mirror.png")));


fn cell_texture_coloring(cell:char, tx:u32, ty:u32) -> u32{
    let default_color  = 0xFF1155;

    match cell {
        '+' => WALL1.get_pixel_color(tx, ty),
        '-' => WALL1.get_pixel_color(tx, ty),
        '|' => WALL1.get_pixel_color(tx, ty),
        'g' => WALL1.get_pixel_color(tx, ty),
        _ => default_color
    }
}

fn cell_coloring(cell:char) -> u32{
    let default_color  = 0xFF1155;

    match cell {
        '+' => 0xFF1155,
        '-' => 0x221155,
        '|' => 0x00FF00,
        'g' => 0xF1F10F,
        _ => default_color
    }
}


fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char){
    for x in xo..xo + block_size{
        for y in yo..yo + block_size{
            if cell != ' ' {
                let color = cell_coloring(cell);
                framebuffer.set_foreground_color(color);
                framebuffer.point(x, y)
            }

        }
    }
}


fn render2d(framebuffer: &mut Framebuffer, player: &Player){
    let maze = load_maze("./labterinto.txt");
    let block_size = 69;

    for row in 0..maze.len(){
        for col in 0..maze[row].len(){
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col])
        }
    }

    framebuffer.set_foreground_color(0xFF0000);
    framebuffer.point(player.pos.x as usize, player.pos.y as usize);
    let num_rays = 10;
            for i in 0..num_rays{
                let current_ray = (i as f32) / (num_rays as f32);
                let a = player.a - (player.fov/2.0) + (player.fov * current_ray);
                cast_ray(framebuffer, &maze, player, a, block_size, true);
            }

    
}

fn render3d(framebuffer: &mut Framebuffer, player: &Player,z_buffer: &mut[f32]){
    let maze = load_maze("./labterinto.txt");
    let block_size = 69;
    let num_rays = framebuffer.width;

    for i in 0..framebuffer.width{
        for j in 0..(framebuffer.height/2){
            if (j % 105) != 0 {
                framebuffer.set_foreground_color(0x85b1d6); 
            }
            else if (j % 16) == 6 {
                framebuffer.set_foreground_color(0xFFFFFF); 
            }
            else {
                framebuffer.set_foreground_color(0xFFFFFF);
            }
            framebuffer.point(i,j);
        }
    }
    for i in 0..framebuffer.width{
        for j in (framebuffer.height/2)..(framebuffer.height/2){
            framebuffer.set_foreground_color(0x0000FF);
            framebuffer.point(i,j);
        }
    }

    let hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;

    framebuffer.set_foreground_color(0xF500F5);
    
    for i in  0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov/2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

        let distance = intersect.distance * (a - player.a).cos();
        let stake_height = (framebuffer.height as f32 / distance as f32) * 110.0;

        let stake_top = (hh - (stake_height/2.0)) as usize;
        let stake_bottom = (hh + (stake_height/2.0)) as usize;
        
        z_buffer[i] = distance;

        for y in stake_top..stake_bottom {
            let ty = (y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32) * 335.0; //335
            let tx = intersect.tx;
            let color = cell_texture_coloring(intersect.impact, tx as u32, ty as u32);
            framebuffer.set_foreground_color(color);
            framebuffer.point(i, y)
        }
    }

   


}

fn render_enemy(framebuffer: &mut Framebuffer, player: &Player, pos:&Vec2, z_buffer: &mut[f32]){
    let sprite_a = (pos.y - player.pos.y).atan2(pos.x - player.pos.x);
    
    if sprite_a < 0.0 {
        return;
    }
    let sprite_d = ((player.pos.x - pos.x).powi(2) + (player.pos.y - pos.y).powi(2)).sqrt();
    // let sprite_d = distance(player, p);

    if sprite_d < 20.0 {
        return;
    }

    let screenheight = framebuffer.height as f32;
    let screenwidth = framebuffer.width as f32;

    
    let sprite_size = (screenheight / sprite_d) * 100.0;
    let start_y = (screenheight as f32 / 2.0) - (sprite_size /2.0 );
    let start_x = (sprite_a - player.a) * (screenheight / player.fov) + (screenwidth as f32 / 2.0) - (sprite_size /2.0 );

    let end_x = (start_x + sprite_size).min(framebuffer.width as f32) as usize;
    let end_y = (start_y + sprite_size).min(framebuffer.height as f32) as usize;
    let start_x = start_x.max(0.0) as usize;
    let start_y = start_y.max(0.0) as usize;

    if end_x <= 0 {
        return;
    }
    

    if  start_x < framebuffer.width && sprite_d < z_buffer[start_x] {
    for x in start_x..end_x{
            for y in start_y..end_y{
                let tx = ((x - start_x ) * 330 / sprite_size as usize) as u32;
                let ty = ((y - start_y ) * 330 / sprite_size as usize) as u32;
                let color = JB1.get_pixel_color(tx, ty);
                framebuffer.set_foreground_color(color);
                if color != 0xf500ff {
                    framebuffer.point(x, y);
                }
                z_buffer[x] = sprite_d;
            }
        }
    }
}

fn render_enemies(framebuffer: &mut Framebuffer, player: &Player, z_buffer: &mut[f32]){
    let enemies = vec![
        Vec2::new(250.0, 250.0),
        Vec2::new(600.0, 180.0), //Que tan lejos izquierda
        Vec2::new(250.0, 600.0), //Que tan lejos derecha 
    ];

    for enemy in enemies{
        render_enemy(framebuffer, &player, &enemy, z_buffer);
    }
}

fn render_ui(framebuffer: &mut Framebuffer){
    let ui_width = 320 as u32;
    let ui_height = 320 as u32;
    let ui_x = ((framebuffer.width as f32 /2.0 + 100.0) - (ui_width as f32 /2.0)) as u32;
    let ui_y = (framebuffer.height as f32 - ui_height as f32) as u32;
    
    for x in ui_x..(ui_x + ui_width){
        for y in ui_y..(ui_y + ui_height){
            
            let tx = x - ui_x;
            let ty = y - ui_y;

            let color = PLAYER.get_pixel_color(tx, ty);
            framebuffer.set_foreground_color(color);
            if color != 0xF500FE {
                    framebuffer.point(x as usize, y as usize);
            }
        }
    }
}

fn render_minimap(framebuffer: &mut Framebuffer, player: &Player, scale: usize) {
    let maze = load_maze("./labterinto.txt");
    let block_size = 69 / scale;

    // Define the minimap area dimensions and position based on framebuffer size
    let minimap_width = framebuffer.width / 5; // Example: Minimap width is a quarter of framebuffer width
    let minimap_height = framebuffer.height / 5; // Example: Minimap height is a quarter of framebuffer height

    // Fill the minimap area with white color
    framebuffer.set_foreground_color(0xFFFFFF); // White color
    for x in 0..(minimap_width-15 as usize) {
        for y in 0..(minimap_height-15 as usize) {
            framebuffer.point(x, y);
        }
    }

    // Draw the maze on the minimap
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
        }
    }

    // Draw the player's position on the minimap
    framebuffer.set_foreground_color(0xFF0000); // Red color for player position
    let player_x = (player.pos.x as usize) / scale;
    let player_y = (player.pos.y as usize) / scale;
    framebuffer.point(player_x, player_y);
}


fn main() {
    let window_width = 1300;
    let window_height = 900;

    let framebuffer_width = 1300;
    let framebuffer_height = 900;

    let frame_delay = Duration::from_millis(0);
    let mut gilrs = Gilrs::new().unwrap();

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    
    let audioplay = Arc::new(AudioPlayer::new("assets/jb.mp3"));
    audioplay.clone().play_in_background();

    let mut window = Window::new(
        "VG and LCalibre",
        window_width, window_height, WindowOptions::default()
    ).unwrap();

    window.set_position(100, 100);
    window.update();

    framebuffer.set_background_color(0x6b6565);
    let mut player = Player{
        pos: Vec2::new(150.0,150.0),
        a:PI/3.0,
        fov:PI/3.0
    };

    let mut last_time = Instant::now();
    let mut frame_count = 0;

    while window.is_open() {

        frame_count += 1;
        
        
        // Listen to inputs
        let mut mode = "3D";
        if window.is_key_down(Key::Escape) {
            break;
        }
        if window.is_key_down(Key::M) {
            
            audioplay.clone().stop_in_background();
        }
        
        if window.is_key_down(Key::O)  {
            mode = if mode == "2D" { "3D"} else { "2D" };
        }
        process_events(&window, &mut player, &load_maze("./labterinto.txt"), 69, &mut gilrs );
        framebuffer.clear();

        if mode == "2D" {
            render2d(&mut framebuffer, &player);
        }
        else {
            let mut z_buffer = vec![f32::INFINITY; framebuffer.width];
            render3d(&mut framebuffer, &player, &mut z_buffer);
            render_enemies(&mut framebuffer, &player, &mut z_buffer);
            render_ui(&mut framebuffer);
            render_minimap(&mut framebuffer,&player, 5);
        }

        // Calculate FPS every second
        let now = Instant::now();
        if now.duration_since(last_time).as_secs() >= 1 {
            let fps = frame_count;
            frame_count = 0;
            last_time = now;

            // Update the window title with FPS
            let title = format!("VG and LCalibre - FPS: {}", fps);
            window.set_title(&title);
        }

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }

    
}

