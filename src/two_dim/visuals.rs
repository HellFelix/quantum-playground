#![allow(non_snake_case)]

use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::keyboard::KeyboardInput,
    prelude::*,
};
use nalgebra::DMatrix;

use super::L;

#[derive(Component)]
struct Data {
    values: DMatrix<f32>,
}

#[derive(Component)]
struct XCoordText;
#[derive(Component)]
struct YCoordText;
#[derive(Component)]
struct ZCoordText;
#[derive(Component)]
struct RotationText;
#[derive(Component)]
struct FOVTExt;

pub fn twoD() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, (controls, update_text))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform {
            translation: Vec3::new(6.6, 1.6, 0.),
            rotation: Quat::from_xyzw(-0.1, 0.7, 0.1, 0.7),
            ..default()
        },
        projection: Projection::Perspective(PerspectiveProjection {
            fov: 0.75,
            ..default()
        }),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(L as f32, L as f32)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });

    // info text
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            info_text(parent, RotationText);
            info_text(parent, FOVTExt);
            info_text(parent, XCoordText);
            info_text(parent, YCoordText);
            info_text(parent, ZCoordText);
        });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0., 8.0, 0.),
        ..default()
    });
}

fn info_text(parent: &mut ChildBuilder, identifier: impl Component) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect {
                        top: Val::Px(10.),
                        left: Val::Px(10.),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 25.,
                        ..default()
                    },
                ),
                identifier,
            ));
        });
}

fn controls(
    mut key_evs: EventReader<KeyboardInput>,
    mut projection_query: Query<&mut Projection, With<Camera3d>>,
    mut transform_query: Query<&mut Transform, With<Camera3d>>,
) {
    let Projection::Perspective(persp) = projection_query.single_mut().into_inner() else {
        panic!("Failed to find perspective. Camera perspective not found");
    };

    let mut transform = transform_query.get_single_mut().unwrap();

    for e in key_evs.read() {
        match e.key_code {
            // Swedish keyboard, this is plus :)
            KeyCode::Minus => {
                persp.fov /= 1.1;
            }
            // Swedish keyboard, this is minus
            KeyCode::Slash => {
                persp.fov *= 1.1;
            }

            // Move camera in the plane
            KeyCode::ArrowUp => {
                transform.translation.x -= 0.1;
            }
            KeyCode::ArrowDown => {
                transform.translation.x += 0.1;
            }
            KeyCode::ArrowLeft => {
                transform.translation.z += 0.1;
            }
            KeyCode::ArrowRight => {
                transform.translation.z -= 0.1;
            }

            // Move camera up and down
            KeyCode::ShiftLeft => {
                transform.translation.y -= 0.1;
            }
            KeyCode::Space => {
                transform.translation.y += 0.1;
            }

            // rotate camera up and down
            KeyCode::KeyU => {
                transform.rotate_z(-0.01);
            }
            KeyCode::KeyD => {
                transform.rotate_z(0.01);
            }
            _ => {}
        }
    }
}

fn update_text(
    mut projection_query: Query<&mut Projection, With<Camera3d>>,
    transform_query: Query<&Transform, With<Camera3d>>,
    mut text_set: ParamSet<(
        Query<&mut Text, With<XCoordText>>,
        Query<&mut Text, With<YCoordText>>,
        Query<&mut Text, With<ZCoordText>>,
        Query<&mut Text, With<RotationText>>,
        Query<&mut Text, With<FOVTExt>>,
    )>,
) {
    let Projection::Perspective(persp) = projection_query.single_mut().into_inner() else {
        panic!("Failed to find perspective. Camera perspective not found");
    };
    let transform = transform_query.get_single().unwrap();

    for mut rotation_text in &mut text_set.p3() {
        rotation_text.sections[0].value = format!("Rotation: {}", transform.rotation);
    }
    for mut fov_text in &mut text_set.p4() {
        fov_text.sections[0].value = format!("FOV: {}", persp.fov);
    }
    for mut x_coord_text in &mut text_set.p0() {
        x_coord_text.sections[0].value = format!("X: {}", transform.translation.x);
    }
    for mut y_coord_text in &mut text_set.p1() {
        y_coord_text.sections[0].value = format!("Y: {}", transform.translation.y);
    }
    for mut z_coord_text in &mut text_set.p2() {
        z_coord_text.sections[0].value = format!("Z: {}", transform.translation.z);
    }
}
