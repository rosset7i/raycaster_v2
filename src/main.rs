use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::KeyCode;
use winit::window::WindowAttributes;

struct World {
    map: Vec<Vec<u8>>,
    cols: usize,
    rows: usize,
    tile_size: f32,
    player: Player,
    max_depth: f32,
}

impl World {
    fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.map.get(y)?.get(x).copied()
    }

    fn is_wall(&self, x: usize, y: usize) -> bool {
        self.get(x, y).is_some_and(|v| v != 0)
    }

    fn to_tile(&self, px: f32, py: f32) -> (usize, usize) {
        (
            (px / self.tile_size) as usize,
            (py / self.tile_size) as usize,
        )
    }

    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let px = (i % WIDTH as usize) as f32;
            let py = (i / WIDTH as usize) as f32;

            let (x, y) = self.to_tile(px, py);
            let rgba = if self.is_wall(x, y) {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}

struct Player {
    x: f32,
    y: f32,
    dir: f32,
    fov: f32,
    move_speed: f32,
    rot_speed: f32,
}

impl Player {
    fn to_tile(&self, px: f32, py: f32) -> (usize, usize) {
        (
            (px / self.tile_size) as usize,
            (py / self.tile_size) as usize,
        )
    }

    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let px = (i % WIDTH as usize) as f32;
            let py = (i / WIDTH as usize) as f32;

            let (x, y) = self.to_tile(px, py);
            let rgba = if self.is_wall(x, y) {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;

fn main() -> Result<(), Error> {
    let map = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 1, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 1, 0, 1, 2, 1, 1, 1],
        vec![1, 0, 0, 1, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 3, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 1, 0, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 1, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 1, 0, 0, 0, 0, 0, 1],
        vec![1, 1, 4, 1, 1, 1, 1, 1, 1, 1],
    ];

    let world = World {
        cols: map[0].len(),
        rows: map.len(),
        map: map,
        tile_size: 60.0,
        player: Player {
            x: 1.0,
            y: 1.0,
            dir: 1.0,
            fov: 1.0,
            move_speed: 1.0,
            rot_speed: 1.0,
        },
        max_depth: 100.0,
    };

    let event_loop = EventLoop::new().map_err(|e| Error::UserDefined(Box::new(e)))?;
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);

        #[allow(deprecated)]
        event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("Raycaster")
                    .with_inner_size(size)
                    .with_min_inner_size(size),
            )
            .map_err(|e| Error::UserDefined(Box::new(e)))?
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    #[allow(deprecated)]
    let res = event_loop.run(|event, window_target| match event {
        Event::WindowEvent { event, .. } => {
            handle_window_events(event, &world, &mut pixels, window_target)
        }
        Event::AboutToWait => {
            window.request_redraw();
        }
        _ => (),
    });
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}

fn handle_window_events(
    event: WindowEvent,
    world: &World,
    pixels: &mut Pixels,
    window_target: &ActiveEventLoop,
) {
    match event {
        WindowEvent::CloseRequested => window_target.exit(),
        WindowEvent::KeyboardInput { event, .. } => {
            if event.physical_key == KeyCode::Escape && event.state == ElementState::Pressed {
                window_target.exit();
            }
        }
        WindowEvent::RedrawRequested => {
            world.draw(pixels.frame_mut());
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
    }
}
