mod bounds;
pub mod components;
pub mod resources;
mod systems;

use bevy::log;
use bevy::prelude::*;

use resources::board::Board;
use resources::tile::Tile;
use resources::tile_map::TileMap;
use resources::BoardOptions;
use resources::BoardPosition;
use resources::TileSize;

use bounds::Bounds2;
use components::BombNeighbor;
use components::Coordinates;
// use components::Uncover;
use components::Bomb;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board);
        app.add_system(systems::input::handle_input);
        log::info!("Loaded Board Plugin");

        #[cfg(feature = "debug")]
        {
            app.register_type::<BombNeighbor>();
            app.register_type::<Bomb>();
            app.register_type::<BoardOptions>();
            // app.register_type::<Board>();
            // app.register_type::<Uncover>();
        }
    }
}

impl BoardPlugin {
    /// System to generate the complete board
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        // window: Res<WindowDescriptor>
        window: Query<&Window>,
        asset_server: Res<AssetServer>,
    ) {
        let font: Handle<Font> = asset_server.load("fonts/pixeled.ttf");
        let bomb_sprite: Handle<Image> = asset_server.load("sprites/bomb.png");

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

        commands
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
                            color: Color::WHITE,
                            custom_size: Some(board_size),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));

                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    Color::GRAY,
                    bomb_sprite,
                    font,
                )
            });

        commands.insert_resource(Board {
            tile_map,
            bounds: Bounds2 {
                position: Vec2::new(board_position.x, board_position.y),
                size: board_size,
            },
            tile_size,
        });
    }

    fn adaptive_tile_size(
        windows: Query<&Window>,
        (min, max): (f32, f32),
        (width, height): (u16, u16),
    ) -> f32 {
        for window in &windows {
            // TODO: fix this
            let max_width = window.resolution.width() / width as f32;
            let max_heigth = window.resolution.height() / height as f32;
            return max_width.min(max_heigth).clamp(min, max);
        }

        return 0.;
    }

    fn bomb_count_text_bundle(count: u8, font: Handle<Font>, font_size: f32) -> Text2dBundle {
        let (text, color) = (
            count.to_string(),
            match count {
                1 => Color::WHITE,
                2 => Color::GREEN,
                3 => Color::YELLOW,
                4 => Color::ORANGE,
                _ => Color::PURPLE,
            },
        );

        let text_style = TextStyle {
            color,
            font,
            font_size,
        };

        Text2dBundle {
            text: Text::from_section(text, text_style.clone())
                .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(0., 2.5, 1.),
            ..default()
        }
    }

    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        size: f32,
        padding: f32,
        color: Color,
        bomb_image: Handle<Image>,
        font: Handle<Font>,
    ) {
        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let coordinates = Coordinates {
                    x: x as u16,
                    y: y as u16,
                };

                let mut cmd = parent.spawn_empty();
                // parent.spawn_empty

                cmd.insert(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::splat(size - padding as f32)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32 * size) + (size / 2.),
                        (y as f32 * size) + (size / 2.),
                        1.,
                    ),
                    ..Default::default()
                })
                .insert(Name::new(format!("Tile ({}, {})", x, y)))
                // We add the `Coordinates` component to our tile entity
                .insert(coordinates);

                match tile {
                    Tile::Bomb => {
                        cmd.insert(Bomb);
                        cmd.with_children(|parent| {
                            parent.spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(size - padding as f32)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0., 0., 1.),
                                texture: bomb_image.clone(),
                                ..default()
                            });
                        });
                    }
                    Tile::BombNeighbor(count) => {
                        cmd.insert(BombNeighbor { count: *count });
                        cmd.with_children(|parent| {
                            parent.spawn(Self::bomb_count_text_bundle(
                                *count,
                                font.clone(),
                                size - padding,
                            ));
                        });
                    }
                    _ => (),
                }
            }
        }
    }
}
