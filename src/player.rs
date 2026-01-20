pub struct Player {
    pub x: usize,
    pub y: usize,
    pub hp: i32,
    pub max_hp: i32,
    pub power: i32,
}

impl Player {
    pub fn new(x: usize, y: usize) -> Self {
        Player {
            x,
            y,
            hp: 20,
            max_hp: 20,
            power: 5,
        }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x = (self.x as i32 + dx) as usize;
        self.y = (self.y as i32 + dy) as usize;
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

    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }

    pub fn to_char(&self) -> char {
        '@'
    }
}
