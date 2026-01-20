use crate::map::Map;

#[derive(Clone, Copy, PartialEq)]
pub enum EnemyType {
    Goblin,
}

impl EnemyType {
    pub fn to_char(&self) -> char {
        match self {
            EnemyType::Goblin => 'g',
        }
    }

    pub fn base_hp(&self) -> i32 {
        match self {
            EnemyType::Goblin => 6,
        }
    }

    pub fn base_power(&self) -> i32 {
        match self {
            EnemyType::Goblin => 3,
        }
    }
}

#[derive(Clone)]
pub struct Enemy {
    pub x: usize,
    pub y: usize,
    pub hp: i32,
    pub max_hp: i32,
    pub power: i32,
    pub enemy_type: EnemyType,
}

impl Enemy {
    pub fn new(x: usize, y: usize, enemy_type: EnemyType) -> Self {
        let hp = enemy_type.base_hp();
        let power = enemy_type.base_power();
        Enemy {
            x,
            y,
            hp,
            max_hp: hp,
            power,
            enemy_type,
        }
    }

    pub fn goblin(x: usize, y: usize) -> Self {
        Enemy::new(x, y, EnemyType::Goblin)
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
        if self.hp < 0 {
            self.hp = 0;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn to_char(&self) -> char {
        self.enemy_type.to_char()
    }

    pub fn move_toward(&mut self, target_x: usize, target_y: usize, map: &Map, enemies: &[Enemy], self_index: usize, player_x: usize, player_y: usize) {
        let dx = (target_x as i32 - self.x as i32).signum();
        let dy = (target_y as i32 - self.y as i32).signum();

        let new_x = (self.x as i32 + dx) as usize;
        let new_y = (self.y as i32 + dy) as usize;

        // Check if we can move to the new position (not occupied by enemies or player)
        if map.is_walkable(new_x, new_y) && !Self::position_occupied(new_x, new_y, enemies, self_index, player_x, player_y) {
            self.x = new_x;
            self.y = new_y;
        } else if dx != 0 && map.is_walkable((self.x as i32 + dx) as usize, self.y)
            && !Self::position_occupied((self.x as i32 + dx) as usize, self.y, enemies, self_index, player_x, player_y) {
            // Try horizontal only
            self.x = (self.x as i32 + dx) as usize;
        } else if dy != 0 && map.is_walkable(self.x, (self.y as i32 + dy) as usize)
            && !Self::position_occupied(self.x, (self.y as i32 + dy) as usize, enemies, self_index, player_x, player_y) {
            // Try vertical only
            self.y = (self.y as i32 + dy) as usize;
        }
    }

    fn position_occupied(x: usize, y: usize, enemies: &[Enemy], exclude_index: usize, player_x: usize, player_y: usize) -> bool {
        if x == player_x && y == player_y {
            return true;
        }
        enemies.iter().enumerate().any(|(i, e)| {
            i != exclude_index && e.is_alive() && e.x == x && e.y == y
        })
    }

    pub fn distance_to(&self, x: usize, y: usize) -> usize {
        let dx = (self.x as i32 - x as i32).unsigned_abs() as usize;
        let dy = (self.y as i32 - y as i32).unsigned_abs() as usize;
        dx + dy
    }
}
