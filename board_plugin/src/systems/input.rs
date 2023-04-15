use bevy::log;
use bevy::prelude::*;

use crate::events::{TileDiscoverEvent, TileMarkEvent};
use crate::Board;

pub fn handle_input(
    windows: Query<&Window>,
    board: Res<Board>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut tile_discover_event_writer: EventWriter<TileDiscoverEvent>,
    mut tile_mark_event_writer: EventWriter<TileMarkEvent>,
) {
    let window = match windows.get_single() {
        Ok(w) => w,
        Err(e) => {
            log::warn!("Failed to retrieve window: {:?}", e);
            return;
        }
    };

    let cursor_position = match window.cursor_position() {
        Some(pos) => pos,
        None => {
            return;
        }
    };

    if let Some(coordinates) = board.mouse_position(window, cursor_position) {
        if mouse_button_input.just_released(MouseButton::Left) {
            if let Some(tile) = board.get_tile_entity(coordinates) {
                tile_discover_event_writer.send(TileDiscoverEvent(*tile));
                // log::info!("Trying to uncover tile on {}", coordinates);
            }
        }

        if mouse_button_input.just_released(MouseButton::Right) {
            if let Some(tile) = board.get_tile_entity(coordinates) {
                tile_mark_event_writer.send(TileMarkEvent(*tile));
                // log::info!("Trying to mark tile on {}", coordinates);
            }
        }
    }
}
