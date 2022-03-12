use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const CELL_STARTING_RADIUS: f32 = 20.0;
const CELL_BORDER_THICKNESS: f32 = 2.0;
const CELL_FILL_COLOR: Color = Color::CYAN;
const CELL_BORDER_COLOR: Color = Color::BLACK;

fn spawn_cell(mut commands: Commands) {
    let circle = shapes::Circle {
        radius: CELL_STARTING_RADIUS,
        center: Vec2::new(0.0, 0.0),
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(GeometryBuilder::build_as(
        &circle,
        DrawMode::Outlined {
            fill_mode: FillMode::color(CELL_FILL_COLOR),
            outline_mode: StrokeMode::new(CELL_BORDER_COLOR, CELL_BORDER_THICKNESS),
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

pub struct CellPlugin;

impl Plugin for CellPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_cell);
    }
}
