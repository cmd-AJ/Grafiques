use std::f32::consts::PI;
use gilrs::{Axis, Button, Gilrs, Event};
use nalgebra_glm::Vec2;
use minifb::{Key, Window};

pub struct Player {
    pub pos: Vec2,
    pub a: f32,
    pub fov: f32,
}

pub struct GameState {
    prev_mouse_x: f32,
    prev_mouse_y: f32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            prev_mouse_x: 0.0,
            prev_mouse_y: 0.0,
        }
    }
}

pub fn process_events(
    window: &Window,
    player: &mut Player,
    maze: &Vec<Vec<char>>,
    block_size: usize,
    gilrs: &mut Gilrs,
    game_state: &mut GameState,
) {
    const MOVE_SPEED: f32 = 3.0;

    const ROTATION_SPEED: f32 = PI / 80.0;
    const MOUSE_SENSITIVITY: f32 = 0.01;
    const TWO_PI: f32 = PI * 2.0;

     // Handle mouse movement
     let (mouse_x, mouse_y) = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap_or((0.0, 0.0));
     // Calculate mouse movement delta
     let mouse_delta_x = (mouse_x - game_state.prev_mouse_x) * MOUSE_SENSITIVITY;
    
     // Update player's angle based on mouse movement
     player.a -= -mouse_delta_x;
 
     // Update previous mouse position
     game_state.prev_mouse_x = mouse_x;
     game_state.prev_mouse_y = mouse_y;

    let mut new_pos = player.pos;
    
    if window.is_key_down(Key::A) {
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(Key::D) {
        player.a += ROTATION_SPEED;
    }

    player.a = player.a.rem_euclid(TWO_PI);

    if window.is_key_down(Key::W) {
        new_pos.x += MOVE_SPEED * player.a.cos();
        new_pos.y += MOVE_SPEED * player.a.sin();

        if !is_colliding_with_wall(&new_pos, maze, block_size) {
            player.pos = new_pos;
        }
    }
    if window.is_key_down(Key::S) {
        new_pos.x -= MOVE_SPEED * player.a.cos();
        new_pos.y -= MOVE_SPEED * player.a.sin();

        if !is_colliding_with_wall(&new_pos, maze, block_size) {
            player.pos = new_pos;
        }
    }

    if window.is_key_pressed(Key::NumPad4, minifb::KeyRepeat::No) {
        player.a = 0.0;
    }
    if window.is_key_pressed(Key::NumPad6, minifb::KeyRepeat::No) {
        player.a = PI;
    }
    if window.is_key_pressed(Key::NumPad2, minifb::KeyRepeat::No) {
        player.a = 1.5 * PI;
    }
    if window.is_key_pressed(Key::NumPad8, minifb::KeyRepeat::No) {
        player.a = 0.5 * PI;
    }


    while let Some(Event { event, .. }) = gilrs.next_event() {
        match event {
            gilrs::EventType::ButtonPressed(button, _) => {
                match button {
                    Button::DPadLeft => player.a -= ROTATION_SPEED,
                    Button::DPadRight => player.a += ROTATION_SPEED,
                    Button::DPadUp => {
                        new_pos.x += MOVE_SPEED * player.a.cos();
                        new_pos.y += MOVE_SPEED * player.a.sin();

                        if !is_colliding_with_wall(&new_pos, maze, block_size) {
                            player.pos = new_pos;
                        }
                    }
                    Button::DPadDown => {
                        new_pos.x -= MOVE_SPEED * player.a.cos();
                        new_pos.y -= MOVE_SPEED * player.a.sin();

                        if !is_colliding_with_wall(&new_pos, maze, block_size) {
                            player.pos = new_pos;
                        }
                    }
                    _ => {}
                }
            }
            gilrs::EventType::AxisChanged(axis, value, _) => {
                match axis {
                    Axis::LeftStickX => {
                        if value.abs() > 0.1 {
                            player.a -= ROTATION_SPEED * value.signum();
                        }
                    }
                    Axis::LeftStickY => {
                        if value.abs() > 0.1 {
                            new_pos.x += MOVE_SPEED * player.a.cos() * value;
                            new_pos.y += MOVE_SPEED * player.a.sin() * value;

                            if !is_colliding_with_wall(&new_pos, maze, block_size) {
                                player.pos = new_pos;
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn is_colliding_with_wall(pos: &Vec2, maze: &Vec<Vec<char>>, block_size: usize) -> bool {
        let x = pos.x as usize / block_size;
        let y = pos.y as usize / block_size;

        if x >= maze[0].len() || y >= maze.len() {
            return true; // Out of bounds
        }

        match maze[y][x] {
            '+' | '-' | '|' | 'g' => true, // Wall characters
            _ => false,
        }
    }

    
}
