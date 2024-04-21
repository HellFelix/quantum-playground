#![allow(non_snake_case)]

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

use super::{iteration::rk4_iter_dt, wave, DT};
use crate::complex::Complex;

// creates bevy application and initiates simulation for one dimension
pub fn oneD() {
    App::new()
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin))
        // setup data
        .add_systems(Startup, setup)
        // draw wave every frame
        .add_systems(Update, draw_wave_function)
        // update parameters and options after each frame
        .add_systems(
            PostUpdate,
            (update_wave_function, update_params, update_options),
        )
        .run();
}

// struct contraining all of the information of the actual simulation
// i.e. holds all the information about the wave packet.
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

// holds informaiton about the buttons and which charts should be active
#[derive(Component)]
struct ToggleButton {
    variant: ToggleVariant,
    active: bool,
    color: BackgroundColor,
}
impl ToggleButton {
    pub fn variant(&self) -> ToggleVariant {
        self.variant
    }
    pub fn active(&self) -> bool {
        self.active
    }
    pub fn toggle(&mut self) {
        self.active = !self.active;
    }
    pub fn color(&self) -> BackgroundColor {
        self.color
    }
}
// enum used to differentiate between the toggle buttons used in ToggleButton
#[derive(Clone, Copy)]
enum ToggleVariant {
    Real,
    Imag,
}

fn setup(mut commands: Commands) {
    // camera settings
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.01;
    commands.spawn(camera);

    // initial wave packet
    let wave = wave();
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

    // Very basic UI to show relevant information and act as a functional interface
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
            // Timer
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

            // Frame rate viewer
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

            // Speed viewer
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
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        padding: UiRect {
                            top: Val::Vh(5.),
                            right: Val::Px(5.),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Options: ",
                            TextStyle {
                                font_size: 25.,
                                ..default()
                            },
                        ),
                        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                    ));
                });

            // Show real chart button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(65.),
                            height: Val::Vw(3.),
                            border: UiRect::all(Val::Px(5.0)),
                            margin: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,

                            ..default()
                        },
                        border_color: Color::RED.into(),
                        background_color: Color::RED.into(),
                        ..default()
                    },
                    ToggleButton {
                        variant: ToggleVariant::Real,
                        active: true,
                        color: Color::RED.into(),
                    },
                    AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Real",
                        TextStyle {
                            font_size: 25.,
                            ..default()
                        },
                    ));
                });

            // Show imaginary chart button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(65.),
                            height: Val::Vw(3.),
                            border: UiRect::all(Val::Px(5.0)),
                            margin: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,

                            ..default()
                        },
                        border_color: Color::BLUE.into(),
                        background_color: Color::BLUE.into(),
                        ..default()
                    },
                    ToggleButton {
                        variant: ToggleVariant::Imag,
                        active: true,
                        color: Color::BLUE.into(),
                    },
                    AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Imaginary",
                        TextStyle {
                            font_size: 25.,
                            ..default()
                        },
                    ));
                });
        });
}
fn draw_wave_function(
    mut gizmos: Gizmos,
    data: Query<&Data>,
    toggle_buttons_query: Query<&ToggleButton, With<Button>>,
) {
    let data = data.get_single().unwrap();

    for button in toggle_buttons_query.iter() {
        match button.variant() {
            ToggleVariant::Real => {
                if button.active() {
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
                }
            }
            ToggleVariant::Imag => {
                if button.active() {
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
                }
            }
        }
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
    // iterate
    let mut data = data.get_single_mut().unwrap();
    let mut next = data.raw.clone();
    // skips to the next time step i.e. data.speed
    // each iteration is still calculated, but the ones in between are not shown
    for _ in 0..data.speed {
        next = rk4_iter_dt(&next);
    }

    // calculate new values
    let next_prob = DVector::from(
        next.iter()
            .map(|x| x.abs_squared() as f32)
            .collect::<Vec<f32>>(),
    );
    data.raw = next;
    data.prob = next_prob;
    data.time_passed += DT * data.speed as f64;
}

fn update_params(
    mut data: Query<&mut Data>,
    mut key_evs: EventReader<KeyboardInput>,
    diagnostics: Res<DiagnosticsStore>,
    mut text_set: ParamSet<(
        Query<&mut Text, With<TimeText>>,
        Query<&mut Text, With<FrameText>>,
        Query<&mut Text, With<SpeedText>>,
    )>,
) {
    let mut data = data.get_single_mut().unwrap();

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

    // update speed
    for e in key_evs.read() {
        if e.key_code == KeyCode::ArrowUp && data.speed < 200 {
            data.speed += 1;
        } else if e.key_code == KeyCode::ArrowDown && data.speed > 1 {
            data.speed -= 1;
        }
    }

    for mut speed_text in &mut text_set.p2() {
        speed_text.sections[0].value = format!("Speed: {}", data.speed);
    }
}

fn update_options(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut ToggleButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut toggle_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // change the state of the chart in question
                toggle_button.toggle();

                // change the colour of the button to match its state
                if toggle_button.active() {
                    *color = toggle_button.color();
                } else {
                    *color = Color::BLACK.into();
                }
            }
            _ => {}
        };
    }
}
