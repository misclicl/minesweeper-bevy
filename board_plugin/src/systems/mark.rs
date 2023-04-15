use bevy::log;
use bevy::prelude::*;

use crate::components::Flag;
use crate::events::TileMarkEvent;
use crate::resources::board_assets::BoardAssets;
use crate::{components::Coordinates, resources::board::Board};

pub fn mark_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    board_assets: Res<BoardAssets>,

    mut tile_trigger_event_reader: EventReader<TileMarkEvent>,

    mut q_tiles: Query<(&Coordinates, &Children)>,
    mut q_children: Query<(Entity, With<Flag>), With<Parent>>,
) {
    for event in tile_trigger_event_reader.iter() {
        let entity = event.0;

        if let Ok((coordinates, children)) = q_tiles.get_mut(entity) {
            if board.try_toggle_mark(*coordinates) {
                let mut transform = Transform::from_xyz(0., 0., 2.);
                transform.scale = Vec3::from_array([0.8;3]);
                
                commands.entity(entity).with_children(|parent| {
                    parent
                        .spawn(SpriteBundle {
                            texture: board_assets.flag_material.texture.clone(),
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(board.tile_size)),
                                color: board_assets.flag_material.color,
                                ..Default::default()
                            },
                            transform,
                            ..default()
                        })
                        .insert(Flag {})
                        .insert(Name::new("Flag"));
                });

                return;
            }

            for &child in children.iter() {
                if let Ok((flag_entity, _)) = q_children.get_mut(child) {
                    commands.entity(flag_entity).despawn_recursive();
                }
            }
        }
    }
}
