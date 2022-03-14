use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod brain;
mod cell;
mod grid;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(cell::CellPlugin)
        .add_plugin(grid::GridPlugin)
        .add_plugin(brain::BrainPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
