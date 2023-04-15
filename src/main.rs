use bevy::{prelude::*, window::WindowResolution};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board_plugin::{
    resources::{BoardAssets, BoardOptions, SpriteMaterial},
    AppState, BoardPlugin,
};

fn main() {
    let mut app = App::new();

    let window_resolution = WindowResolution::new(700., 800.);

    app.add_state::<AppState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mine Sweeper".to_string(),
                resolution: window_resolution,
                window_level: bevy::window::WindowLevel::AlwaysOnTop,
                ..default()
            }),
            ..default()
        }))
        .add_system(state_handler);

    app.add_startup_systems((camera_setup, board_setup));

    app.add_plugin(BoardPlugin {
        state: AppState::Out,
    });

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn board_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(BoardOptions {
        map_size: (16, 16),
        tile_size: board_plugin::resources::TileSize::Adaptive {
            min: 5.0,
            max: 25.0,
        },
        bomb_count: 32,
        safe_start: true,
        tile_padding: 3.0,
        ..default()
    });

    commands.insert_resource(BoardAssets {
        label: String::from("Default"),
        board_material: SpriteMaterial {
            color: Color::ANTIQUE_WHITE,
            ..default()
        },
        tile_material: SpriteMaterial {
            color: Color::DARK_GRAY,
            ..default()
        },
        covered_tile_material: SpriteMaterial {
            color: Color::GRAY,
            ..default()
        },
        bomb_counter_font: asset_server.load("fonts/pixeled.ttf"),
        bomb_counter_colors: BoardAssets::default_colors(),
        flag_material: SpriteMaterial {
            color: Color::WHITE,
            texture: asset_server.load("sprites/flag.png"),
        },
        bomb_material: SpriteMaterial {
            color: Color::WHITE,
            texture: asset_server.load("sprites/bomb.png"),
        },
    })
}

fn state_handler(
    mut next_state: ResMut<NextState<AppState>>,
    state: ResMut<State<AppState>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::C) {
        debug!("clearing detected");
        if state.0 == AppState::InGame {
            info!("clearing game");
            next_state.set(AppState::Out);
        }
    }

    if keys.just_pressed(KeyCode::G) {
        debug!("loading detected");
        if state.0 == AppState::Out {
            info!("loading game");
            next_state.set(AppState::InGame);
        }
    }
}
