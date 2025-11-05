use std::default;

use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let default_width = 120.0;
    let default_height = 60.0;
    let default_y = screen_height() - default_height - 10.0;
    let default_x = screen_width() / 2.0 - default_width / 2.0;
    let default_speed = 15.0;
    let default_projectile_interval = 0.5;
    let mut player = Player { x: default_x, 
                                    y: default_y, 
                                    speed: default_speed, 
                                    width: default_width, 
                                    height: default_height,
                                    projectile_interval: default_projectile_interval };

    let mut projectiles: Vec<Projectile> = Vec::new();
    let mut last_shot_time: f32 = 0.0;
    
    
    loop {
        clear_background(BLACK);

        let current_time: f32 = get_frame_time();
        last_shot_time += current_time;

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

        if last_shot_time >= player.projectile_interval {
            projectiles.push(Projectile {
                x: player.x + player.width / 2.0 - 5.0,
                y: player.y,
                speed: 10.0,
                width: 10.0,
                height: 10.0 
            });
            last_shot_time = 0.0;
        }

        for proj in &mut projectiles {
            proj.y -= proj.speed;
            draw_rectangle(proj.x, proj.y, proj.width, proj.height, GRAY);
        }


        draw_rectangle(player.x, player.y, 120.0, 60.0, WHITE);
        next_frame().await
    }
}

struct Player {
    x: f32,
    y: f32,
    speed: f32,
    width: f32,
    height: f32,
    projectile_interval: f32,
}

struct Projectile {
    x: f32,
    y: f32,
    speed: f32,
    width: f32,
    height: f32,
}