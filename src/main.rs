use pixels::{Pixels, SurfaceTexture};
use std::collections::HashMap;
use std::error::Error;
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
    [1, 0, 0, 0, 0, 4, 0, 0, 2, 0, 0, 1],
    [1, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 1],
    [1, 0, 3, 3, 3, 0, 0, 0, 2, 0, 0, 1],
    [1, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

#[derive(Default)]
struct Raycaster {
    pos_x: f32,
    pos_y: f32,

    dir_x: f32,
    dir_y: f32,

    plane_x: f32,
    plane_y: f32,

    move_speed: f32,
    rot_speed: f32,
}

impl Raycaster {
    fn move_up(&mut self) {
        if MAP[(self.pos_x + self.dir_x * self.move_speed) as usize][self.pos_y as usize] == 0 {
            self.pos_x += self.dir_x * self.move_speed;
        }

        if MAP[self.pos_x as usize][(self.pos_y + self.dir_y * self.move_speed) as usize] == 0 {
            self.pos_y += self.dir_y * self.move_speed;
        }
    }

    fn move_down(&mut self) {
        if MAP[(self.pos_x + self.dir_x * self.move_speed) as usize][self.pos_y as usize] == 0 {
            self.pos_x -= self.dir_x * self.move_speed;
        }

        if MAP[self.pos_x as usize][(self.pos_y + self.dir_y * self.move_speed) as usize] == 0 {
            self.pos_y -= self.dir_y * self.move_speed;
        }
    }

    fn turn_left(&mut self) {
        let cos = self.rot_speed.cos();
        let sin = self.rot_speed.sin();

        let old_dir_x = self.dir_x;
        self.dir_x = self.dir_x * cos - self.dir_y * sin;
        self.dir_y = old_dir_x * sin + self.dir_y * cos;

        let old_plane_x = self.plane_x;
        self.plane_x = self.plane_x * cos - self.plane_y * sin;
        self.plane_y = old_plane_x * sin + self.plane_y * cos;
    }

    fn turn_right(&mut self) {
        let cos = (-self.rot_speed).cos();
        let sin = (-self.rot_speed).sin();

        let old_dir_x = self.dir_x;
        self.dir_x = self.dir_x * cos - self.dir_y * sin;
        self.dir_y = old_dir_x * sin + self.dir_y * cos;

        let old_plane_x = self.plane_x;
        self.plane_x = self.plane_x * cos - self.plane_y * sin;
        self.plane_y = old_plane_x * sin + self.plane_y * cos;
    }
}

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
        move_speed: 0.08,
        rot_speed: 0.08,
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
                        KeyCode::KeyW => raycaster.move_up(),
                        KeyCode::KeyS => raycaster.move_down(),
                        KeyCode::KeyA => raycaster.turn_left(),
                        KeyCode::KeyD => raycaster.turn_right(),
                        _ => (),
                    }
                }
            }
            WindowEvent::RedrawRequested => {
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

                        let cell = MAP[map_x as usize][map_y as usize];

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
