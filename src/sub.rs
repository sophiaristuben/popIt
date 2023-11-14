use std::{borrow::Cow, mem, path::Path};
use winit::{
    event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod input;
mod gpu;
mod sprites;
use sprites::{GPUCamera, SpriteOption, GPUSprite};

struct Game {
    paddle: GPUSprite,
    ball: GPUSprite,
    // Additional game state fields as needed
}

impl Game {
    // Assuming a constant for paddle movement speed
    const PADDLE_MOVE_SPEED: f32 = 5.0;

    fn new(sprites: Vec<GPUSprite>) -> Self {
        // Assuming the first sprite is the paddle and the second is the ball
        Self {
            paddle: sprites[0],
            ball: sprites[1],
        }
    }

    fn update(&mut self) {
        // Update game logic here, such as moving the ball
        // Example values for ball movement speed
        let ball_speed_x = 2.0; // Speed of the ball in the x-direction
        let ball_speed_y = 2.0; // Speed of the ball in the y-direction
 
        // Move the ball
        self.ball.screen_region[0] += ball_speed_x;
        self.ball.screen_region[1] += ball_speed_y;
 
        // Check for collisions with the window edges
        if self.ball.screen_region[0] <= 0.0 || 
            self.ball.screen_region[0] + self.ball.screen_region[2] >= WINDOW_WIDTH {
            // Reverse the x-direction of the ball
            ball_speed_x = -ball_speed_x;
         }
        if self.ball.screen_region[1] <= 0.0 {
            // Reverse the y-direction of the ball
            ball_speed_y = -ball_speed_y;
         }
 
        // Check for collision with the paddle
        if self.ball.screen_region[1] + self.ball.screen_region[3] >= self.paddle.screen_region[1] &&
            self.ball.screen_region[0] + self.ball.screen_region[2] >= self.paddle.screen_region[0] &&
            self.ball.screen_region[0] <= self.paddle.screen_region[0] + self.paddle.screen_region[2] {
            // Reverse the y-direction of the ball
            ball_speed_y = -ball_speed_y;
         }
 
        // TODO: Implement what happens if the ball misses the paddle (game over scenario)
    }

    fn input(&mut self, input: KeyboardInput) {
        // Handle input for moving the paddle
        if let Some(keycode) = input.virtual_keycode {
            match keycode {
                VirtualKeyCode::Left => {
                    // Move paddle left
                    self.paddle.screen_region[0] -= Game::PADDLE_MOVE_SPEED;
                }
                VirtualKeyCode::Right => {
                    // Move paddle right
                    self.paddle.screen_region[0] += Game::PADDLE_MOVE_SPEED;
                }
                _ => {}

                /*
                // Handles edge
                const WINDOW_WIDTH: f32 = 800.0; // Example window width

                // Inside the match statement:
                VirtualKeyCode::Left => {
                    self.paddle.screen_region[0] = (self.paddle.screen_region[0] - Game::PADDLE_MOVE_SPEED).max(0.0);
                }
                VirtualKeyCode::Right => {
                    let max_x = WINDOW_WIDTH - self.paddle.screen_region[2]; // Assumes [2] is the width of the paddle
                    self.paddle.screen_region[0] = (self.paddle.screen_region[0] + Game::PADDLE_MOVE_SPEED).min(max_x);
                }
                */
            }
        }
    }

    fn render(&self) {
        // Rendering code using wgpu
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).expect("Failed to create window");

    let sprites = vec![
        // Initialize your sprites here
        // For example, GPUSprite for paddle and ball
    ];

    let mut game = Game::new(sprites);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => {
                        game.input(input);
                    }
                    _ => {}
                }
            }
            Event::MainEventsCleared => {
                // Update game state
                game.update();
            }
            Event::RedrawRequested(_) => {
                // Render the game
                game.render();
            }
            _ => {}
        }
    });
}
