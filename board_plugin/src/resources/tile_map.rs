use crate::components::Coordinates;
use crate::resources::tile::Tile;
use rand::{thread_rng, Rng};

use std::ops::{Deref, DerefMut};

const NEIGHBOR_COORDS: [(i8, i8); 8] = [
    // Bottom left
    (-1, -1),
    // Bottom
    (0, -1),
    // Bottom right
    (1, -1),
    // Left
    (-1, 0),
    // Right
    (1, 0),
    // Top Left
    (-1, 1),
    // Top
    (0, 1),
    // Top right
    (1, 1),
];

#[derive(Debug, Clone)]
pub struct TileMap {
    pub bomb_count: u16,
    pub height: u16,
    pub width: u16,
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    pub fn empty(width: u16, height: u16) -> Self {
        let map = (0..height)
            .into_iter()
            .map(|_| (0..width).into_iter().map(|_| Tile::Empty).collect())
            .collect();

        Self {
            bomb_count: 0,
            height,
            width,
            map,
        }
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!(
            "Map ({}, {}) with {} bombms:\n",
            self.width, self.height, self.bomb_count
        );
        let line: String = (0..=(self.width + 1)).into_iter().map(|_| '-').collect();
        buffer = format!("{}{}\n", buffer, line);

        for line in self.iter().rev() {
            buffer = format!("{}|", buffer);
            for tile in line.iter() {
                buffer = format!("{}{}", buffer, tile.console_output());
            }
            buffer = format!("{}|\n", buffer);
        }
        format!("{}{}", buffer, line)
    }

    pub fn get_neighbor_coordinates(
        &self,
        coordinates: Coordinates,
    ) -> impl Iterator<Item = Coordinates> {
        NEIGHBOR_COORDS
            .iter()
            .copied()
            .map(move |tuple| coordinates + tuple)
    }

    pub fn is_empty_at(&self, coordinates: Coordinates) -> bool {
        if coordinates.x >= self.width || coordinates.y >= self.height {
            return false;
        };
        self.map[coordinates.y as usize][coordinates.x as usize].is_empty()
    }

    pub fn is_bomb_at(&self, coordinates: Coordinates) -> bool {
        if coordinates.x >= self.width || coordinates.y >= self.height {
            return false;
        };
        self.map[coordinates.y as usize][coordinates.x as usize].is_bomb()
    }

    pub fn bomb_count_at(&self, coordinates: Coordinates) -> u8 {
        if self.is_bomb_at(coordinates) {
            return 0;
        }
        let res = self
            .get_neighbor_coordinates(coordinates)
            .filter(|coord| self.is_bomb_at(*coord))
            .count();
        res as u8
    }

    pub fn set_bombs(&mut self, bomb_count: u16) {
        self.bomb_count = bomb_count;
        let mut bombs_left = bomb_count;
        let mut rng = thread_rng();

        while bombs_left > 0 {
            let (x, y) = (
                rng.gen_range(0..self.width) as usize,
                rng.gen_range(0..self.height) as usize,
            );

            if let Tile::Empty = self.map[y][x] {
                self[y][x] = Tile::Bomb;
                bombs_left -= 1;
            }
            // Place bomb neighbors
            for y in 0..self.height {
                for x in 0..self.width {
                    let coords = Coordinates { x, y };
                    if self.is_bomb_at(coords) {
                        continue;
                    }
                    let num = self.bomb_count_at(coords);
                    if num == 0 {
                        continue;
                    }
                    let tile = &mut self[y as usize][x as usize];
                    *tile = Tile::BombNeighbor(num);
                }
            }
        }
    }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
