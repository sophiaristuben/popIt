use bytemuck::{Pod, Zeroable};
use crate::{WINDOW_WIDTH, WINDOW_HEIGHT, NUMBER_OF_CELLS, CELL_WIDTH, CELL_HEIGHT};
use crate::input::Input;
use rand::{Rng, thread_rng};



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

pub fn create_sprites() ->  (Vec<GPUSprite>, Vec<&'static str>) {
    let mut sprites: Vec<GPUSprite> = vec![
        GPUSprite { //0 
            screen_region: [WINDOW_WIDTH/2.0, 30.0, CELL_WIDTH*2.0, CELL_HEIGHT*0.8],
            sheet_region: [0.1,0.18, 0.1, 0.1], // brick,
        },
        GPUSprite { //1
            screen_region: [128.0, 500.0, 78.0, 64.0],
            sheet_region: [0.0, 0.0, 0.2, 0.2], // ball - for physics
        },
    ];

    let mut rng = thread_rng();
    let brick_color_region = [0.1, 0.18, 0.1, 0.1];
    let coin_color_region = [0.0, 0.3, 0.18, 0.22];
    let mut coin_count = 0;

    let mut sprite_tracker = vec!["brick", "ball"];

    for row in 0..5 {
        for col in 0..(NUMBER_OF_CELLS+1) as usize {
            let x_position = col as f32 * CELL_WIDTH;
            let y_position = WINDOW_HEIGHT - (row as f32 * CELL_HEIGHT);


            let mut color_region = brick_color_region; 
            let mut screen_region: [f32; 4] =  [x_position, y_position, CELL_WIDTH, CELL_HEIGHT*0.8];
            let mut sprite_type = "brick";

            if rng.gen::<f32>() < 0.1 && coin_count < 5 {
                color_region = coin_color_region;
                screen_region = [x_position, y_position, CELL_WIDTH, CELL_HEIGHT];
                coin_count += 1;
                sprite_type = "coin";
            }

            sprites.push(GPUSprite {
                screen_region: screen_region,
                sheet_region: color_region,
            });
            sprite_tracker.push(sprite_type);
        }
    }
    
    (sprites, sprite_tracker)
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