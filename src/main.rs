use pixels::{Error, Pixels, SurfaceTexture};
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

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new().map_err(|e| Error::UserDefined(Box::from(e)))?;

    let window = {
        let size = LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT);

        #[allow(deprecated)]
        event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("Raycaster")
                    .with_inner_size(size),
            )
            .map_err(|e| Error::UserDefined(Box::from(e)))?
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
                for x_stripe in 0..SCREEN_WIDTH {
                    let camera_x = 2.0 * x_stripe as f32 / SCREEN_WIDTH as f32 - 1.0;
                    let ray_dir_x = raycaster.dir_x + raycaster.plane_x * camera_x;
                    let ray_dir_y = raycaster.dir_y + raycaster.plane_y * camera_x;

                    let mut map_x = raycaster.pos_x as i32;
                    let mut map_y = raycaster.pos_y as i32;

                    let delta_dist_x = (1.0 / ray_dir_x).abs();
                    let delta_dist_y = (1.0 / ray_dir_y).abs();

                    let (mut side_dist_x, step_x) = if ray_dir_x < 0.0 {
                        ((raycaster.pos_x - map_x as f32) * delta_dist_x, -1)
                    } else {
                        ((map_x as f32 + 1.0 - raycaster.pos_x) * delta_dist_x, 1)
                    };

                    let (mut side_dist_y, step_y) = if ray_dir_y < 0.0 {
                        ((raycaster.pos_y - map_y as f32) * delta_dist_y, -1)
                    } else {
                        ((map_y as f32 + 1.0 - raycaster.pos_y) * delta_dist_y, 1)
                    };

                    let mut side: u32;
                    loop {
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
                            break;
                        }
                    }

                    let perp_wall_dist = if side == 0 {
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
