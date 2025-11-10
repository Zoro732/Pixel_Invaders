mod enemy;
mod player;

use core::panic;

use enemy::*;
use macroquad::prelude::*;
use player::*;

#[macroquad::main("Pixel Invaders")]
async fn main() {
    let mut player = Player::new(
        screen_width() / 2.0 - 120.0 / 2.0,
        screen_height() - 60.0 - 10.0,
    );

    // Vectors to hold projectiles toward enemies
    let mut player_projectiles: Vec<PlayerProjectile> = Vec::new();
    let mut last_player_shot_time: f32 = 0.0;

    // Vectors to hold enemies
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut last_enemy_spawn_time: f32 = 0.0;
    let mob_cap = 4;
    let enemy_spawn_interval = 2.0;

    // removed global enemy shot timer in favor of per-enemy cooldowns

    loop {
        clear_background(BLACK);

        let current_time: f32 = get_frame_time();
        last_player_shot_time += current_time;
        // global timers updated earlier; per-enemy timers updated in enemy loop below
        last_enemy_spawn_time += current_time;

        // Player movement
        if is_key_down(KeyCode::Right) {
            player.move_right();
        }
        if is_key_down(KeyCode::Left) {
            player.move_left();
        }

        // Keep the player within screen bounds
        player.keep_player_screen_bounds();

        //Player shooting
        if player.shoot(&mut player_projectiles, last_player_shot_time) {
            last_player_shot_time = 0.0;
        }

        // Update and draw player projectiles (frame-rate independent)
        for proj in &mut player_projectiles {
            proj.y -= proj.speed * current_time;
            draw_rectangle(proj.x, proj.y, proj.width, proj.height, GRAY);
        }

        // Enemy spawning
        if last_enemy_spawn_time >= enemy_spawn_interval && enemies.len() < mob_cap {
            enemies.push(Enemy::new(
                rand::gen_range(0.0, screen_width() - 40.0),
                50.0,
            ));
            last_enemy_spawn_time = 0.0;
        }

        // Update and draw enemies
        for enemy in &mut enemies {
            enemy.x += enemy.speed * enemy.direction as f32;

            // Keep the enemy within screen bounds
            enemy.keep_enemy_screen_bounds();

            // Update enemy state based on health
            match enemy.health {
                h if h < 2 => enemy.state = EnemyState::Injured,
                h if h < 5 => enemy.state = EnemyState::Damaged,
                _ => enemy.state = EnemyState::Healthy,
            }

            // Increment per-enemy timer and let enemy decide to shoot
            enemy.last_shot_time += current_time;
            if enemy.shoot() {}

            // Check for collisions with player projectiles
            for proj in &mut player_projectiles {
                if intersect_projectile_enemy(proj, enemy) {
                    enemy.health -= player.attack_power;
                    proj.y = -proj.height - 1000.0;
                }
            }

            // Update and draw this enemy's projectiles (each enemy has its own vector)
            for proj in &mut enemy.projectiles {
                proj.y += proj.speed * current_time;
                draw_rectangle(proj.x, proj.y, proj.width, proj.height, ORANGE);

                // Check for collisions with player
                if intersect_projectile_player(proj, &player) {
                    player.health -= enemy.attack_power;
                    proj.y = proj.height + 1000.0;
                    println!("Hit and player healt {}", player.health);
                    if player.health <= 0 {
                        panic!();
                    }
                    player.state = match player.state {
                        PlayerState::Healthy => PlayerState::Damaged,
                        PlayerState::Damaged => PlayerState::Injured,
                        PlayerState::Injured => PlayerState::Injured,
                    };
                }
            }

            // remove this enemy's off-screen projectiles
            enemy.projectiles.retain(|proj| proj.y < screen_height());

            draw_rectangle(
                enemy.x,
                enemy.y,
                enemy.width,
                enemy.height,
                match enemy.state {
                    EnemyState::Healthy => GREEN,
                    EnemyState::Damaged => YELLOW,
                    EnemyState::Injured => RED,
                },
            );
        }

        // per-enemy timers used; no global reset needed

        // Remove off-screen projectiles
        player_projectiles.retain(|proj| proj.y + proj.height > 0.0);
        // per-enemy projectile vectors are handled inside the enemy loop

        // Update score and remove defeated enemies
        for enemy in &enemies {
            if enemy.health <= 0 {
                player.score += 1;
            }
        }
        enemies.retain(|enemy| enemy.health > 0);

        // Draw player and score
        let text_size = measure_text(&format!("Score: {}", player.score), None, 50, 1.0);
        draw_rectangle(
            player.x,
            player.y,
            player.width,
            player.height,
            match player.state {
                PlayerState::Healthy => GREEN,
                PlayerState::Damaged => YELLOW,
                PlayerState::Injured => RED,
            },
        );

        draw_text(
            &format!("Score: {}", player.score),
            screen_width() - text_size.width - 50.0,
            50.0,
            50.0,
            WHITE,
        );

        next_frame().await
    }
}

fn intersect_projectile_enemy(projectile: &PlayerProjectile, enemy: &Enemy) -> bool {
    // AABB collision - inclusive check
    !(projectile.x + projectile.width < enemy.x
        || projectile.x > enemy.x + enemy.width
        || projectile.y + projectile.height < enemy.y
        || projectile.y > enemy.y + enemy.height)
}

fn intersect_projectile_player(projectile: &EnemyProjectile, player: &Player) -> bool {
    !(projectile.x + projectile.width < player.x
        || projectile.x > player.x + player.width
        || projectile.y + projectile.height < player.y
        || projectile.y > player.y + player.height)
}
