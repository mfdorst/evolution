use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod cell;
mod grid;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(cell::CellPlugin)
        .add_plugin(grid::GridPlugin)
        .run();
}
