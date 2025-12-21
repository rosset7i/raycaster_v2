use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, WindowEvent};
use winit::window::WindowAttributes;
use winit::{
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

const MAP_WIDTH: usize = 12;
const MAP_HEIGHT: usize = 12;
const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
const MAP: [[u8; MAP_WIDTH]; MAP_HEIGHT] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = {
        let size = LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT);

        #[allow(deprecated)]
        event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("Raycaster")
                    .with_inner_size(size)
                    .with_min_inner_size(size),
            )
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture).unwrap()
    };

    let mut pos_x: f32 = 6.0;
    let mut pos_y: f32 = 6.0;

    let mut dir_x: f32 = -1.0;
    let mut dir_y: f32 = 0.0;

    let mut plane_x: f32 = 0.0;
    let mut plane_y: f32 = 0.66;

    const MOVE_SPEED: f32 = 0.1;
    const ROT_SPEED: f32 = 0.1;

    #[allow(deprecated)]
    let _ = event_loop
        .run(|event, window_target| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),
                WindowEvent::KeyboardInput { event, .. } => {
                    if let PhysicalKey::Code(key) = event.physical_key
                        && event.state == ElementState::Pressed
                    {
                        match key {
                            KeyCode::Escape => window_target.exit(),
                            KeyCode::KeyW => {
                                if MAP[(pos_x + dir_x * MOVE_SPEED) as usize][pos_y as usize] == 0 {
                                    pos_x += dir_x * MOVE_SPEED;
                                }

                                if MAP[pos_x as usize][(pos_y + dir_y * MOVE_SPEED) as usize] == 0 {
                                    pos_y += dir_y * MOVE_SPEED;
                                }
                            }
                            KeyCode::KeyS => {
                                if MAP[(pos_x - dir_x * MOVE_SPEED) as usize][pos_y as usize] == 0 {
                                    pos_x -= dir_x * MOVE_SPEED;
                                }

                                if MAP[pos_x as usize][(pos_y - dir_y * MOVE_SPEED) as usize] == 0 {
                                    pos_y -= dir_y * MOVE_SPEED;
                                }
                            }
                            KeyCode::KeyA => {
                                let cos = ROT_SPEED.cos();
                                let sin = ROT_SPEED.sin();

                                let old_dir_x = dir_x;
                                dir_x = dir_x * cos - dir_y * sin;
                                dir_y = old_dir_x * sin + dir_y * cos;

                                let old_plane_x = plane_x;
                                plane_x = plane_x * cos - plane_y * sin;
                                plane_y = old_plane_x * sin + plane_y * cos;
                            }
                            KeyCode::KeyD => {
                                let cos = (-ROT_SPEED).cos();
                                let sin = (-ROT_SPEED).sin();

                                let old_dir_x = dir_x;
                                dir_x = dir_x * cos - dir_y * sin;
                                dir_y = old_dir_x * sin + dir_y * cos;

                                let old_plane_x = plane_x;
                                plane_x = plane_x * cos - plane_y * sin;
                                plane_y = old_plane_x * sin + plane_y * cos;
                            }
                            _ => (),
                        }
                    }
                }
                WindowEvent::RedrawRequested => {
                    pixels.frame_mut().fill(0);
                    for x_stripe in 0..SCREEN_WIDTH {
                        let camera_x = 2.0 * x_stripe as f32 / SCREEN_WIDTH as f32 - 1.0;
                        let ray_dir_x = dir_x + plane_x * camera_x;
                        let ray_dir_y = dir_y + plane_y * camera_x;

                        let mut map_x = pos_x as i32;
                        let mut map_y = pos_y as i32;

                        let mut side_dist_x: f32;
                        let mut side_dist_y: f32;

                        let delta_dist_x = if ray_dir_x == 0.0 {
                            1e30
                        } else {
                            (1.0 / ray_dir_x).abs()
                        };
                        let delta_dist_y = if ray_dir_y == 0.0 {
                            1e30
                        } else {
                            (1.0 / ray_dir_y).abs()
                        };
                        let perp_wall_dist: f32;

                        let step_x: i32;
                        let step_y: i32;

                        let mut hit: u32 = 0;
                        let mut side: u32 = 0;

                        if ray_dir_x < 0.0 {
                            step_x = -1;
                            side_dist_x = (pos_x - map_x as f32) * delta_dist_x;
                        } else {
                            step_x = 1;
                            side_dist_x = (map_x as f32 + 1.0 - pos_x) * delta_dist_x;
                        }

                        if ray_dir_y < 0.0 {
                            step_y = -1;
                            side_dist_y = (pos_y - map_y as f32) * delta_dist_y;
                        } else {
                            step_y = 1;
                            side_dist_y = (map_y as f32 + 1.0 - pos_y) * delta_dist_y;
                        }

                        while hit == 0 {
                            if side_dist_x < side_dist_y {
                                side_dist_x += delta_dist_x;
                                map_x += step_x;
                                side = 0;
                            } else {
                                side_dist_y += delta_dist_y;
                                map_y += step_y;
                                side = 1;
                            }

                            if MAP[map_x as usize][map_y as usize] > 0 {
                                hit = 1;
                            }
                        }

                        if side == 0 {
                            perp_wall_dist = side_dist_x - delta_dist_x;
                        } else {
                            perp_wall_dist = side_dist_y - delta_dist_y;
                        }

                        let line_height = (SCREEN_HEIGHT as f32 / perp_wall_dist) as i32;

                        let mut draw_start = -line_height / 2 + SCREEN_HEIGHT as i32 / 2;
                        if draw_start < 0 {
                            draw_start = 0;
                        }
                        let mut draw_end = line_height / 2 + SCREEN_HEIGHT as i32 / 2;
                        if draw_end >= SCREEN_HEIGHT as i32 {
                            draw_end = SCREEN_HEIGHT as i32 - 1;
                        }

                        let mut rgba = [0x5e, 0x48, 0xe8, 0xff];

                        if side == 1 {
                            rgba = [rgba[0] / 2, rgba[1] / 2, rgba[2] / 2, rgba[3]];
                        }

                        draw_vertical_line(
                            x_stripe as usize,
                            draw_start as usize,
                            draw_end as usize,
                            rgba,
                            pixels.frame_mut(),
                            SCREEN_WIDTH as usize,
                        );
                    }

                    if let Err(_err) = pixels.render() {
                        window_target.exit();
                    }
                }
                WindowEvent::Resized(size) => {
                    if let Err(_) = pixels.resize_surface(size.width, size.height) {
                        window_target.exit();
                    }
                }
                _ => (),
            },
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        })
        .unwrap();
}

fn draw_vertical_line(
    x: usize,
    y_start: usize,
    y_end: usize,
    color: [u8; 4],
    frame: &mut [u8],
    width: usize,
) {
    for y in y_start..=y_end {
        let idx = (y * width + x) * 4;
        frame[idx..idx + 4].copy_from_slice(&color);
    }
}
