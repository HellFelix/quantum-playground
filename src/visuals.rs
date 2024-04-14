#![allow(non_snake_case)]

use std::{thread, time::Duration};

use bevy::prelude::*;

use crate::{complex::Complex, iterate_pde_rk4, DT};

pub fn oneD() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_wave_function)
        .add_systems(PostUpdate, update_wave_function)
        .run();
}

#[derive(Component)]
struct Data {
    raw: Vec<Complex>,
    prob: Vec<f32>,
    x: Vec<f32>,
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.01;
    commands.spawn(camera);

    let wave = crate::wave_n(2);
    let x = wave.0.iter().map(|x| *x as f32).collect();
    let raw = wave.1.clone();
    let prob = wave.1.iter().map(|x| x.abs_squared() as f32).collect();

    let data = Data { raw, prob, x };
    commands.spawn(data);
}

fn draw_wave_function(mut gizmos: Gizmos, data: Query<&Data>) {
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

fn update_wave_function(mut data: Query<&mut Data>) {
    thread::sleep(Duration::from_secs(1));
    let mut data = data.get_single_mut().unwrap();
    let next = iterate_pde_rk4(data.raw.clone(), DT);
    let next_prob = next.iter().map(|x| x.abs_squared() as f32).collect();
    data.raw = next;
    data.prob = next_prob;
}
