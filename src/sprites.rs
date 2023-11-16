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

pub fn create_sprites() -> Vec<GPUSprite> {
    let mut sprites: Vec<GPUSprite> = vec![
        GPUSprite { //0 somethin weird is happenign where 1-4 are not showing up
            screen_region: [WINDOW_WIDTH/2.0, -16.0, CELL_WIDTH*2.0, CELL_HEIGHT],
            sheet_region: [0.5, 0.0, 0.5, 0.5/3.0], // green brick PLATFORM SPRITE
        },
        GPUSprite { //1
            screen_region: [128.0, 32.0, CELL_WIDTH, CELL_HEIGHT],
            sheet_region: [0.0, 2.0 / 3.0, 0.5, 0.5 / 3.0], // yellow brick
        },
        GPUSprite { //2
            screen_region: [128.0, 64.0, CELL_WIDTH, CELL_HEIGHT],
            sheet_region: [0.0, 1.0 / 3.0, 0.5, 0.5 / 3.0], // blue brick
        },
        GPUSprite { //3
            screen_region: [128.0, 128.0, CELL_WIDTH, CELL_HEIGHT],
            sheet_region: [0.0, 0.0, 0.5, 0.5/3.0], // pink brick
        },
        GPUSprite { //4
            screen_region: [128.0, 300.0, CELL_WIDTH, CELL_HEIGHT],
            sheet_region: [0.5, 1.0 / 3.0, 0.5, 0.5 / 3.0], // purple brick
        },
        GPUSprite { //5
            screen_region: [128.0, 500.0, 64.0, 96.0],
            sheet_region: [0.5, 0.5, 0.5, 0.5], // ball - for physics
        },
    ];

    sprites
}




pub fn move_platform(input: &Input, mut sprite_position: [f32; 2]) -> [f32; 2] {
    if input.is_key_down(winit::event::VirtualKeyCode::Left) {
        sprite_position[0] -= 5.0;
    }
    if input.is_key_down(winit::event::VirtualKeyCode::Right) {
        sprite_position[0] += 5.0;
    }  
    sprite_position
}

