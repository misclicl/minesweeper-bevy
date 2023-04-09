use bevy::log;
use bevy::prelude::*;

use crate::components::Covered;
use crate::components::TileCover;
use crate::{components::Coordinates, events::TileTriggerEvent, resources::board::Board};

pub fn handle_discover_event(
    board: Res<Board>,
    mut tile_trigger_event_reader: EventReader<TileTriggerEvent>,
    mut tiles: Query<(&mut Covered, &Coordinates, Entity)>,
) {
    for trigger_event in tile_trigger_event_reader.iter() {
        let entity = trigger_event.0;

        let (mut covered, coordinates, _) = tiles.get_mut(entity).unwrap();
        covered.is_covered = false;


        if !board.tile_map.is_empty_at(*coordinates) {
            return;
        }

        let temp = board.flood_discovery(coordinates);

        for (mut covered, _, entity) in tiles.iter_mut() {
            if temp.contains(&entity) {
                covered.is_covered = false;
            }
        }
    }
}

pub fn change_detection(
    query: Query<(Entity, &Covered, &Children), Changed<Covered>>,
    mut q_children: Query<(&mut Visibility, With<TileCover>)>,
) {
    for (entity, covered, children) in &query {
        if !covered.is_covered {
            log::debug!("{:?} changed: {:?}", entity, covered,);

            for &child in children.iter() {
                if let Ok((mut visibility, _)) = q_children.get_mut(child) {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}
