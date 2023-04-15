use bevy::log;
use bevy::prelude::*;

use crate::components::Covered;
use crate::components::TileCover;
use crate::{
    components::Coordinates,
    events::{BombExplosionEvent, TileDiscoverEvent},
    resources::board::Board,
};

pub fn handle_discover_event(
    mut board: ResMut<Board>,
    mut bomb_explosion_event_writer: EventWriter<BombExplosionEvent>,
    mut tile_trigger_event_reader: EventReader<TileDiscoverEvent>,

    mut tiles: Query<(&mut Covered, &Coordinates, Entity)>,
) {
    for trigger_event in tile_trigger_event_reader.iter() {
        let entity = trigger_event.0;

        if let Ok((mut covered, coordinates, _)) = tiles.get_mut(entity) {
            if board.is_flag_at(coordinates) {
                continue;
            }
            board.coordinates_discovered.insert(*coordinates);
            covered.is_covered = false;

            if board.tile_map.is_bomb_at(*coordinates) {
                log::info!("Boom!");
                bomb_explosion_event_writer.send(BombExplosionEvent);
            }

            if !board.tile_map.is_empty_at(*coordinates) {
                continue;
            }

            let discovered_entities = board.flood_discovery(&coordinates).clone();
            for (mut covered, _, entity) in tiles.iter_mut() {
                if discovered_entities.contains(&entity) {
                    covered.is_covered = false;
                }
            }
        }
    }
}

pub fn discover_tiles(
    query: Query<(&Covered, &Children), Changed<Covered>>,
    mut q_children: Query<(&mut Visibility, With<TileCover>)>,
) {
    query
        .iter()
        .filter(|(covered, _)| !covered.is_covered)
        .for_each(|(_, children)| {
            children.iter().for_each(|&child| {
                if let Ok((mut visibility, _)) = q_children.get_mut(child) {
                    *visibility = Visibility::Hidden;
                }
            });
        });
}
