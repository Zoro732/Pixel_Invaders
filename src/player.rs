use macroquad::prelude::*;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub width: f32,
    pub height: f32,
    pub attack_power: i32,
    pub attack_speed: f32,
    pub score: i32,
    pub state: PlayerState,
    pub health: i32,
}

pub enum PlayerState {
    Healthy,
    Damaged,
    Injured,
}

pub struct PlayerProjectile {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub width: f32,
    pub height: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            speed: 15.0,
            width: 60.0,
            height: 60.0,
            attack_power: 2,
            attack_speed: 0.3,
            score: 0,
            state: PlayerState::Healthy,
            health: 10,
        }
    }

    pub fn move_left(&mut self) {
          if self.health <= 0 {
            return;
        }
        self.x -= self.speed;
    }

    pub fn move_right(&mut self) {
        if self.health <= 0 {
            return;
        }
        self.x += self.speed;
    }

    pub fn keep_player_screen_bounds(&mut self) {
        // Keep the player within screen bounds
          if self.health <= 0 {
            return;
        }
        if self.x < 0.0 {
            self.x = 0.0;
        }
        if self.x + self.width > screen_width() {
            self.x = screen_width() - self.width;
        }
    }

    pub fn shoot(
        &mut self,
        projectile_vector: &mut Vec<PlayerProjectile>,
        last_player_shot_time: f32,
    ) -> bool {
        if self.health <= 0 {
            return false;
        }
        if last_player_shot_time >= self.attack_speed {
            projectile_vector.push(PlayerProjectile {
                x: self.x + self.width / 2.0 - 5.0,
                y: self.y,
                speed: 700.0,
                width: 10.0,
                height: 10.0,
            });
            return true;
        }
        return false;
    }

    pub fn draw(&self) {
        if self.health <= 0 {
            return;
        }
            draw_rectangle(
                self.x,
                self.y,
                self.width,
                self.height,
                match self.state {
                    PlayerState::Healthy => GREEN,
                    PlayerState::Damaged => YELLOW,
                    PlayerState::Injured => RED,
                },
            );
        
    }
}
