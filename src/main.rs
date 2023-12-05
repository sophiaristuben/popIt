use std::{borrow::Cow, mem, path::Path, collections::HashMap};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
mod input;
mod gpu;
mod sprites;
use sprites::{GPUCamera, SpriteOption, GPUSprite};

#[cfg(all(not(feature = "uniforms"), not(feature = "vbuf")))]
const SPRITES: SpriteOption = SpriteOption::Storage;
#[cfg(feature = "uniforms")]
const SPRITES: SpriteOption = SpriteOption::Uniform;
#[cfg(feature = "vbuf")]
const SPRITES: SpriteOption = SpriteOption::VertexBuffer;
#[cfg(all(feature = "vbuf", feature = "uniform"))]
compile_error!("Can't choose both vbuf and uniform sprite features");

// ask about how we can auto set this
pub const  WINDOW_WIDTH: f32 = 970.0;
pub const  WINDOW_HEIGHT: f32 = 700.0;

pub const NUMBER_OF_CELLS: i32 = 16;

// here divide by a number to create the number of grids
pub const CELL_WIDTH: f32 = WINDOW_WIDTH / NUMBER_OF_CELLS as f32;
pub const CELL_HEIGHT: f32 = WINDOW_HEIGHT / NUMBER_OF_CELLS as f32;

async fn run(event_loop: EventLoop<()>, window: Window) {
    // let mut gpu = gpu::GPUState::new(&window);
    // let mut input = input::Input::default();
    // let mut renderer = sprites::SpriteRenderer::new(&gpu);
    let mut game_over = false; 
    let mut you_won = false;
    let mut coin_gained = 0;
    
    let mut gpu = gpu::WGPU::new(&window).await; //added to
    
    log::info!("Use sprite mode {:?}", SPRITES);

    // Load the shaders from disk
    let shader = gpu.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    let shader2 = gpu.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader2.wgsl"))),
    });

    let texture_bind_group_layout =
        gpu.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            // It needs the first entry for the texture and the second for the sampler.
            // This is like defining a type signature.
            entries: &[
                // The texture binding
                wgpu::BindGroupLayoutEntry {
                    // This matches the binding in the shader
                    binding: 0,
                    // Only available in the fragment shader
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // It's a texture binding
                    ty: wgpu::BindingType::Texture {
                        // We can use it with float samplers
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        // It's being used as a 2D texture
                        view_dimension: wgpu::TextureViewDimension::D2,
                        // This is not a multisampled texture
                        multisampled: false,
                    },
                    count: None,
                },
                // The sampler binding
                wgpu::BindGroupLayoutEntry {
                    // This matches the binding in the shader
                    binding: 1,
                    // Only available in the fragment shader
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // It's a sampler
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    // No count
                    count: None,
                },
            ],
        });
    // The camera binding
    let camera_layout_entry = wgpu::BindGroupLayoutEntry {
        // This matches the binding in the shader
        binding: 0,
        // Available in vertex shader
        visibility: wgpu::ShaderStages::VERTEX,
        // It's a buffer
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        // No count, not a buffer array binding
        count: None,
    };
    let sprite_bind_group_layout = match SPRITES {
        SpriteOption::Storage => {
            gpu.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    camera_layout_entry,
                    wgpu::BindGroupLayoutEntry {
                        // This matches the binding in the shader
                        binding: 1,
                        // Available in vertex shader
                        visibility: wgpu::ShaderStages::VERTEX,
                        // It's a buffer
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        // No count, not a buffer array binding
                        count: None,
                    },
                ],
            })
        }
        SpriteOption::Uniform => {
            gpu.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    camera_layout_entry,
                    wgpu::BindGroupLayoutEntry {
                        // This matches the binding in the shader
                        binding: 1,
                        // Available in vertex shader
                        visibility: wgpu::ShaderStages::VERTEX,
                        // It's a buffer
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(SPRITE_UNIFORM_SIZE),
                        },
                        // No count, not a buffer array binding
                        count: None,
                    },
                ],
            })
        }
        SpriteOption::VertexBuffer => {
            gpu.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[camera_layout_entry],
            })
        }
    };
    let pipeline_layout = gpu.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&sprite_bind_group_layout, &texture_bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline_layout_over = gpu.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&texture_bind_group_layout],
        push_constant_ranges: &[],
    });

    let _render_pipeline_full = gpu.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout_over),
        vertex: wgpu::VertexState {
            module: &shader2,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader2,
            entry_point: "fs_main",
            targets: &[Some(gpu.config.format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    let _render_pipeline = gpu.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: match SPRITES {
                SpriteOption::Storage => "vs_storage_main",
                SpriteOption::Uniform => "vs_uniform_main",
                SpriteOption::VertexBuffer => "vs_vbuf_main",
            },
            buffers: match SPRITES {
                SpriteOption::VertexBuffer => &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<GPUSprite>() as u64,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &[
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 0,
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: std::mem::size_of::<[f32; 4]>() as u64,
                            shader_location: 1,
                        },
                    ],
                }],
                _ => &[],
            },
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(gpu.config.format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    let render_pipeline = gpu.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: match SPRITES {
                SpriteOption::Storage => "vs_storage_main",
                SpriteOption::Uniform => "vs_uniform_main",
                SpriteOption::VertexBuffer => "vs_vbuf_main",
            },
            buffers: match SPRITES {
                SpriteOption::VertexBuffer => &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<GPUSprite>() as u64,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &[
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 0,
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: std::mem::size_of::<[f32; 4]>() as u64,
                            shader_location: 1,
                        },
                    ],
                }],
                _ => &[],
            },
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(gpu.config.format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    gpu.surface.configure(&gpu.device, &gpu.config);
    let path_sprites = Path::new("content/sprites.png");
    let (sprite_tex, _sprite_img) = gpu.load_texture(path_sprites, None)
         .await
        .expect("Couldn't load spritesheet texture");
    let view_sprite = sprite_tex.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler_sprite = gpu.device.create_sampler(&wgpu::SamplerDescriptor::default());
    let texture_bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &texture_bind_group_layout,
        entries: &[
            // One for the texture, one for the sampler
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view_sprite),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler_sprite),
            },
        ],
    });
    let camera = GPUCamera {
        screen_pos: [0.0, 0.0],
        screen_size: [1024.0, 768.0],
    };
    let buffer_camera = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: bytemuck::bytes_of(&camera).len() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let sprites_return = sprites::create_sprites();
    let mut sprites = sprites_return.0;
    let sprite_types = sprites_return.1;
    let mut coin_dict: HashMap<usize, &str> = HashMap::new();

    let mut platform_position: [f32; 2] = [WINDOW_WIDTH/2.0, 30.0]; 

    // for the ball motion
    let mut ball_position: [f32; 2] = [WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0];
    let mut ball_velocity: [f32; 2] = [3.0, 7.0]; // Adjust these values as needed

    const SPRITE_UNIFORM_SIZE: u64 = 512 * mem::size_of::<GPUSprite>() as u64;
    let buffer_sprite = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: if SPRITES == SpriteOption::Uniform {
            SPRITE_UNIFORM_SIZE
        } else {
            sprites.len() as u64 * std::mem::size_of::<GPUSprite>() as u64
        },
        usage: match SPRITES {
            SpriteOption::Storage => wgpu::BufferUsages::STORAGE,
            SpriteOption::Uniform => wgpu::BufferUsages::UNIFORM,
            SpriteOption::VertexBuffer => wgpu::BufferUsages::VERTEX,
        } | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let sprite_bind_group = match SPRITES {
        SpriteOption::VertexBuffer => gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &sprite_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer_camera.as_entire_binding(),
            }],
        }),
        _ => gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &sprite_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer_camera.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: buffer_sprite.as_entire_binding(),
                },
            ],
        }),
    };
    gpu.queue.write_buffer(&buffer_camera, 0, bytemuck::bytes_of(&camera));
    gpu.queue.write_buffer(&buffer_sprite, 0, bytemuck::cast_slice(&sprites));
    let mut input = input::Input::default();
    event_loop.run(move |event, _, control_flow| {

        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Reconfigure the surface with the new size
                gpu.resize(size);
                // On macos the window needs to be redrawn manually after resizing
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {

                if game_over {
                    println!("Game Over");
                    *control_flow = ControlFlow::Exit;
                } else if you_won {
                    println!("You Won!");
                    *control_flow = ControlFlow::Exit;
                }

                else {

                    // PLATOFORM MOTION
                    platform_position = sprites::move_platform(&input, platform_position);
                    sprites[0].screen_region[0] = platform_position[0];
                    sprites[0].screen_region[1] = platform_position[1];
                    // PLATOFORM MOTION END

                    // BALL MOTION
                    ball_position[0] += ball_velocity[0];
                    ball_position[1] += ball_velocity[1];

                    // colliding off bricks
                    for i in (2..sprites.len()).rev() {
                        let brick_top = sprites[i].screen_region[1];
                        let brick_bottom = brick_top - CELL_HEIGHT;
                        let brick_left = sprites[i].screen_region[0];
                        let brick_right = brick_left + CELL_WIDTH;
                        if ball_position[1] >= brick_bottom && ball_position[0] > brick_left && ball_position[0] < brick_right{  
                            //println!("collided {} index with {} bottom {} left", i, brick_bottom, brick_left);
                            ball_velocity[1] = -ball_velocity[1];

                            if sprite_types[i] == "coin" {
                                if !coin_dict.contains_key(&i) {
                                    coin_dict.insert(i, sprite_types[i]);
                                    coin_gained += 1;
                                    println!("Coin gained! Total coins: {}", coin_gained);
                                }
                            }
                            // erase the brick
                            sprites[i].screen_region = [0.0, 0.0, 0.0, 0.0];
                        }
                    }
                    

                    // colliding off walls
                    if ball_position[0] < 0.0 || ball_position[0] > WINDOW_WIDTH {
                        ball_velocity[0] = -ball_velocity[0];
                    }
                    if ball_position[1] > WINDOW_HEIGHT {
                        ball_velocity[1] = -ball_velocity[1];
                    }
  
                    // for bouncing off the bottom, comment out later
                    // if ball_position[1] < 0.0 || ball_position[1] > WINDOW_HEIGHT {
                    //     ball_velocity[1] = -ball_velocity[1];
                    // }
                     
        
                    //collision with platform
                    let platform_bottom = platform_position[1];
                    let platform_top = platform_bottom + CELL_HEIGHT*0.8;
                    let platform_left = platform_position[0]-15.0;
                    let platform_right = platform_left + CELL_WIDTH * 2.0;
                    if ball_position[1] < platform_top && 
                        ball_position[1] > platform_bottom && 
                        ball_position[0] > platform_left && 
                        ball_position[0] < platform_right{
                
                            // bounce off platformm
                            ball_velocity[1] = -ball_velocity[1];
                    }
                    

                    // game over
                    if ball_position[1] < 0.0 {
                        game_over = true;
                    }

                    // game win
                    // set the number of bricks
                    if coin_gained > 4 {
                        println!("Yay! You smashed 5 bricks!");
                        you_won = true;

                        for (i, sprite_type) in sprite_types.iter().enumerate() {
                            println!("Index: {}, Sprite Type: {}", i, sprite_type);
                        }
                    }

                    // update ball's screen region in sprites vector
                    sprites[1].screen_region[0] = ball_position[0];
                    sprites[1].screen_region[1] = ball_position[1];
                    sprites[1].screen_region[0] = ball_position[0];
                    sprites[1].screen_region[1] = ball_position[1];
                    // BALL MOTION END


                }
                

                // Then send the data to the GPU!
                input.next_frame();

                gpu.queue.write_buffer(&buffer_camera, 0, bytemuck::bytes_of(&camera));
                gpu.queue.write_buffer(&buffer_sprite, 0, bytemuck::cast_slice(&sprites));

                let frame = gpu.surface
                    .get_current_texture()
                    .expect("Failed to acquire next swap chain texture");
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder =
                    gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                    });
                    rpass.set_pipeline(&render_pipeline);
                    if SPRITES == SpriteOption::VertexBuffer {
                        rpass.set_vertex_buffer(0, buffer_sprite.slice(..));
                    }
                    rpass.set_bind_group(0, &sprite_bind_group, &[]);
                    rpass.set_bind_group(1, &texture_bind_group, &[]);
                    // draw two triangles per sprite, and sprites-many sprites.
                    // this uses instanced drawing, but it would also be okay
                    // to draw 6 * sprites.len() vertices and use modular arithmetic
                    // to figure out which sprite we're drawing.
                    rpass.draw(0..6, 0..(sprites.len() as u32));
                }
                gpu.queue.submit(Some(encoder.finish()));
                frame.present();
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            // WindowEvent->KeyboardInput: Keyboard input!
            Event::WindowEvent {
                // Note this deeply nested pattern match
                event: WindowEvent::KeyboardInput { input: key_ev, .. },
                ..
            } => {
                input.handle_key_event(key_ev);
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                input.handle_mouse_button(state, button);
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                input.handle_mouse_move(position);
            }
            _ => {}
        }
    });
}

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(run(event_loop, window));
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Trace).expect("could not initialize logger");
        use winit::platform::web::WindowExtWebSys;
        // On wasm, append the canvas to the document body
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");
        wasm_bindgen_futures::spawn_local(run(event_loop, window));
    }
}