use bevy::prelude::Vec2;

// #[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
#[derive(Debug, Clone)]
pub struct Bounds2 {
    pub position: Vec2,
    pub size: Vec2,
}

impl Bounds2 {
    pub fn in_bounds(&self, coords: Vec2) -> bool {
        coords.x >= self.position.x
            && coords.x <= self.position.x + self.size.x
            && coords.y >= self.position.y
            && coords.y <= self.position.y + self.size.y
    }
}
