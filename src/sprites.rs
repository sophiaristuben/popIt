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
            sheet_region: [0.5, 1.0 / 3.0, 0.5, 0.5 / 3.0], // purple PLATFORM SPRITE
        },
        GPUSprite { //1
            screen_region: [128.0, 500.0, 64.0, 96.0],
            sheet_region: [0.5, 0.5, 0.5, 0.5], // ball - for physics
        },
    ];

    for row in 0..5 {
        for col in 0..(NUMBER_OF_CELLS+1) as usize {
            let x_position = col as f32 * CELL_WIDTH;
            let y_position = WINDOW_HEIGHT - (row as f32 * CELL_HEIGHT);
    
            // Use modulo to cycle through all four colors
            let color_region = match (row % 2, col % 2) {
                (0, 0) => [0.0, 1.0 / 3.0, 0.5, 0.5 / 3.0], // Blue brick
                (0, 1) => [0.0, 0.0, 0.5, 0.5 / 3.0], // Pink brick
                (1, 0) => [0.5, 0.0, 0.5, 0.5/3.0], // Green brick
                (1, 1) => [0.0, 2.0 / 3.0, 0.5, 0.5 / 3.0], // Yellow brick
                _ => unreachable!(),
            };
    
            sprites.push(GPUSprite {
                screen_region: [x_position, y_position, CELL_WIDTH, CELL_HEIGHT],
                sheet_region: color_region,
            });
        }
    }
    
    
    

    // for row in 0..5 {
    //     for col in 0..(WINDOW_WIDTH / CELL_WIDTH) as usize {
    //         let x_position = col as f32 * CELL_WIDTH;
    //         let y_position = WINDOW_HEIGHT - (row as f32 * CELL_HEIGHT);

    //         sprites.push(GPUSprite {
    //             screen_region: [x_position, y_position, CELL_WIDTH, CELL_HEIGHT],
    //             sheet_region: [0.0, 1.0 / 3.0, 0.5, 0.5 / 3.0], // blue brick
    //         });
    //         sprites.push(GPUSprite {
    //             screen_region: [x_position, y_position, CELL_WIDTH, CELL_HEIGHT],
    //             sheet_region: [0.0, 0.0, 0.5, 0.5 / 3.0], // pink brick
    //         });
    //         sprites.push(GPUSprite {
    //             screen_region: [x_position, y_position, CELL_WIDTH, CELL_HEIGHT],
    //             sheet_region: [0.5, 1.0 / 3.0, 0.5, 0.5 / 3.0], // purple brick
    //         });
    //         sprites.push(GPUSprite {
    //             screen_region: [x_position, y_position, CELL_WIDTH, CELL_HEIGHT],
    //             sheet_region: [0.0, 2.0 / 3.0, 0.5, 0.5 / 3.0], // yellow brick
    //         });
    //     }
    // };

    sprites
}




pub fn move_platform(input: &Input, mut platform_position: [f32; 2]) -> [f32; 2] {
    if input.is_key_down(winit::event::VirtualKeyCode::Left) {
        platform_position[0] -= 5.0;
    }
    if input.is_key_down(winit::event::VirtualKeyCode::Right) {
        platform_position[0] += 5.0;
    }  

    // prevent from going off screen
    if platform_position[0] < 0.0 {
        platform_position[0] = 0.0;
    }
    if platform_position[0] + CELL_WIDTH > WINDOW_WIDTH {
        platform_position[0] = WINDOW_WIDTH - CELL_WIDTH;
    }
    platform_position
}

