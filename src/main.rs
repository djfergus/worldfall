mod map;
mod player;
mod enemy;
mod combat;
mod render;
mod input;

use map::Map;
use player::Player;
use enemy::Enemy;
use render::Renderer;
use input::{get_input, wait_for_key, Action};
use combat::{player_attack, enemy_attack};

const MAP_WIDTH: usize = 60;
const MAP_HEIGHT: usize = 20;
const NUM_ROOMS: usize = 6;
const MIN_ROOM_SIZE: usize = 4;
const MAX_ROOM_SIZE: usize = 8;
const ENEMY_CHASE_RANGE: usize = 8;

struct Game {
    map: Map,
    player: Player,
    enemies: Vec<Enemy>,
    renderer: Renderer,
    running: bool,
}

impl Game {
    fn new() -> Self {
        // Generate dungeon
        let mut map = Map::new(MAP_WIDTH, MAP_HEIGHT);
        map.generate(NUM_ROOMS, MIN_ROOM_SIZE, MAX_ROOM_SIZE);

        // Spawn player in first room
        let (px, py) = map.player_spawn();
        let player = Player::new(px, py);

        // Spawn enemies in other rooms
        let spawn_points = map.enemy_spawn_points();
        let enemies: Vec<Enemy> = spawn_points
            .into_iter()
            .map(|(x, y)| Enemy::goblin(x, y))
            .collect();

        let renderer = Renderer::new();

        Game {
            map,
            player,
            enemies,
            renderer,
            running: true,
        }
    }

    fn run(&mut self) -> std::io::Result<()> {
        self.renderer.init()?;

        while self.running {
            self.renderer.render(&self.map, &self.player, &self.enemies)?;

            let action = get_input();

            match action {
                Action::Quit => {
                    self.running = false;
                }
                Action::Move(dx, dy) => {
                    self.handle_player_move(dx, dy);

                    if self.player.is_alive() {
                        self.enemy_turns();
                    }

                    self.check_game_state()?;
                }
                Action::None => {}
            }
        }

        self.renderer.cleanup()?;
        Ok(())
    }

    fn handle_player_move(&mut self, dx: i32, dy: i32) {
        let new_x = (self.player.x as i32 + dx) as usize;
        let new_y = (self.player.y as i32 + dy) as usize;

        // Check for enemy at target position
        if let Some(enemy_idx) = self.enemy_at(new_x, new_y) {
            let result = player_attack(&self.player, &mut self.enemies[enemy_idx]);
            self.renderer.add_message(result.message);
        } else if self.map.is_walkable(new_x, new_y) {
            self.player.move_by(dx, dy);
        }
    }

    fn enemy_turns(&mut self) {
        let player_x = self.player.x;
        let player_y = self.player.y;

        for i in 0..self.enemies.len() {
            if !self.enemies[i].is_alive() {
                continue;
            }

            let distance = self.enemies[i].distance_to(player_x, player_y);

            if distance == 1 {
                // Adjacent to player - attack
                let result = enemy_attack(&self.enemies[i], &mut self.player);
                self.renderer.add_message(result.message);
            } else if distance <= ENEMY_CHASE_RANGE {
                // Within chase range - move toward player
                // Create a snapshot of current positions for collision checking
                let enemies_snapshot: Vec<Enemy> = self.enemies.clone();
                self.enemies[i].move_toward(player_x, player_y, &self.map, &enemies_snapshot, i, player_x, player_y);
            }
        }
    }

    fn enemy_at(&self, x: usize, y: usize) -> Option<usize> {
        self.enemies.iter().position(|e| e.is_alive() && e.x == x && e.y == y)
    }

    fn check_game_state(&mut self) -> std::io::Result<()> {
        if !self.player.is_alive() {
            self.renderer.render_game_over()?;
            wait_for_key();
            self.running = false;
        } else if self.all_enemies_dead() {
            self.renderer.render_victory()?;
            wait_for_key();
            self.running = false;
        }
        Ok(())
    }

    fn all_enemies_dead(&self) -> bool {
        self.enemies.iter().all(|e| !e.is_alive())
    }
}

fn main() {
    let mut game = Game::new();

    if let Err(e) = game.run() {
        // Make sure we clean up even on error
        let _ = game.renderer.cleanup();
        eprintln!("Error: {}", e);
    }
}
