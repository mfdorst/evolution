use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use rand::Rng;

// Brain model
const LAYER_SIZE: usize = 5;
const LAYER_COUNT: usize = 3;

// Brain visualization
const NEURON_RADIUS: f32 = 10.0;
const NEURON_SPACING: f32 = 40.0;
const NEURON_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const NEURON_BORDER_COLOR: Color = Color::BLACK;
const NEURON_BORDER_WIDTH: f32 = 2.0;
const X_POSITION: f32 = 500.0;
const Y_POSITION: f32 = -200.0;
const BACKDROP_MARGIN: f32 = 30.0;
const BACKDROP_COLOR: Color = Color::GRAY;
const BACKDROP_BORDER_COLOR: Color = Color::BLACK;
const BACKDROP_BORDER_WIDTH: f32 = 2.0;

#[derive(Component)]
struct Brain {
    synapses: Vec<Synapse>,
}

struct Synapse {
    weight: f32,
    bias: f32,
}

impl Brain {
    fn new() -> Self {
        let num_synapses = LAYER_SIZE * LAYER_SIZE * (LAYER_COUNT - 1);
        let mut synapses = Vec::with_capacity(num_synapses);
        let mut rng = rand::thread_rng();
        for _ in 0..num_synapses {
            synapses.push(Synapse {
                weight: rng.gen_range(-1.0..1.0),
                bias: rng.gen_range(-1.0..1.0),
            });
        }
        Brain { synapses }
    }

    fn compute_network(&self, input: Vec<f32>) -> Vec<Vec<f32>> {
        let synapses_per_layer = LAYER_SIZE * LAYER_SIZE;
        let mut layers = vec![input];
        let mut computed_layer = vec![0.0; LAYER_SIZE];

        for layer in 0..LAYER_COUNT - 1 {
            // Use the last computed layer as the input
            let input = &layers[layers.len() - 1];
            // Compute the next layer
            for (i, neuron) in input.iter().enumerate() {
                let start_index = layer * synapses_per_layer + i * LAYER_SIZE;
                let end_index = layer * synapses_per_layer + (i + 1) * LAYER_SIZE;
                for synapse in &self.synapses[start_index..end_index] {
                    computed_layer[i] += neuron * synapse.weight + synapse.bias;
                }
            }
            // Clamp the next layer to [-1.0, 1.0]
            for neuron in computed_layer.iter_mut() {
                *neuron = sigmoid(*neuron);
            }
            layers.push(computed_layer.clone());
        }
        layers
    }
}

/// Clamps the input to between -1.0 and 1.0 with the function x/sqrt(1+x^2).
fn sigmoid(x: f32) -> f32 {
    x / (1.0 + x * x).sqrt()
}

#[derive(Component)]
struct BrainVisual {
    neurons: Vec<Entity>,
}

impl BrainVisual {
    fn new() -> Self {
        Self {
            neurons: Vec::with_capacity(LAYER_SIZE * LAYER_COUNT),
        }
    }
}

#[derive(Component)]
struct Neuron;

fn spawn_visual(mut commands: Commands) {
    // Backdrop
    let backdrop_width = BACKDROP_MARGIN + LAYER_COUNT as f32 * NEURON_SPACING;
    let backdrop_height = BACKDROP_MARGIN + LAYER_SIZE as f32 * NEURON_SPACING;
    let backdrop = shapes::Rectangle {
        extents: Vec2::new(backdrop_width, backdrop_height),
        origin: RectangleOrigin::Center,
    };
    let mut brain_visual = BrainVisual::new();
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &backdrop,
            DrawMode::Outlined {
                fill_mode: FillMode::color(BACKDROP_COLOR),
                outline_mode: StrokeMode::new(BACKDROP_BORDER_COLOR, BACKDROP_BORDER_WIDTH),
            },
            Transform::from_xyz(X_POSITION, Y_POSITION, 0.5),
        ))
        .with_children(|parent| {
            // Neurons
            let brain_width = LAYER_COUNT as f32 * NEURON_SPACING;
            let brain_height = LAYER_SIZE as f32 * NEURON_SPACING;
            let brain_start_x = (BACKDROP_MARGIN + NEURON_RADIUS - brain_width) / 2.0;
            let brain_start_y = (BACKDROP_MARGIN + NEURON_RADIUS - brain_height) / 2.0;
            for x in 0..LAYER_COUNT {
                for y in 0..LAYER_SIZE {
                    let neuron = shapes::Circle {
                        radius: NEURON_RADIUS,
                        center: Vec2::new(0.0, 0.0),
                    };
                    let entity = parent
                        .spawn_bundle(GeometryBuilder::build_as(
                            &neuron,
                            DrawMode::Outlined {
                                fill_mode: FillMode::color(NEURON_COLOR),
                                outline_mode: StrokeMode::new(
                                    NEURON_BORDER_COLOR,
                                    NEURON_BORDER_WIDTH,
                                ),
                            },
                            Transform::from_xyz(
                                brain_start_x + x as f32 * NEURON_SPACING,
                                brain_start_y + y as f32 * NEURON_SPACING,
                                1.0,
                            ),
                        ))
                        .insert(Neuron)
                        .id();
                    brain_visual.neurons.push(entity);
                }
            }
        })
        .insert(Brain::new())
        .insert(brain_visual);
}

fn update_network(
    mut brain_query: Query<(&Brain, &mut BrainVisual)>,
    mut neuron_query: Query<(&mut DrawMode, Entity), With<Neuron>>,
) {
    for (brain, brain_visual) in brain_query.iter_mut() {
        let network_input = vec![0.5; LAYER_SIZE];
        let neuron_values = brain.compute_network(network_input);
        for (&neuron_value, entity) in neuron_values
            .iter()
            .flatten()
            .zip(brain_visual.neurons.clone())
        {
            match neuron_query.get_mut(entity).map(|(dm, _)| dm.into_inner()) {
                Ok(DrawMode::Outlined {
                    fill_mode,
                    outline_mode: _,
                }) => {
                    fill_mode.color = Color::rgb(neuron_value, neuron_value, neuron_value);
                }
                Ok(_) => {
                    println!("Expected draw mode to be Outlined")
                }
                Err(e) => {
                    println!("Query error: {e}");
                }
            }
        }
    }
}

pub struct BrainPlugin;

impl Plugin for BrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_visual)
            .add_system(update_network);
    }
}
