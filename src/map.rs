use rand::Rng;

// Wall characters indexed by 4-bit mask: UP(1) + DOWN(2) + LEFT(4) + RIGHT(8)
const WALL_CHARS: [char; 16] = [
    '#', '│', '│', '│', '─', '┘', '┐', '┤',
    '─', '└', '┌', '├', '─', '┴', '┬', '┼',
];

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
    Corridor,
    Door,
    Potion,
}

impl Tile {
    pub fn to_char(&self) -> char {
        match self {
            Tile::Wall => '#',
            Tile::Floor => '·',
            Tile::Corridor => ':',
            Tile::Door => '╬',
            Tile::Potion => '♥',
        }
    }

    pub fn is_walkable(&self) -> bool {
        matches!(self, Tile::Floor | Tile::Corridor | Tile::Door | Tile::Potion)
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
    pub revealed: Vec<Vec<bool>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![Tile::Wall; width]; height];
        let revealed = vec![vec![false; width]; height];
        Map {
            width,
            height,
            tiles,
            rooms: Vec::new(),
            revealed,
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

        self.place_doors();
        self.place_potions();
    }

    /// Place health potions randomly in rooms
    fn place_potions(&mut self) {
        let mut rng = rand::thread_rng();

        for room in &self.rooms.clone() {
            // 50% chance to spawn a potion in each room
            if rng.gen_bool(0.5) {
                // Pick a random floor tile in the room (not center to avoid player/enemy spawn)
                let x = rng.gen_range(room.x..room.x + room.width);
                let y = rng.gen_range(room.y..room.y + room.height);
                let (cx, cy) = room.center();

                // Don't place on room center (spawn point)
                if (x, y) != (cx, cy) && self.tiles[y][x] == Tile::Floor {
                    self.tiles[y][x] = Tile::Potion;
                }
            }
        }
    }

    /// Place doors at room entrances (in the wall where corridors meet rooms)
    /// Only place doors on straight corridor sections, not at turns/corners
    fn place_doors(&mut self) {
        for room in &self.rooms.clone() {
            // Check each edge of the room for corridor connections
            // Place door in the corridor tile (the wall position), not inside the room
            // Only place if it's a straight section (walls on both perpendicular sides)

            // Top wall
            for x in room.x..room.x + room.width {
                if room.y > 0 && self.tiles[room.y - 1][x] == Tile::Corridor {
                    // Check that left and right are walls (not corridor turning here)
                    let left_is_wall = x == 0 || self.tiles[room.y - 1][x - 1] == Tile::Wall;
                    let right_is_wall = x + 1 >= self.width || self.tiles[room.y - 1][x + 1] == Tile::Wall;
                    if left_is_wall && right_is_wall {
                        self.tiles[room.y - 1][x] = Tile::Door;
                    }
                }
            }
            // Bottom wall
            let below_room = room.y + room.height;
            for x in room.x..room.x + room.width {
                if below_room < self.height && self.tiles[below_room][x] == Tile::Corridor {
                    let left_is_wall = x == 0 || self.tiles[below_room][x - 1] == Tile::Wall;
                    let right_is_wall = x + 1 >= self.width || self.tiles[below_room][x + 1] == Tile::Wall;
                    if left_is_wall && right_is_wall {
                        self.tiles[below_room][x] = Tile::Door;
                    }
                }
            }
            // Left wall
            for y in room.y..room.y + room.height {
                if room.x > 0 && self.tiles[y][room.x - 1] == Tile::Corridor {
                    // Check that up and down are walls
                    let up_is_wall = y == 0 || self.tiles[y - 1][room.x - 1] == Tile::Wall;
                    let down_is_wall = y + 1 >= self.height || self.tiles[y + 1][room.x - 1] == Tile::Wall;
                    if up_is_wall && down_is_wall {
                        self.tiles[y][room.x - 1] = Tile::Door;
                    }
                }
            }
            // Right wall
            let right_of_room = room.x + room.width;
            for y in room.y..room.y + room.height {
                if right_of_room < self.width && self.tiles[y][right_of_room] == Tile::Corridor {
                    let up_is_wall = y == 0 || self.tiles[y - 1][right_of_room] == Tile::Wall;
                    let down_is_wall = y + 1 >= self.height || self.tiles[y + 1][right_of_room] == Tile::Wall;
                    if up_is_wall && down_is_wall {
                        self.tiles[y][right_of_room] = Tile::Door;
                    }
                }
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
            if y < self.height && x < self.width && self.tiles[y][x] == Tile::Wall {
                self.tiles[y][x] = Tile::Corridor;
            }
        }
    }

    fn carve_vertical_corridor(&mut self, y1: usize, y2: usize, x: usize) {
        let start = y1.min(y2);
        let end = y1.max(y2);
        for y in start..=end {
            if y < self.height && x < self.width && self.tiles[y][x] == Tile::Wall {
                self.tiles[y][x] = Tile::Corridor;
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

    /// Check if position is a border wall (wall adjacent to non-wall)
    fn is_border_wall_at(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return false;
        }
        let ux = x as usize;
        let uy = y as usize;
        if self.tiles[uy][ux] != Tile::Wall {
            return false;
        }
        self.is_border_wall(ux, uy)
    }

    /// Compute the box-drawing character for a wall based on neighboring border walls
    fn compute_wall_char(&self, x: usize, y: usize) -> char {
        let ix = x as i32;
        let iy = y as i32;

        let mut mask: usize = 0;
        if self.is_border_wall_at(ix, iy - 1) { mask |= 1; } // UP
        if self.is_border_wall_at(ix, iy + 1) { mask |= 2; } // DOWN
        if self.is_border_wall_at(ix - 1, iy) { mask |= 4; } // LEFT
        if self.is_border_wall_at(ix + 1, iy) { mask |= 8; } // RIGHT

        WALL_CHARS[mask]
    }

    /// Get the display character for a tile, handling wall connections and fog of war
    pub fn get_tile_char(&self, x: usize, y: usize) -> char {
        if !self.is_revealed(x, y) {
            return ' ';
        }
        match self.get_tile(x, y) {
            Some(Tile::Wall) => {
                // Only render walls that border non-wall tiles
                if self.is_border_wall(x, y) {
                    self.compute_wall_char(x, y)
                } else {
                    ' '
                }
            }
            Some(tile) => tile.to_char(),
            None => ' ',
        }
    }

    /// Check if a wall is adjacent to any non-wall tile (should be rendered)
    /// Checks all 8 directions including diagonals for corners
    fn is_border_wall(&self, x: usize, y: usize) -> bool {
        let ix = x as i32;
        let iy = y as i32;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = ix + dx;
                let ny = iy + dy;
                if nx >= 0 && ny >= 0 && (nx as usize) < self.width && (ny as usize) < self.height {
                    let tile = &self.tiles[ny as usize][nx as usize];
                    if *tile != Tile::Wall {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Check if a tile has been revealed
    pub fn is_revealed(&self, x: usize, y: usize) -> bool {
        self.revealed.get(y).and_then(|row| row.get(x)).copied().unwrap_or(false)
    }

    /// Reveal a single tile
    pub fn reveal_at(&mut self, x: usize, y: usize) {
        if y < self.height && x < self.width {
            self.revealed[y][x] = true;
        }
    }

    /// Find which room contains the given position (returns room index)
    pub fn room_at(&self, x: usize, y: usize) -> Option<usize> {
        self.rooms.iter().position(|room| {
            x >= room.x && x < room.x + room.width &&
            y >= room.y && y < room.y + room.height
        })
    }

    /// Reveal an entire room including its surrounding walls
    pub fn reveal_room(&mut self, room_idx: usize) {
        if room_idx >= self.rooms.len() {
            return;
        }
        let room = self.rooms[room_idx].clone();

        // Reveal the room interior
        for y in room.y..room.y + room.height {
            for x in room.x..room.x + room.width {
                self.revealed[y][x] = true;
            }
        }

        // Reveal surrounding walls (1 tile border)
        let start_x = room.x.saturating_sub(1);
        let end_x = (room.x + room.width + 1).min(self.width);
        let start_y = room.y.saturating_sub(1);
        let end_y = (room.y + room.height + 1).min(self.height);

        // Top and bottom walls
        for x in start_x..end_x {
            if start_y < room.y {
                self.revealed[start_y][x] = true;
            }
            if end_y > room.y + room.height && end_y <= self.height {
                self.revealed[end_y - 1][x] = true;
            }
        }
        // Left and right walls
        for y in start_y..end_y {
            if start_x < room.x {
                self.revealed[y][start_x] = true;
            }
            if end_x > room.x + room.width && end_x <= self.width {
                self.revealed[y][end_x - 1] = true;
            }
        }
    }

    /// Check if position is a door
    pub fn is_door(&self, x: usize, y: usize) -> bool {
        self.get_tile(x, y).map_or(false, |t| *t == Tile::Door)
    }

    /// Check if position is a corridor
    pub fn is_corridor(&self, x: usize, y: usize) -> bool {
        self.get_tile(x, y).map_or(false, |t| *t == Tile::Corridor)
    }

    /// Reveal surrounding tiles (for corridor visibility)
    pub fn reveal_surroundings(&mut self, x: usize, y: usize) {
        let ix = x as i32;
        let iy = y as i32;
        for dy in -1..=1 {
            for dx in -1..=1 {
                let nx = ix + dx;
                let ny = iy + dy;
                if nx >= 0 && ny >= 0 && (nx as usize) < self.width && (ny as usize) < self.height {
                    self.revealed[ny as usize][nx as usize] = true;
                }
            }
        }
    }

    /// Check if position has a potion
    pub fn is_potion(&self, x: usize, y: usize) -> bool {
        self.get_tile(x, y).map_or(false, |t| *t == Tile::Potion)
    }

    /// Pick up potion at position (converts to floor)
    pub fn pickup_potion(&mut self, x: usize, y: usize) {
        if y < self.height && x < self.width && self.tiles[y][x] == Tile::Potion {
            self.tiles[y][x] = Tile::Floor;
        }
    }
}
