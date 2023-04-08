use bevy::log;
use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::queue::Queue;
use crate::{
    components::{Bomb, BombNeighbor, Coordinates, Uncover},
    events::TileTriggerEvent,
    resources::board::Board,
};

pub fn trigger_event_handler(
    mut commands: Commands,
    board: Res<Board>,
    mut tile_trigger_event_reader: EventReader<TileTriggerEvent>,
) {
    for trigger_event in tile_trigger_event_reader.iter() {
        if let Some(entity) = board.get_tile_cover_entity(&trigger_event.0) {
            commands.entity(*entity).insert(Uncover);
        }
    }
}

// TODO: terrible performance
pub fn uncover_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    tile_covers: Query<(Entity, &Parent), With<Uncover>>,
    tiles: Query<(&Coordinates, Option<&Bomb>, Option<&BombNeighbor>, Entity)>,
) {
    for (tile_cover_entity, tile_cover_parent) in tile_covers.iter() {
        commands.entity(tile_cover_entity).despawn_recursive();

        let (coords, bomb, bomb_counter, _) = match tiles.get(tile_cover_parent.get()) {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}", e);
                continue;
            }
        };

        match board.try_uncover_tile(coords) {
            None => log::debug!("Tried to uncover an already uncovered tile"),
            Some(e) => log::debug!("Uncovered tile {} (entity: {:?})", coords, e),
        }

        if bomb.is_some() {
            log::info!("Boom !");
            // TODO: Add explosion event
        } else if bomb_counter.is_none() {
            // let adjacent_cover_tiles = board.get_adjacent_cover_tiles(*coords);
            let adjacent_tiles_coordinates = board.get_adjacent_cover_tiles_coordinates(*coords);

            let mut queue = Queue::from(adjacent_tiles_coordinates);
            let mut visited: HashSet<Coordinates> = HashSet::new();
            let mut to_be_processed: HashSet<Coordinates> = HashSet::new();

            while !queue.is_empty() {
                let tile_coordinates = queue.dequeue().unwrap();
                visited.insert(tile_coordinates);
                // 1. get tile cover by its coordinates and do uncover thing
                if let Some(tile_cover) = board.get_tile_cover_entity(&tile_coordinates) {
                    commands.entity(*tile_cover).insert(Uncover);
                } else {
                    println!("uh-oh, {:?}", tile_coordinates);
                }

                // 2. find adjacent coordinates
                let next_coordinates = board.get_adjacent_cover_tiles_coordinates(tile_coordinates);

                // 3. for each adjacent coordinates pair find parent and check the bomb_count
                for (parent_coordinates, bomb, bomb_counter, _) in tiles.iter() {
                    if next_coordinates.contains(parent_coordinates)
                        && !to_be_processed.contains(parent_coordinates)
                        && bomb_counter.is_none()
                        && !bomb.is_some()
                    {
                        println!("AHAHAHHA, {:?}", next_coordinates);
                        to_be_processed.insert(*parent_coordinates);
                        queue.enqueue(*parent_coordinates);
                    }
                }
            }
        }
    }
}
