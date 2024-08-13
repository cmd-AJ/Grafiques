use std::f32::consts::PI;
use gilrs::{Axis, Button, Gilrs, Event};
use nalgebra_glm::Vec2;
use minifb::{Key, Window};

pub struct Player {
    pub pos: Vec2,
    pub a: f32,
    pub fov: f32,
}

pub fn process_events(
    window: &Window,
    player: &mut Player,
    maze: &Vec<Vec<char>>,
    block_size: usize,
    gilrs: &mut Gilrs,
) {
    const MOVE_SPEED: f32 = 3.0;
    const ROTATION_SPEED: f32 = PI / 80.0;
    const MOUSE_SENSITIVITY: f32 = 0.001;

    let mut new_pos = player.pos;

    if window.is_key_down(Key::Left) {
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(Key::Right) {
        player.a += ROTATION_SPEED;
    }

    if window.is_key_down(Key::Up) {
        new_pos.x += MOVE_SPEED * player.a.cos();
        new_pos.y += MOVE_SPEED * player.a.sin();

        if !is_colliding_with_wall(&new_pos, maze, block_size) {
            player.pos = new_pos;
        }
    }
    if window.is_key_down(Key::Down) {
        new_pos.x -= MOVE_SPEED * player.a.cos();
        new_pos.y -= MOVE_SPEED * player.a.sin();

        if !is_colliding_with_wall(&new_pos, maze, block_size) {
            player.pos = new_pos;
        }
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
