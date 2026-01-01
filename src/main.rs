use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, WindowEvent};
use winit::window::WindowAttributes;
use winit::{
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::raycaster::Raycaster;

mod raycaster;

fn main() -> Result<(), Error> {
    let mut raycaster = Raycaster::new();

    let event_loop = EventLoop::new().map_err(|e| Error::UserDefined(Box::from(e)))?;
    let window = {
        let size = LogicalSize::new(raycaster.screen.width, raycaster.screen.height);

        #[allow(deprecated)]
        event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("Raycaster")
                    .with_min_inner_size(size),
            )
            .map_err(|e| Error::UserDefined(Box::from(e)))?
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(
            raycaster.screen.width,
            raycaster.screen.height,
            surface_texture,
        )?
    };

    #[allow(deprecated)]
    let res = event_loop.run(|event, window_target| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => window_target.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key) = event.physical_key
                    && event.state == ElementState::Pressed
                {
                    match key {
                        KeyCode::Escape => window_target.exit(),
                        KeyCode::KeyW => raycaster.player.move_up(&raycaster.map),
                        KeyCode::KeyS => raycaster.player.move_down(&raycaster.map),
                        KeyCode::KeyA => raycaster.player.rotate(true),
                        KeyCode::KeyD => raycaster.player.rotate(false),
                        _ => (),
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                pixels.frame_mut().fill(0);
                for x_stripe in 0..raycaster.screen.width {
                    let camera_x = 2.0 * x_stripe as f32 / raycaster.screen.width as f32 - 1.0;
                    let ray_dir_x =
                        raycaster.player.direction.x + raycaster.player.plane.x * camera_x;
                    let ray_dir_y =
                        raycaster.player.direction.y + raycaster.player.plane.y * camera_x;

                    let mut map_x = raycaster.player.position.x as i32;
                    let mut map_y = raycaster.player.position.y as i32;

                    let mut side_dist_x: f32;
                    let mut side_dist_y: f32;

                    // Possible division by 0
                    let delta_dist_x = (1.0 / ray_dir_x).abs();
                    let delta_dist_y = (1.0 / ray_dir_y).abs();

                    let perp_wall_dist: f32;

                    let step_x: i32;
                    let step_y: i32;

                    let mut hit: u32 = 0;
                    let mut side: u32 = 0;

                    side_dist_x = if ray_dir_x < 0.0 {
                        step_x = -1;
                        (raycaster.player.position.x - map_x as f32) * delta_dist_x
                    } else {
                        step_x = 1;
                        (map_x as f32 + 1.0 - raycaster.player.position.x) * delta_dist_x
                    };

                    side_dist_y = if ray_dir_y < 0.0 {
                        step_y = -1;
                        (raycaster.player.position.y - map_y as f32) * delta_dist_y
                    } else {
                        step_y = 1;
                        (map_y as f32 + 1.0 - raycaster.player.position.y) * delta_dist_y
                    };

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

                        if raycaster.map[map_x as usize][map_y as usize] > 0 {
                            hit = 1;
                        }
                    }

                    if side == 0 {
                        perp_wall_dist = side_dist_x - delta_dist_x;
                    } else {
                        perp_wall_dist = side_dist_y - delta_dist_y;
                    }

                    let line_height = (raycaster.screen.height as f32 / perp_wall_dist) as i32;

                    let mut draw_start = -line_height / 2 + raycaster.screen.height as i32 / 2;
                    if draw_start < 0 {
                        draw_start = 0;
                    }
                    let mut draw_end = line_height / 2 + raycaster.screen.height as i32 / 2;
                    if draw_end >= raycaster.screen.height as i32 {
                        draw_end = raycaster.screen.height as i32 - 1;
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
                        raycaster.screen.width as usize,
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
    });
    res.map_err(|e| Error::UserDefined(Box::new(e)))
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
