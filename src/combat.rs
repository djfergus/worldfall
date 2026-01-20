use rand::Rng;
use crate::player::Player;
use crate::enemy::Enemy;

pub struct CombatResult {
    pub damage: i32,
    pub message: String,
}

pub fn player_attack(player: &Player, enemy: &mut Enemy) -> CombatResult {
    let mut rng = rand::thread_rng();
    let variance = rng.gen_range(0..=3);
    let damage = (player.power - variance).max(1);

    enemy.take_damage(damage);

    let message = if enemy.is_alive() {
        format!("You hit the goblin for {} damage!", damage)
    } else {
        format!("You killed the goblin!")
    };

    CombatResult { damage, message }
}

pub fn enemy_attack(enemy: &Enemy, player: &mut Player) -> CombatResult {
    let mut rng = rand::thread_rng();
    let variance = rng.gen_range(0..=2);
    let damage = (enemy.power - variance).max(1);

    player.take_damage(damage);

    let message = if player.is_alive() {
        format!("The goblin hits you for {} damage!", damage)
    } else {
        format!("The goblin killed you!")
    };

    CombatResult { damage, message }
}
