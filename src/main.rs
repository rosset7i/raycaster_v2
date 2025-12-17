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
const MAP: [[u8; MAP_HEIGHT]; MAP_WIDTH] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

fn main() {
    // Setup logic
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

    // Raycasting logic
    let pos_x: f32 = 11.0;
    let pos_y: f32 = 6.0;

    let dir_x: f32 = -1.0;
    let dir_y: f32 = 0.0;

    let plane_x: f32 = 0.0;
    let plane_y: f32 = 0.66;

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
                            _ => (),
                        }
                    }
                }
                WindowEvent::RedrawRequested => {
                    for x_stripe in 0..SCREEN_WIDTH {
                        let camera_x = 2.0 * x_stripe as f32 / SCREEN_WIDTH as f32 - 1.0;
                        let ray_dir_x = dir_x + plane_x * camera_x;
                        let ray_dir_y = dir_y + plane_y * camera_x;

                        let map_x = pos_x as u8;
                        let map_y = pos_y as u8;

                        let side_dist_x: f32;
                        let side_dist_y: f32;

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

                        let step_x: u32;
                        let step_y: u32;

                        let hit: u32 = 0;
                        let side: u32;

                        if let Err(_err) = pixels.render() {
                            window_target.exit();
                        }
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
