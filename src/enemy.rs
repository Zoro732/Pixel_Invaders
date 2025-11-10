 pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub width: f32,
    pub height: f32,
    pub health: i32,
    pub direction: i32,
    pub state: EnemyState,
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
    pub fn spawn(&mut self, spawning_interval: f32) {
        
    }
}
