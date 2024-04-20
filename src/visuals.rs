#![allow(non_snake_case)]

use std::time::SystemTime;

use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::keyboard::KeyboardInput,
    prelude::*,
};
use nalgebra::DVector;

use crate::{complex::Complex, iteration::rk4_iter_dt, DT};

pub fn oneD() {
    App::new()
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, draw_wave_function)
        .add_systems(PostUpdate, (update_wave_function, update_speed))
        .run();
}

#[derive(Component)]
struct Data {
    raw: DVector<Complex>,
    prob: DVector<f32>,
    x: DVector<f32>,
    speed: usize,
    time_passed: f64,
}

#[derive(Component)]
struct TimeText;
#[derive(Component)]
struct FrameText;
#[derive(Component)]
struct SpeedText;

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
        speed: 1,
        time_passed: 0.,
    };
    commands.spawn(data);

    // text
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(15.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgb(0., 0., 0.).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Time:",
                    TextStyle {
                        font_size: 25.,
                        ..default()
                    },
                ),
                TimeText,
                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
            ));

            parent.spawn((
                TextBundle::from_section(
                    "FPS:",
                    TextStyle {
                        font_size: 25.,
                        ..default()
                    },
                ),
                FrameText,
                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
            ));

            parent.spawn((
                TextBundle::from_section(
                    "Speed: ",
                    TextStyle {
                        font_size: 25.,
                        ..default()
                    },
                ),
                SpeedText,
                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
            ));
        });
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

fn update_wave_function(
    mut data: Query<&mut Data>,
    diagnostics: Res<DiagnosticsStore>,
    mut text_set: ParamSet<(
        Query<&mut Text, With<TimeText>>,
        Query<&mut Text, With<FrameText>>,
    )>,
) {
    // iterate
    let mut data = data.get_single_mut().unwrap();
    let mut next = data.raw.clone();
    //let start = SystemTime::now();
    for _ in 0..data.speed {
        next = rk4_iter_dt(&next);
    }
    //info!("iteration took {}us", start.elapsed().unwrap().as_micros());

    // calculate new values
    let next_prob = DVector::from(
        next.iter()
            .map(|x| x.abs_squared() as f32)
            .collect::<Vec<f32>>(),
    );
    data.raw = next;
    data.prob = next_prob;
    data.time_passed += DT * data.speed as f64;

    // update timer
    for mut timer in &mut text_set.p0() {
        timer.sections[0].value = format!("Time [t.u.]:\n{}", data.time_passed as f32);
    }

    // update FPS
    for mut fps_text in &mut text_set.p1() {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                fps_text.sections[0].value = format!("FPS: {value:.2}");
            }
        }
    }
}

fn update_speed(
    mut data: Query<&mut Data>,
    mut key_evs: EventReader<KeyboardInput>,
    mut speed_query: Query<&mut Text, With<SpeedText>>,
) {
    let mut data = data.get_single_mut().unwrap();
    for e in key_evs.read() {
        if e.key_code == KeyCode::ArrowUp && data.speed < 200 {
            data.speed += 1;
        } else if e.key_code == KeyCode::ArrowDown && data.speed > 1 {
            data.speed -= 1;
        }
    }

    for mut speed_text in &mut speed_query {
        speed_text.sections[0].value = format!("Speed: {}", data.speed);
    }
}
