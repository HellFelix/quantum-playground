#![allow(non_snake_case)]

use std::time::SystemTime;

use bevy::prelude::*;
use nalgebra::DVector;

use crate::{complex::Complex, iteration::rk4_iter_dt, DT};

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
    raw: DVector<Complex>,
    prob: DVector<f32>,
    x: DVector<f32>,
    time_passed: f64,
}
fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.01;
    commands.spawn(camera);

    let wave = crate::wave();
    let x = DVector::from(wave.0.iter().map(|x| *x as f32).collect::<Vec<f32>>());
    let raw = wave.1.clone();
    let prob = DVector::from(
        wave.1
            .iter()
            .map(|x| x.abs_squared() as f32)
            .collect::<Vec<f32>>(),
    );

    let data = Data {
        raw,
        prob,
        x,
        time_passed: 0.,
    };
    commands.spawn(data);
}
fn draw_wave_function(mut gizmos: Gizmos, data: Query<&Data>) {
    let data = data.get_single().unwrap();
    for i in 0..data.x.len() - 1 {
        gizmos.line_2d(
            Vec2 {
                x: data.x[i],
                y: data.raw[i].real() as f32,
            },
            Vec2 {
                x: data.x[i + 1],
                y: data.raw[i + 1].real() as f32,
            },
            Color::RED,
        );
    }

    for i in 0..data.x.len() - 1 {
        gizmos.line_2d(
            Vec2 {
                x: data.x[i],
                y: data.raw[i].imag() as f32,
            },
            Vec2 {
                x: data.x[i + 1],
                y: data.raw[i + 1].imag() as f32,
            },
            Color::BLUE,
        );
    }

    for i in 0..data.x.len() - 1 {
        gizmos.line_2d(
            Vec2 {
                x: data.x[i],
                y: data.prob[i],
            },
            Vec2 {
                x: data.x[i + 1],
                y: data.prob[i + 1],
            },
            Color::GREEN,
        );
    }
}

fn update_wave_function(mut data: Query<&mut Data>) {
    let mut data = data.get_single_mut().unwrap();

    let mut next = data.raw.clone();
    let start = SystemTime::now();
    for _ in 0..20 {
        next = rk4_iter_dt(&next);
    }
    info!("iteration took {}us", start.elapsed().unwrap().as_micros());

    // calculate new values
    let next_prob = DVector::from(
        next.iter()
            .map(|x| x.abs_squared() as f32)
            .collect::<Vec<f32>>(),
    );
    data.raw = next;
    data.prob = next_prob;
    data.time_passed += DT;
}
