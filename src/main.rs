mod tiled;

use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use tiled::{TiledMapPlugin, TiledMapBundle};


fn setup_graphics(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let tiled_map = server.load("tiled/test-1-export.tmx");

    commands.spawn(TiledMapBundle {
        tiled_map,
        ..default()
    });
}


fn main() {

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(TilemapPlugin)
        .add_plugins(TiledMapPlugin)
        .add_systems(Startup, setup_graphics)        
        .run();
}
