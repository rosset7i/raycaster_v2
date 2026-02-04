use std::{
    fs,
    io::{Error, ErrorKind},
    num::ParseIntError,
};

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
}

impl Raycaster {
    pub fn load_map(&mut self, map: Vec<Vec<u8>>) {
        self.map = map;
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
