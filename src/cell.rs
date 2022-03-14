use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const CELL_STARTING_RADIUS: f32 = 10.0;
const CELL_BORDER_THICKNESS: f32 = 2.0;
const CELL_FILL_COLOR: Color = Color::BLACK;
const CELL_BORDER_COLOR: Color = Color::DARK_GRAY;

#[derive(Component)]
struct Mote;

fn spawn_cell(mut commands: Commands) {
    let circle = shapes::Circle {
        radius: CELL_STARTING_RADIUS,
        center: Vec2::new(0.0, 0.0),
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &circle,
            DrawMode::Outlined {
                fill_mode: FillMode::color(CELL_FILL_COLOR),
                outline_mode: StrokeMode::new(CELL_BORDER_COLOR, CELL_BORDER_THICKNESS),
            },
            Transform::from_xyz(0.0, 0.0, 1.0),
        ))
        .insert(Mote);
}

fn update_cell_color(
    mut commands: Commands,
    mut query: Query<&mut bevy_prototype_lyon::draw::DrawMode, With<Mote>>,
) {
    for draw_mode in query.iter_mut() {
        match draw_mode.into_inner() {
            DrawMode::Outlined {
                fill_mode,
                outline_mode: _,
            } => {
                let color = fill_mode.color;
                let lighter = Color::rgb(color.r() + 0.01, color.g() + 0.01, color.b() + 0.01);
                fill_mode.color = lighter;
            }
            _ => {}
        }
    }
}

pub struct CellPlugin;

impl Plugin for CellPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_cell)
            .add_system(update_cell_color);
    }
}
