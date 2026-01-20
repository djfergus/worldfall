use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
    Door,
}

impl Tile {
    pub fn to_char(&self) -> char {
        match self {
            Tile::Wall => '#',
            Tile::Floor => '.',
            Tile::Door => '+',
        }
    }

    pub fn is_walkable(&self) -> bool {
        matches!(self, Tile::Floor | Tile::Door)
    }
}

#[derive(Clone)]
pub struct Room {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Room {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Room { x, y, width, height }
    }

    pub fn center(&self) -> (usize, usize) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }

    pub fn intersects(&self, other: &Room) -> bool {
        self.x < other.x + other.width + 1
            && self.x + self.width + 1 > other.x
            && self.y < other.y + other.height + 1
            && self.y + self.height + 1 > other.y
    }
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
    pub rooms: Vec<Room>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![Tile::Wall; width]; height];
        Map {
            width,
            height,
            tiles,
            rooms: Vec::new(),
        }
    }

    pub fn generate(&mut self, num_rooms: usize, min_room_size: usize, max_room_size: usize) {
        let mut rng = rand::thread_rng();

        for _ in 0..num_rooms * 10 {
            if self.rooms.len() >= num_rooms {
                break;
            }

            let room_width = rng.gen_range(min_room_size..=max_room_size);
            let room_height = rng.gen_range(min_room_size..=max_room_size);
            let x = rng.gen_range(1..self.width - room_width - 1);
            let y = rng.gen_range(1..self.height - room_height - 1);

            let new_room = Room::new(x, y, room_width, room_height);

            let mut overlaps = false;
            for room in &self.rooms {
                if new_room.intersects(room) {
                    overlaps = true;
                    break;
                }
            }

            if !overlaps {
                self.carve_room(&new_room);

                if !self.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = self.rooms.last().unwrap().center();

                    if rng.gen_bool(0.5) {
                        self.carve_horizontal_corridor(prev_x, new_x, prev_y);
                        self.carve_vertical_corridor(prev_y, new_y, new_x);
                    } else {
                        self.carve_vertical_corridor(prev_y, new_y, prev_x);
                        self.carve_horizontal_corridor(prev_x, new_x, new_y);
                    }
                }

                self.rooms.push(new_room);
            }
        }
    }

    fn carve_room(&mut self, room: &Room) {
        for y in room.y..room.y + room.height {
            for x in room.x..room.x + room.width {
                self.tiles[y][x] = Tile::Floor;
            }
        }
    }

    fn carve_horizontal_corridor(&mut self, x1: usize, x2: usize, y: usize) {
        let start = x1.min(x2);
        let end = x1.max(x2);
        for x in start..=end {
            if y < self.height && x < self.width {
                self.tiles[y][x] = Tile::Floor;
            }
        }
    }

    fn carve_vertical_corridor(&mut self, y1: usize, y2: usize, x: usize) {
        let start = y1.min(y2);
        let end = y1.max(y2);
        for y in start..=end {
            if y < self.height && x < self.width {
                self.tiles[y][x] = Tile::Floor;
            }
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(y).and_then(|row| row.get(x))
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        self.get_tile(x, y).map_or(false, |t| t.is_walkable())
    }

    pub fn player_spawn(&self) -> (usize, usize) {
        if let Some(room) = self.rooms.first() {
            room.center()
        } else {
            (self.width / 2, self.height / 2)
        }
    }

    pub fn enemy_spawn_points(&self) -> Vec<(usize, usize)> {
        self.rooms.iter().skip(1).map(|r| r.center()).collect()
    }
}
