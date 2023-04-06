use bevy::prelude::*;

use crate::bounds::Bounds2;

use crate::{Coordinates, TileMap};

// #[derive(Default, Debug, Clone, Serialize, Deserialize, Resource, Reflect)]
#[derive(Debug, Clone, Resource)]
// #[reflect(Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
}

impl Board {
    pub fn mouse_position(&self, window: &Window, mouse_position: Vec2) -> Option<Coordinates> {
        let window_size = Vec2::new(window.width(), window.height());

        // window_size: (300, 300)
        // mouse_position: (100, 100)
        // position = (100, 100) - (150, 150) = (-50, -50)
        // Keep in mind (0, 0) is in the center of the screen
        let mouse_position = mouse_position - window_size / 2.;

        // Bounds check
        if !self.bounds.in_bounds(mouse_position) {
            return None;
        }
        // World space to board space
        let coordinates = mouse_position - self.bounds.position;
        Some(Coordinates {
            x: (coordinates.x / self.tile_size) as u16,
            y: (coordinates.y / self.tile_size) as u16,
        })
    }
}
