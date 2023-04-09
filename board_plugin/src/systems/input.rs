use bevy::log;
use bevy::prelude::*;

use crate::Board;
use crate::events::TileTriggerEvent;

pub fn handle_input(
    windows: Query<&Window>,
    board: Res<Board>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut tile_trigger_ewr: EventWriter<TileTriggerEvent>
) {
    let window = windows.get_single().unwrap();
    let position = window.cursor_position();

    if let Some(pos) = position {
        let tile_coordinates = board.mouse_position(window, pos);

        if let Some(coordinates) = tile_coordinates {
            if mouse_button_input.just_released(MouseButton::Left) {
                let tile = board.get_tile_entity(&coordinates).unwrap();
                tile_trigger_ewr.send(TileTriggerEvent(*tile));

                log::info!("left mouse just released");
                log::info!("Trying to uncover tile on {}", coordinates);
            }

            if mouse_button_input.just_released(MouseButton::Right) {
                info!("right mouse just released");
                log::info!("Trying to mark tile on {}", coordinates);
            }
        }
    };
}
