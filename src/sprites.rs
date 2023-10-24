use bytemuck::{Pod, Zeroable};
use crate::{WINDOW_WIDTH, WINDOW_HEIGHT, NUMBER_OF_CELLS, CELL_WIDTH, CELL_HEIGHT};
use crate::input::Input;

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
pub struct GPUSprite {
    pub screen_region: [f32; 4],
    pub sheet_region: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
pub struct GPUCamera {
    pub screen_pos: [f32; 2],
    pub screen_size: [f32; 2],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SpriteOption {
    Storage,
    Uniform,
    VertexBuffer,
}

pub fn create_sprites() ->  Vec<GPUSprite> {

    let mut sprites: Vec<GPUSprite> = vec![GPUSprite {
        screen_region: [WINDOW_WIDTH/2.0, 32.0, 64.0, 64.0],
        sheet_region: [0.0, 0.0, 0.5, 0.35], // duck
    }];

    sprites.push(GPUSprite {
        screen_region: [512.0, 0.0, 64.0, 64.0],
        sheet_region: [0.5, 11.0/28.0, 13.0/28.0, 6.0/28.0], // starting landing pad
        // sprite_dir: SpriteDir(0),
    });


    for y in (1..NUMBER_OF_CELLS-1).step_by(2) {
        // Create a horizontal line of stars, asteroids, and bombs
        // for x in 1..3 {
            let y_value = y as f32 * CELL_HEIGHT;

            // LOG
            sprites.push(GPUSprite {
                screen_region: [1 as f32 * CELL_WIDTH, y_value, 64.0, 16.0],
                sheet_region: [0.5, 4.0/28.0, 0.5, 1.25/28.0], // big log
                // sprite_dir: SpriteDir(0),
            });
            sprites.push(GPUSprite {
                screen_region: [2 as f32 * CELL_WIDTH, y_value, 64.0, 16.0],
                sheet_region: [0.5, 4.0/28.0, 0.5, 1.25/28.0], // big log
            });
            sprites.push(GPUSprite {
                screen_region: [3 as f32 * CELL_WIDTH, y_value, 64.0, 16.0],
                sheet_region: [0.5, 4.0/28.0, 0.5, 1.25/28.0], // big log
            });

            // LILLYPAD
            sprites.push(GPUSprite {
                screen_region: [6 as f32 * CELL_WIDTH, y_value, 60.0, 50.0],
                sheet_region: [0.5, 11.0/28.0, 13.0/28.0, 6.0/28.0], // lillypad
            });
            sprites.push(GPUSprite {
                screen_region: [7 as f32 * CELL_WIDTH, y_value, 60.0, 50.0],
                sheet_region: [0.5, 11.0/28.0, 13.0/28.0, 6.0/28.0], // lillypad
            });
            sprites.push(GPUSprite {
                screen_region: [8 as f32 * CELL_WIDTH, y_value, 60.0, 50.0],
                sheet_region: [0.5, 11.0/28.0, 13.0/28.0, 6.0/28.0], // lillypad
            });

            // FLOWER
            sprites.push(GPUSprite {
                screen_region: [11 as f32 * CELL_WIDTH, y_value, 60.0, 50.0],
                sheet_region: [0.75/28.0, 20.0/28.0, 7.0/16.5, 6.5/28.0], // flower lillypad
            });
            sprites.push(GPUSprite {
                screen_region: [12 as f32 * CELL_WIDTH, y_value, 60.0, 50.0],
                sheet_region: [0.75/28.0, 20.0/28.0, 7.0/16.5, 6.5/28.0], // flower lillypad
            });
            sprites.push(GPUSprite {
                screen_region: [13 as f32 * CELL_WIDTH, y_value, 60.0, 50.0],
                sheet_region: [0.75/28.0, 20.0/28.0, 7.0/16.5, 6.5/28.0], // flower lillypad
            });
        // }
    }
    sprites

}

pub fn move_sprite_input(input: &Input, mut sprite_position: [f32; 2]) -> [f32; 2] {
        // Update sprite position based on keyboard input
        if input.is_key_pressed(winit::event::VirtualKeyCode::Up) {
            if sprite_position[1] + CELL_HEIGHT < WINDOW_HEIGHT {
                sprite_position[1] += 1.5*CELL_HEIGHT;
            } else {
                sprite_position[1] = WINDOW_HEIGHT - CELL_HEIGHT;
            }
        }
        
        if input.is_key_pressed(winit::event::VirtualKeyCode::Down) {
            sprite_position[1] -= 1.5*CELL_HEIGHT;

            if sprite_position[1] < 0.0 {
                sprite_position[1] = 0.0;
            }
        }
        if input.is_key_pressed(winit::event::VirtualKeyCode::Left) {
            sprite_position[0] -= CELL_WIDTH;
        }
        if input.is_key_pressed(winit::event::VirtualKeyCode::Right) {
            sprite_position[0] += CELL_WIDTH;
        }  
        sprite_position
}

