use macroquad::prelude::*;

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub width: f32,
    pub height: f32,
    pub health: i32,
    pub direction: i32,
    pub state: EnemyState,
    pub attack_speed: f32,
    pub attack_power: i32,
    pub last_shot_time: f32,
    pub projectiles: Vec<EnemyProjectile>,
}

pub enum EnemyState {
    Healthy,
    Damaged,
    Injured,
}

pub struct EnemyProjectile {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub width: f32,
    pub height: f32,
}

impl Enemy {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            speed: 2.0,
            width: 40.0,
            height: 40.0,
            health: 10,
            direction: 1,
            state: EnemyState::Healthy,
            attack_speed : 0.5,
            attack_power : 3,
            last_shot_time: 0.0,
            projectiles: Vec::new(),
        }
    }

    pub fn keep_enemy_screen_bounds(&mut self) {
        if self.x < 0.0 {
            self.x = 0.0;
            self.direction = 1;
        }

        if self.x + self.width > screen_width() {
            self.x = screen_width() - self.width;
            self.direction = -1;
        }
    }

    // Use per-enemy timer so each enemy has its own cooldown
    pub fn shoot(&mut self) -> bool {
        if self.last_shot_time >= self.attack_speed {
            self.projectiles.push(EnemyProjectile {
                x: self.x + self.width / 2.0 - 5.0,
                y: self.y + self.height,
                speed: 250.0,
                width: 10.0,
                height: 10.0,
            });
            self.last_shot_time = 0.0;
            return true;
        }
        false
    }


}
