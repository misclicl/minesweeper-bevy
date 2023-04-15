use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};

use crate::bounds::Bounds2;
use crate::queue::Queue;
use crate::{Coordinates, TileMap};

// #[derive(Default, Debug, Clone, Serialize, Deserialize, Resource, Reflect)]
#[derive(Debug, Clone, Resource)]
// #[reflect(Resource)]
pub struct Board {
    pub entity: Entity,

    pub bounds: Bounds2,
    pub tile_size: f32,

    pub tile_map: TileMap,
    pub tiles: HashMap<Coordinates, Entity>,

    pub coordinates_discovered: HashSet<Coordinates>,
    pub coordinates_marked: HashSet<Coordinates>,
}

impl Board {
    pub fn new(
        entity: Entity,
        bounds: Bounds2,
        tile_size: f32,
        tile_map: TileMap,
        tiles: HashMap<Coordinates, Entity>,
    ) -> Self {
        Board {
            entity,
            bounds,
            tile_size,
            tile_map,
            coordinates_discovered: HashSet::with_capacity(tiles.len()),
            coordinates_marked: HashSet::with_capacity(tiles.len()),
            tiles,
        }
    }

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

    pub fn get_tile_entity(&self, coordinates: Coordinates) -> Option<&Entity> {
        self.tiles.get(&coordinates)
    }

    pub fn get_adjacent_tiles(&self, coordinates: Coordinates) -> Vec<Entity> {
        self.tile_map
            .get_neighbor_coordinates(coordinates)
            .filter_map(|neighbor_coordinates| self.tiles.get(&neighbor_coordinates))
            .copied()
            .collect()
    }

    pub fn flood_discovery(&mut self, coordinates: &Coordinates) -> HashSet<Entity> {
        let mut queue = Queue::from([*coordinates]);
        let mut discovered: HashMap<Entity, Coordinates> = HashMap::new();
        let mut visited: HashSet<Entity> = HashSet::new();

        while let Some(current_coordinates) = queue.dequeue() {
            if let Some(entity) = self.get_tile_entity(current_coordinates) {
                if self.is_flag_at(&current_coordinates) {
                    continue;
                }

                discovered.insert(*entity, current_coordinates);

                if !self.tile_map.is_empty_at(current_coordinates) {
                    continue;
                }

                for neighbor_coordinates in
                    self.tile_map.get_neighbor_coordinates(current_coordinates)
                {
                    if let Some(entity) = self.get_tile_entity(neighbor_coordinates) {
                        if !visited.contains(entity) {
                            queue.enqueue(neighbor_coordinates);
                        }
                        visited.insert(*entity);
                    }
                }
            }
        }

        for v in discovered.values() {
            self.coordinates_discovered.insert(*v);
        }

        discovered.keys().cloned().collect()
    }

    fn unmark_tile(&mut self, coords: &Coordinates) -> bool {
        self.coordinates_marked.remove(coords)
    }

    pub fn try_toggle_mark(&mut self, coordinates: Coordinates) -> bool {
        let is_marked = if self.coordinates_marked.contains(&coordinates)
            || self.coordinates_discovered.contains(&coordinates)
        {
            self.unmark_tile(&coordinates);
            false
        } else {
            self.coordinates_marked.insert(coordinates);
            true
        };

        is_marked
    }

    pub fn is_completed(&self) -> bool {
        let goal_count =
            (self.tile_map.height * self.tile_map.width - self.tile_map.bomb_count) as usize;

        if self.coordinates_discovered.len() == goal_count {
            // TODO: corner case when last element is bomb
            return true;
        }
        return false;
    }

    pub fn is_flag_at(&self, coordinates: &Coordinates) -> bool {
        self.coordinates_marked.contains(&coordinates)
    }
}
