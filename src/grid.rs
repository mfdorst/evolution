use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const GRID_HEIGHT: i32 = 40;
const GRID_WIDTH: i32 = 60;
const GRID_CELL_SIZE: f32 = 40.0;
const GRID_CELL_BORDER_THICKNESS: f32 = 1.0;
const GRID_CELL_FILL_COLOR: Color = Color::GRAY;
const GRID_CELL_BORDER_COLOR: Color = Color::DARK_GRAY;

fn spawn_grid(mut commands: Commands) {
    let square = shapes::Rectangle {
        extents: Vec2::splat(GRID_CELL_SIZE),
        origin: RectangleOrigin::Center,
    };
    for x in -GRID_WIDTH / 2..GRID_WIDTH / 2 {
        for y in -GRID_HEIGHT / 2..GRID_HEIGHT / 2 {
            commands.spawn_bundle(GeometryBuilder::build_as(
                &square,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(GRID_CELL_FILL_COLOR),
                    outline_mode: StrokeMode::new(
                        GRID_CELL_BORDER_COLOR,
                        GRID_CELL_BORDER_THICKNESS,
                    ),
                },
                Transform::from_xyz(x as f32 * GRID_CELL_SIZE, y as f32 * GRID_CELL_SIZE, 0.0),
            ));
        }
    }
}

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_grid);
    }
}
