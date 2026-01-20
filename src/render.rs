use std::io::{self, Write};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::Print,
    terminal::{self, Clear, ClearType},
};

use crate::map::Map;
use crate::player::Player;
use crate::enemy::Enemy;

pub struct Renderer {
    messages: Vec<String>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            messages: Vec::new(),
        }
    }

    pub fn init(&self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), Hide, Clear(ClearType::All))?;
        Ok(())
    }

    pub fn cleanup(&self) -> io::Result<()> {
        terminal::disable_raw_mode()?;
        execute!(io::stdout(), Show, Clear(ClearType::All))?;
        Ok(())
    }

    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
        if self.messages.len() > 5 {
            self.messages.remove(0);
        }
    }

    pub fn render(&self, map: &Map, player: &Player, enemies: &[Enemy]) -> io::Result<()> {
        let mut stdout = io::stdout();

        execute!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;

        // Render map
        for y in 0..map.height {
            execute!(stdout, MoveTo(0, y as u16))?;
            for x in 0..map.width {
                let ch = self.get_char_at(x, y, map, player, enemies);
                execute!(stdout, Print(ch))?;
            }
        }

        // Render status line
        let status_y = map.height as u16;
        execute!(
            stdout,
            MoveTo(0, status_y),
            Print(format!("HP: {}/{}  ", player.hp, player.max_hp))
        )?;

        // Render messages
        for (i, message) in self.messages.iter().rev().take(3).enumerate() {
            execute!(
                stdout,
                MoveTo(0, status_y + 1 + i as u16),
                Print(message)
            )?;
        }

        // Render controls hint
        execute!(
            stdout,
            MoveTo(0, status_y + 5),
            Print("Arrow keys/WASD: move | Q: quit")
        )?;

        stdout.flush()?;
        Ok(())
    }

    fn get_char_at(&self, x: usize, y: usize, map: &Map, player: &Player, enemies: &[Enemy]) -> char {
        // Check for player
        if player.x == x && player.y == y {
            return player.to_char();
        }

        // Only show enemies in revealed areas
        if map.is_revealed(x, y) {
            for enemy in enemies {
                if enemy.is_alive() && enemy.x == x && enemy.y == y {
                    return enemy.to_char();
                }
            }
        }

        // Return map tile (handles fog of war internally)
        map.get_tile_char(x, y)
    }

    pub fn render_game_over(&self) -> io::Result<()> {
        let mut stdout = io::stdout();
        execute!(stdout, Clear(ClearType::All), MoveTo(10, 10))?;
        execute!(stdout, Print("=== GAME OVER ==="))?;
        execute!(stdout, MoveTo(10, 12))?;
        execute!(stdout, Print("You have been slain!"))?;
        execute!(stdout, MoveTo(10, 14))?;
        execute!(stdout, Print("Press any key to exit..."))?;
        stdout.flush()?;
        Ok(())
    }

    pub fn render_victory(&self) -> io::Result<()> {
        let mut stdout = io::stdout();
        execute!(stdout, Clear(ClearType::All), MoveTo(10, 10))?;
        execute!(stdout, Print("=== VICTORY! ==="))?;
        execute!(stdout, MoveTo(10, 12))?;
        execute!(stdout, Print("All enemies defeated!"))?;
        execute!(stdout, MoveTo(10, 14))?;
        execute!(stdout, Print("Press any key to exit..."))?;
        stdout.flush()?;
        Ok(())
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}
