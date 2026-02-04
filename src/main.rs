use pixels::{Pixels, SurfaceTexture};
use raycaster::Raycaster;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::time::{Duration, Instant};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, WindowEvent};
use winit::window::WindowAttributes;
use winit::{
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

mod raycaster;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

enum Texture {
    Color([u8; 4], [u8; 4]),
}

impl Texture {
    fn color(&self, side: bool) -> &[u8; 4] {
        match self {
            Texture::Color(light, dark) => {
                if side {
                    dark
                } else {
                    light
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let mut pressed_keys: HashSet<KeyCode> = HashSet::new();

    let fallback_texture = Texture::Color([255, 0, 255, 255], [128, 0, 128, 255]); // debug pink

    let textures: HashMap<u8, Texture> = HashMap::from([
        (1, Texture::Color([94, 72, 232, 255], [47, 36, 116, 255])), // BLUE
        (2, Texture::Color([232, 72, 94, 255], [116, 36, 47, 255])), // RED
        (3, Texture::Color([72, 232, 94, 255], [36, 116, 47, 255])), // GREEN
    ]);

    let event_loop = EventLoop::new()?;

    let window = {
        let size = LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT);

        #[allow(deprecated)]
        event_loop.create_window(
            WindowAttributes::default()
                .with_title("Raycaster")
                .with_inner_size(size),
        )?
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)?
    };

    let mut raycaster = Raycaster {
        pos_x: 6.0,
        pos_y: 6.0,
        dir_x: -1.0,
        dir_y: 0.0,
        plane_x: 0.0,
        plane_y: 0.66,
        move_speed: 0.05,
        rot_speed: 0.05,
        map: vec![],
    };
    raycaster.load_map_from_file(None)?;

    let mut now = Instant::now();
    let mut frame_count: u32 = 0;

    #[allow(deprecated)]
    let res = event_loop.run(|event, window_target| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => window_target.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                let PhysicalKey::Code(key) = event.physical_key else {
                    return;
                };

                match event.state {
                    ElementState::Pressed => pressed_keys.insert(key),
                    ElementState::Released => pressed_keys.remove(&key),
                };
            }
            WindowEvent::RedrawRequested => {
                for key in &pressed_keys {
                    match key {
                        KeyCode::Escape => window_target.exit(),
                        KeyCode::KeyW => raycaster.move_up(),
                        KeyCode::KeyS => raycaster.move_down(),
                        KeyCode::KeyA => raycaster.turn_left(),
                        KeyCode::KeyD => raycaster.turn_right(),
                        _ => (),
                    }
                }

                pixels.frame_mut().fill(0);
                for vertical_stripe in 0..SCREEN_WIDTH {
                    let camera_x = 2.0 * vertical_stripe as f32 / SCREEN_WIDTH as f32 - 1.0;
                    let ray_dir_x = raycaster.dir_x + raycaster.plane_x * camera_x;
                    let ray_dir_y = raycaster.dir_y + raycaster.plane_y * camera_x;

                    let delta_dist_x = (1.0 / ray_dir_x).abs();
                    let delta_dist_y = (1.0 / ray_dir_y).abs();

                    let (mut side_dist_x, step_x) = if ray_dir_x < 0.0 {
                        (raycaster.pos_x.fract() * delta_dist_x, -1)
                    } else {
                        ((1.0 - raycaster.pos_x.fract()) * delta_dist_x, 1)
                    };

                    let (mut side_dist_y, step_y) = if ray_dir_y < 0.0 {
                        (raycaster.pos_y.fract() * delta_dist_y, -1)
                    } else {
                        ((1.0 - raycaster.pos_y.fract()) * delta_dist_y, 1)
                    };

                    let mut side: bool;
                    let mut map_x = raycaster.pos_x as i32;
                    let mut map_y = raycaster.pos_y as i32;
                    let texture: &Texture;
                    loop {
                        if side_dist_x < side_dist_y {
                            side_dist_x += delta_dist_x;
                            map_x += step_x;
                            side = false;
                        } else {
                            side_dist_y += delta_dist_y;
                            map_y += step_y;
                            side = true;
                        }

                        let cell = raycaster.map[map_x as usize][map_y as usize];

                        if cell > 0 {
                            texture = textures.get(&cell).unwrap_or(&fallback_texture);
                            break;
                        }
                    }

                    let perp_wall_dist = if !side {
                        side_dist_x - delta_dist_x
                    } else {
                        side_dist_y - delta_dist_y
                    };

                    let line_height = (SCREEN_HEIGHT as f32 / perp_wall_dist) as i32;

                    let mut draw_start = -line_height / 2 + SCREEN_HEIGHT as i32 / 2;
                    if draw_start < 0 {
                        draw_start = 0;
                    }

                    let mut draw_end = line_height / 2 + SCREEN_HEIGHT as i32 / 2;
                    if draw_end >= SCREEN_HEIGHT as i32 {
                        draw_end = SCREEN_HEIGHT as i32 - 1;
                    }

                    draw_vertical_line(
                        vertical_stripe as usize,
                        draw_start as usize,
                        draw_end as usize,
                        *texture.color(side),
                        pixels.frame_mut(),
                        SCREEN_WIDTH as usize,
                    );
                }

                if pixels.render().is_err() {
                    window_target.exit();
                }

                frame_count += 1;
                let elapsed = now.elapsed();
                if elapsed >= Duration::from_secs(1) {
                    let fps = frame_count as f64 / elapsed.as_secs_f64();
                    log::info!("fps: {:.1}", fps);

                    frame_count = 0;
                    now = Instant::now();
                }
            }
            WindowEvent::Resized(size) => {
                if pixels.resize_surface(size.width, size.height).is_err() {
                    window_target.exit();
                }
            }
            _ => (),
        },
        Event::AboutToWait => {
            window.request_redraw();
        }
        _ => (),
    });

    Ok(res?)
}

fn draw_vertical_line(
    line: usize,
    y_start: usize,
    y_end: usize,
    color: [u8; 4],
    frame: &mut [u8],
    width: usize,
) {
    for y in y_start..=y_end {
        let idx = (y * width + line) * 4;
        frame[idx..idx + 4].copy_from_slice(&color);
    }
}
