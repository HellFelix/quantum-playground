#![allow(non_snake_case)]

use bevy::prelude::*;

pub fn oneD() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_example_collection)
        .run();
}

#[derive(Component)]
struct Data {
    raw: Vec<f32>,
    prob: Vec<f32>,
    x: Vec<f32>,
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.01;
    commands.spawn(camera);

    let wave = crate::wave_n(2);
    let x = wave.0.iter().map(|x| *x as f32).collect();
    let raw = wave.1.clone().iter().map(|x| *x as f32).collect();
    let prob = wave.1.iter().map(|x| (x * x) as f32).collect();

    let data = Data { raw, prob, x };
    commands.spawn(data);
}

fn draw_example_collection(mut gizmos: Gizmos, data: Query<&Data>) {
    let data = data.get_single().unwrap();
    for i in 0..data.x.len() - 1 {
        gizmos.line_2d(
            Vec2 {
                x: data.x[i] * 5.,
                y: data.prob[i],
            },
            Vec2 {
                x: data.x[i + 1] * 5.,
                y: data.prob[i + 1],
            },
            Color::RED,
        );
    }
}
