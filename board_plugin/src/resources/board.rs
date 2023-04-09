use bevy::log;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};

use crate::bounds::Bounds2;
use crate::queue::Queue;
use crate::{Coordinates, TileMap};

// #[derive(Default, Debug, Clone, Serialize, Deserialize, Resource, Reflect)]
#[derive(Debug, Clone, Resource)]
// #[reflect(Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub tiles: HashMap<Coordinates, Entity>,
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

    pub fn get_tile_entity(&self, coordinates: &Coordinates) -> Option<&Entity> {
        self.tiles.get(coordinates)
    }

    pub fn get_adjacent_tiles(&self, coordinates: Coordinates) -> Vec<Entity> {
        self.tile_map
            .get_neighbor_coordinates(coordinates)
            .filter_map(|neighbor_coordinates| self.tiles.get(&neighbor_coordinates))
            .copied()
            .collect()
    }

    pub fn flood_discovery(&self, coordinates: &Coordinates) -> HashSet<&Entity> {
        let mut queue = Queue::from([*coordinates]);
        let mut visited: HashSet<&Entity> = HashSet::new();
        let mut discovered: HashSet<&Entity> = HashSet::new();

        let mut counter = 0;
        // TODO: Floods to all non-bomb cells
        while let Some(current_coordinates) = queue.dequeue() {
            counter += 1;
            if let Some(entity) = self.get_tile_entity(&current_coordinates) {
                visited.insert(entity);

                for neighbor_coordinates in
                    self.tile_map.get_neighbor_coordinates(current_coordinates)
                {
                    let is_empty = self.tile_map.is_empty_at(current_coordinates);
                    let is_bomb_at = self.tile_map.is_bomb_at(neighbor_coordinates);
                    // if is_empty {
                    if !is_bomb_at && is_empty {
                        if let Some(entity) = self.get_tile_entity(&neighbor_coordinates) {
                            if !visited.contains(entity) && !discovered.contains(entity) {
                                queue.enqueue(neighbor_coordinates);
                            }
                            discovered.insert(entity);
                        }
                    }
                }
            }
        }

        // TODO: no need for two objects?
        log::info!("{:?}", visited);
        log::info!("Counter: {:?}", counter);
        // result
        visited
    }
}
