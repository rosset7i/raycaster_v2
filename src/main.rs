use crate::raycaster::Texture;
use pixels::{Pixels, SurfaceTexture};
use raycaster::Raycaster;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, WindowEvent};
use winit::window::WindowAttributes;
use winit::{
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

mod raycaster;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1024;

fn main() -> Result<(), Box<dyn Error>> {
    let mut pressed_keys: HashSet<KeyCode> = HashSet::new();
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
        screen_width: SCREEN_WIDTH,
        screen_height: SCREEN_HEIGHT,
    };
    raycaster.load_map_from_file(None)?;

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

                raycaster.draw_frame(pixels.frame_mut(), &textures);

                if pixels.render().is_err() {
                    window_target.exit();
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
