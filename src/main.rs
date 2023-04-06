use bevy::{prelude::*, window::WindowResolution};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board_plugin::{BoardPlugin, resources::BoardOptions};

fn main() {
    let mut app = App::new();

    let window_resolution = WindowResolution::new(700., 800.);

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Mine Sweeper".to_string(),
            resolution: window_resolution,
            window_level: bevy::window::WindowLevel::AlwaysOnTop,
            ..default()
        }),
        ..default()
    }));

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    app.add_startup_system(camera_setup);
    app.add_plugin(BoardPlugin);

    app.insert_resource(BoardOptions {
        map_size: (16, 16),
        tile_size: board_plugin::resources::TileSize::Adaptive { min: 5.0, max: 25.0 },
        bomb_count: 42,
        tile_padding: 3.0,
        ..Default::default()
    });

    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
