use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let default_width = 120.0;
    let default_height = 60.0;
    let default_y = screen_height() - default_height - 10.0;
    let default_x = screen_width() / 2.0 - default_width / 2.0;
    let default_speed = 15.0;
    let mut player = Player { x: default_x, 
                                    y: default_y, 
                                    speed: default_speed, 
                                    width: default_width, 
                                    height: default_height,
                                    attack_power: 2,
                                    attack_speed: 0.1,
                                };

    let mut projectiles: Vec<Projectile> = Vec::new();
    let mut last_shot_time: f32 = 0.0;

    let mut enemies: Vec<Enemy> = Vec::new();
    let mut last_enemy_spawn_time: f32 = 0.0;
    let mob_cap = 4;
    let enemy_spawn_interval = 2.0;
    
    loop {
        clear_background(BLACK);

        let current_time: f32 = get_frame_time();
        last_shot_time += current_time;
        last_enemy_spawn_time += current_time;

        // Player movement
        if is_key_down(KeyCode::Right) {
            player.x += player.speed;
        }
        if is_key_down(KeyCode::Left) {
            player.x -= player.speed;
        }

        // Keep the player within screen bounds
        if player.x < 0.0 {
            player.x = 0.0;
        }
        if player.x + player.width > screen_width() {
            player.x = screen_width() - player.width;
        }

        //Player shooting
        if last_shot_time >= player.attack_speed {
            projectiles.push(Projectile {
                x: player.x + player.width / 2.0 - 5.0,
                y: player.y,
                speed: 10.0,
                width: 10.0,
                height: 10.0 
            });
            last_shot_time = 0.0;
        }

        // Update and draw projectiles
        for proj in &mut projectiles {
            proj.y -= proj.speed;
            draw_rectangle(proj.x, proj.y, proj.width, proj.height, GRAY);
        }

        if last_enemy_spawn_time >= enemy_spawn_interval && enemies.len() < mob_cap {
            enemies.push(Enemy {
                x: rand::gen_range(0.0, screen_width() - 40.0),
                y: 50.0,
                speed: 2.0,
                width: 40.0,
                height: 40.0,
                health: 10,
                direction: 1,
                state: EnemyState::Healthy,
            });
            last_enemy_spawn_time = 0.0;
        }

        // Update and draw enemies
        for enemy in &mut enemies {
            enemy.x += enemy.speed * enemy.direction as f32;

            // Keep the enemy within screen bounds
            if enemy.x < 0.0 {
                enemy.x = 0.0;
                enemy.direction = 1;
            }

            if enemy.x + enemy.width > screen_width() {
                enemy.x = screen_width() - enemy.width;
                enemy.direction = -1;
            }

           for proj in &mut projectiles {
               if intersect_projectile_enemy(proj, enemy) {
                println!("Hit!");
                enemy.health -= player.attack_power;
                proj.y = -proj.height - 1000.0;

               }
           }

           match enemy.health {
               h if h < 2 => enemy.state = EnemyState::Injured,
               h if h < 5 => enemy.state = EnemyState::Damaged,
               _ => enemy.state = EnemyState::Healthy, 
            }

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

        enemies.retain(|enemy| enemy.health > 0);

        draw_rectangle(player.x, player.y, player.width, player.height, WHITE);
        draw_text("score", screen_width()/2.0 - 50.0, 50.0, 50.0, WHITE);
        next_frame().await
    }
}

fn intersect_projectile_enemy(projectile: &mut Projectile, enemy: &mut Enemy) -> bool {

    if projectile.x < enemy.x + enemy.width &&
       projectile.x + projectile.width > enemy.x &&
       projectile.y < enemy.y + enemy.height &&
       projectile.y + projectile.height > enemy.y {
       return true
    }
    return false;
}

struct Player {
    x: f32,
    y: f32,
    speed: f32,
    width: f32,
    height: f32,
    attack_power: i32,
    attack_speed: f32,
}

struct Projectile {
    x: f32,
    y: f32,
    speed: f32,
    width: f32,
    height: f32,
}

struct Enemy {
    x: f32,
    y: f32,
    speed: f32,
    width: f32,
    height: f32,
    health: i32,
    direction: i32,
    state: EnemyState,
}
enum EnemyState {
    Healthy,
    Damaged,
    Injured,
}