use bevy::{prelude::*, window::WindowResolution};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board_plugin::{resources::BoardOptions, AppState, BoardPlugin};

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

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    app.add_startup_system(camera_setup);
    app.add_plugin(BoardPlugin {
        state: AppState::InGame,
    });

    app.insert_resource(BoardOptions {
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

    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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
