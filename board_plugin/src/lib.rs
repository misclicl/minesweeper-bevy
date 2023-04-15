mod bounds;
pub mod components;
mod events;
mod queue;
pub mod resources;
mod systems;

use bevy::log;
use bevy::prelude::*;
use bevy::utils::HashMap;

use resources::board::Board;
use resources::tile::Tile;
use resources::tile_map::TileMap;
use resources::BoardAssets;
use resources::BoardOptions;
use resources::BoardPosition;
use resources::TileSize;

use components::Bomb;
use components::BombNeighbor;
use components::Coordinates;
use components::Covered;
#[cfg(feature = "debug")]
use components::Flag;
use components::TileCover;

use events::BombExplosionEvent;
use events::TileDiscoverEvent;
use events::TileMarkEvent;

use bounds::Bounds2;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    InGame,
    Out,
}

pub struct BoardPlugin<T> {
    pub state: T,
}

impl<T: States> Plugin for BoardPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system(Self::create_board.in_schedule(OnEnter(AppState::InGame)))
            .add_systems(
                (
                    systems::input::handle_input,
                    systems::uncover::handle_discover_event,
                    systems::uncover::discover_tiles,
                    systems::mark::mark_tiles,
                )
                    .in_set(OnUpdate(AppState::InGame)),
            )
            .add_system(Self::cleanup_board.in_schedule(OnExit(AppState::InGame)))
            .add_event::<BombExplosionEvent>()
            .add_event::<TileMarkEvent>()
            .add_event::<TileDiscoverEvent>();

        log::info!("Loaded Board Plugin");
        #[cfg(feature = "debug")]
        {
            app.register_type::<BombNeighbor>();
            app.register_type::<Bomb>();
            app.register_type::<BoardOptions>();
            app.register_type::<Covered>();
            app.register_type::<Flag>();
        }
    }
}

impl<T> BoardPlugin<T> {
    /// System to generate the complete board
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        window: Query<&Window>,
        mut tile_trigger_ewr: EventWriter<TileDiscoverEvent>,
        board_assets: Res<BoardAssets>,
    ) {
        let options = match board_options {
            None => BoardOptions::default(),
            Some(opts) => opts.clone(),
        };

        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.set_bombs(options.bomb_count);

        #[cfg(feature = "debug")]
        // Tilemap debugging
        log::info!("{}", tile_map.console_output());

        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptive { min, max } => {
                Self::adaptive_tile_size(window, (min, max), (tile_map.width, tile_map.height))
            }
        };

        let board_size = Vec2::new(
            tile_map.width as f32 * tile_size,
            tile_map.height as f32 * tile_size,
        );
        log::info!("board size: {}", board_size);
        // We define the board anchor position (bottom left)
        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardPosition::Custom(p) => p,
        };

        // let mut covered_tiles = HashSet::with_capacity((tile_map.width * tile_map.height).into());
        let mut tiles = HashMap::with_capacity((tile_map.width * tile_map.height).into());
        let mut safe_start = None;

        let board_entity = commands
            .spawn(SpatialBundle {
                visibility: Visibility::Visible,
                transform: Transform::from_translation(board_position.into()),
                ..Default::default()
            })
            .insert(Name::new("Board"))
            .with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: board_assets.board_material.color,
                            custom_size: Some(board_size),
                            ..Default::default()
                        },
                        texture: board_assets.board_material.texture.clone(),
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));

                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    &board_assets,
                    &mut tiles,
                    &mut safe_start,
                );
            })
            .id();

        if options.safe_start {
            if let Some(entity) = safe_start {
                tile_trigger_ewr.send(TileDiscoverEvent(entity));
            }
        }

        commands.insert_resource(Board::new(
            board_entity,
            Bounds2 {
                position: Vec2::new(board_position.x, board_position.y),
                size: board_size,
            },
            tile_size,
            tile_map,
            tiles,
        ));
    }

    fn adaptive_tile_size(
        windows: Query<&Window>,
        (min, max): (f32, f32),
        (width, height): (u16, u16),
    ) -> f32 {
        for window in &windows {
            // TODO: fix this (get primary window instead)
            let max_width = window.resolution.width() / width as f32;
            let max_heigth = window.resolution.height() / height as f32;
            return max_width.min(max_heigth).clamp(min, max);
        }

        return 0.;
    }

    fn bomb_count_text_bundle(
        count: u8,
        board_assets: &BoardAssets, 
    ) -> SpriteBundle {
        let asset = match count {
            1 => board_assets.material_1.texture.clone(),
            2 => board_assets.material_2.texture.clone(),
            3 => board_assets.material_3.texture.clone(),
            _ => panic!()
        };

        SpriteBundle {
            sprite: Sprite {
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 2.),
            texture: asset,
            ..default()
        }
    }

    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        size: f32,
        padding: f32,
        board_assets: &BoardAssets,
        tiles: &mut HashMap<Coordinates, Entity>,
        safe_start_entity: &mut Option<Entity>,
    ) {
        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let coordinates = Coordinates {
                    x: x as u16,
                    y: y as u16,
                };

                let covered = Covered { is_covered: true };

                let mut cmd = parent.spawn_empty();

                // spawn border tiles
                

                // spawn tile base
                let base_command = cmd
                    .insert(SpriteBundle {
                        sprite: Sprite {
                            color: board_assets.tile_material.color,
                            custom_size: Some(Vec2::splat(size - padding as f32)),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(
                            (x as f32 * size) + (size / 2.),
                            (y as f32 * size) + (size / 2.),
                            1.,
                        ),
                        texture: board_assets.tile_material.texture.clone(),
                        ..Default::default() // We add the `Coordinates` component to our tile entity
                    })
                    .insert(covered)
                    .insert(coordinates);

                let tile_entity = base_command.id();

                tiles.insert(coordinates, tile_entity);

                base_command.insert(Name::new(format!(
                    "Tile: Base ({}, {}, {:?})",
                    x, y, tile_entity
                )));

                if safe_start_entity.is_none() && *tile == Tile::Empty {
                    *safe_start_entity = Some(tile_entity);
                }

                // spawn tile cover
                cmd.with_children(|parent| {
                    let entity = parent
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(-1., 1., 3.),
                            texture: board_assets.covered_tile_material.texture.clone(),
                            visibility: Visibility::Visible,
                            ..Default::default()
                        })
                        .insert(Name::new("Tile: Cover"))
                        .insert(TileCover)
                        .id();

                    if safe_start_entity.is_none() && *tile == Tile::Empty {
                        *safe_start_entity = Some(entity);
                    }
                });

                // spawn tile face
                match tile {
                    Tile::Bomb => {
                        cmd.insert(Bomb);
                        cmd.with_children(|parent| {
                            parent
                                .spawn(SpriteBundle {
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::splat(size - padding)),
                                        ..default()
                                    },
                                    transform: Transform::from_xyz(0., 0., 1.),
                                    texture: board_assets.bomb_material.texture.clone(),
                                    ..default()
                                })
                                .insert(Name::new("Tile: Bomb face"));
                        });
                    }
                    Tile::BombNeighbor(count) => {
                        cmd.insert(BombNeighbor { count: *count });
                        cmd.with_children(|parent| {
                            parent
                                .spawn(Self::bomb_count_text_bundle(
                                    *count,
                                    board_assets,
                                ))
                                .insert(Name::new("Tile: Neighbor face"));
                        });
                    }
                    _ => (),
                }
            }
        }
    }

    fn cleanup_board(board: Res<Board>, mut commands: Commands) {
        info!("Cleaning");
        commands.entity(board.entity).despawn_recursive();
        commands.remove_resource::<Board>();
    }
}
