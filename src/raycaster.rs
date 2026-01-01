/// The rotation speed multiplier, use this to set the sensitivity of the camera
const ROT_SPEED: f32 = 0.08;

const MAP: [[u8; 12]; 13] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
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

pub struct Raycaster {
    /// The 2D array representing the walls of the map, any value other than 0 is equivalent to a wall
    pub map: Vec<Vec<u8>>,

    /// Screen properties
    pub screen: Screen,

    /// Player properties
    pub player: Player,
}

pub struct Screen {
    /// The height of the screen, in pixels
    pub height: u32,

    /// The width of the screen, in pixels
    pub width: u32,
}

// TODO: Use struct to validade the array bounds
pub struct Player {
    /// Player position in the 2D array, that being, any value greater than the array bounds will result on the player being out of the map
    pub position: Position,

    /// The direction vector, this is mutated then the player rotates its position
    pub direction: Position,

    /// The direction vector, this is mutated then the player rotates its position
    pub plane: Position,

    /// The move speed multiplier, use this to set the velocity of the player
    pub move_speed: f32,
}

impl Player {
    /// Moves the player up in the X and Y axys if no colision is detected
    pub fn move_up(&mut self, map: &Vec<Vec<u8>>) {
        if map[(self.position.x + self.direction.x * self.move_speed) as usize]
            [self.position.y as usize]
            == 0
        {
            self.position.x += self.direction.x * self.move_speed;
        }

        if map[self.position.x as usize]
            [(self.position.y + self.direction.y * self.move_speed) as usize]
            == 0
        {
            self.position.y += self.direction.y * self.move_speed;
        }
    }

    /// Moves the player down in the X and Y axys if no colision is detected
    pub fn move_down(&mut self, map: &Vec<Vec<u8>>) {
        if map[(self.position.x + self.direction.x * self.move_speed) as usize]
            [self.position.y as usize]
            == 0
        {
            self.position.x -= self.direction.x * self.move_speed;
        }

        if map[self.position.x as usize]
            [(self.position.y + self.direction.y * self.move_speed) as usize]
            == 0
        {
            self.position.y -= self.direction.y * self.move_speed;
        }
    }

    pub fn rotate(&mut self, left: bool) {
        let cos = if left { ROT_SPEED } else { -ROT_SPEED }.cos();
        let sin = if left { ROT_SPEED } else { -ROT_SPEED }.sin();

        let old_dir_x = self.direction.x;
        self.direction.x = self.direction.x * cos - self.direction.y * sin;
        self.direction.y = old_dir_x * sin + self.direction.y * cos;

        let old_plane_x = self.plane.x;
        self.plane.x = self.plane.x * cos - self.plane.y * sin;
        self.plane.y = old_plane_x * sin + self.plane.y * cos;
    }
}
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Raycaster {
    pub fn new() -> Raycaster {
        Raycaster {
            player: Player {
                plane: Position { x: 0.0, y: 0.66 },
                position: Position { x: 1.0, y: 1.0 },
                direction: Position { x: -1.0, y: 0.0 },
                move_speed: 0.08,
            },
            map: MAP.iter().map(|col| col.to_vec()).collect(),
            screen: Screen {
                height: 480,
                width: 640,
            },
        }
    }
}
