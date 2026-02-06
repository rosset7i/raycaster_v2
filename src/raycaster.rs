use std::{
    collections::HashMap,
    fs,
    io::{Error, ErrorKind},
    num::ParseIntError,
};

pub enum Texture {
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

const FALLBACK_TEXTURE: Texture = Texture::Color([255, 0, 255, 255], [128, 0, 128, 255]); // debug pink

#[derive(Default)]
pub struct Raycaster {
    pub pos_x: f32,
    pub pos_y: f32,

    pub dir_x: f32,
    pub dir_y: f32,

    pub plane_x: f32,
    pub plane_y: f32,

    pub move_speed: f32,
    pub rot_speed: f32,

    pub map: Vec<Vec<u8>>,

    pub screen_width: u32,
    pub screen_height: u32,
}

impl Raycaster {
    pub fn draw_frame(&mut self, frame: &mut [u8], textures: &HashMap<u8, Texture>) {
        frame.fill(0);
        for vertical_stripe in 0..self.screen_width {
            let camera_x = 2.0 * vertical_stripe as f32 / self.screen_width as f32 - 1.0;
            let ray_dir_x = self.dir_x + self.plane_x * camera_x;
            let ray_dir_y = self.dir_y + self.plane_y * camera_x;

            let delta_dist_x = (1.0 / ray_dir_x).abs();
            let delta_dist_y = (1.0 / ray_dir_y).abs();

            let (mut side_dist_x, step_x) = if ray_dir_x < 0.0 {
                (self.pos_x.fract() * delta_dist_x, -1)
            } else {
                ((1.0 - self.pos_x.fract()) * delta_dist_x, 1)
            };

            let (mut side_dist_y, step_y) = if ray_dir_y < 0.0 {
                (self.pos_y.fract() * delta_dist_y, -1)
            } else {
                ((1.0 - self.pos_y.fract()) * delta_dist_y, 1)
            };

            let mut side: bool;
            let mut map_x = self.pos_x as i32;
            let mut map_y = self.pos_y as i32;
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

                let cell = self.map[map_x as usize][map_y as usize];

                if cell > 0 {
                    texture = textures.get(&cell).unwrap_or(&FALLBACK_TEXTURE);
                    break;
                }
            }

            let perp_wall_dist = if !side {
                side_dist_x - delta_dist_x
            } else {
                side_dist_y - delta_dist_y
            };

            let line_height = (self.screen_height as f32 / perp_wall_dist) as i32;

            let mut draw_start = -line_height / 2 + self.screen_height as i32 / 2;
            if draw_start < 0 {
                draw_start = 0;
            }

            let mut draw_end = line_height / 2 + self.screen_height as i32 / 2;
            if draw_end >= self.screen_height as i32 {
                draw_end = self.screen_height as i32 - 1;
            }

            draw_vertical_line(
                vertical_stripe as usize,
                draw_start as usize,
                draw_end as usize,
                *texture.color(side),
                frame,
                self.screen_width as usize,
            );
        }
    }

    pub fn load_map_from_file(&mut self, path: Option<&str>) -> Result<(), Error> {
        let read_from = path.unwrap_or("map.txt");
        let map_from_file: Vec<Vec<u8>> = fs::read_to_string(read_from)?
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|char| char.trim().parse::<u8>())
                    .collect::<Result<Vec<u8>, ParseIntError>>()
            })
            .collect::<Result<Vec<Vec<u8>>, ParseIntError>>()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        self.map = map_from_file;
        Ok(())
    }

    pub fn move_up(&mut self) {
        if self.map[(self.pos_x + self.dir_x * self.move_speed) as usize][self.pos_y as usize] == 0
        {
            self.pos_x += self.dir_x * self.move_speed;
        }

        if self.map[self.pos_x as usize][(self.pos_y + self.dir_y * self.move_speed) as usize] == 0
        {
            self.pos_y += self.dir_y * self.move_speed;
        }
    }

    pub fn move_down(&mut self) {
        if self.map[(self.pos_x + self.dir_x * self.move_speed) as usize][self.pos_y as usize] == 0
        {
            self.pos_x -= self.dir_x * self.move_speed;
        }

        if self.map[self.pos_x as usize][(self.pos_y + self.dir_y * self.move_speed) as usize] == 0
        {
            self.pos_y -= self.dir_y * self.move_speed;
        }
    }

    pub fn turn_left(&mut self) {
        let cos = self.rot_speed.cos();
        let sin = self.rot_speed.sin();

        let old_dir_x = self.dir_x;
        self.dir_x = self.dir_x * cos - self.dir_y * sin;
        self.dir_y = old_dir_x * sin + self.dir_y * cos;

        let old_plane_x = self.plane_x;
        self.plane_x = self.plane_x * cos - self.plane_y * sin;
        self.plane_y = old_plane_x * sin + self.plane_y * cos;
    }

    pub fn turn_right(&mut self) {
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
