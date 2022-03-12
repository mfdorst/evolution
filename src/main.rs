use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const GRID_HEIGHT: i32 = 80;
const GRID_WIDTH: i32 = 120;
const GRID_CELL_SIZE: f32 = 10.0;
const GRID_CELL_BORDER_THICKNESS: f32 = 1.0;
const GRID_CELL_FILL_COLOR: Color = Color::GRAY;
const GRID_CELL_BORDER_COLOR: Color = Color::DARK_GRAY;
const CELL_STARTING_RADIUS: f32 = 20.0;
const CELL_BORDER_THICKNESS: f32 = 2.0;
const CELL_FILL_COLOR: Color = Color::CYAN;
const CELL_BORDER_COLOR: Color = Color::BLACK;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(spawn_cell)
        .add_startup_system(spawn_grid)
        .run();
}

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
                Transform::from_xyz((x * 10) as f32, (y * 10) as f32, 0.0),
            ));
        }
    }
}
