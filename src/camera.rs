use std::f32::consts::PI;

use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::{input::mouse::MouseWheel, prelude::*, window::PrimaryWindow};

use crate::components::CameraController;

pub const CAMERA_MOVE_SPEED: f32 = 5.0;
pub const CAMERA_SCROLL_SPEED: f32 = 0.8;
pub const CAMERA_Y_MIN: f32 = 2.0;
pub const CAMERA_Y_MAX: f32 = 5.0;

pub fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, -5.0).looking_at(Vec3::default(), Vec3::Y),
            ..default()
        })
        .insert(CameraController {
            direction: Vec3::new(0.0, 0.0, 0.0),
            position: Vec3::new(-2.0, 2.5, -5.0),
            speed: CAMERA_MOVE_SPEED,
            looking_at: Vec3::new(0.0, 0.0, 0.0),
        });
}

pub fn camera_keyboard(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();
    let mut direction = Vec3::default();

    if keyboard_input.pressed(KeyCode::A) {
        let left = camera.left();
        direction += left;
    }
    if keyboard_input.pressed(KeyCode::D) {
        let right = camera.right();
        direction += right;
    }
    if keyboard_input.pressed(KeyCode::W) {
        let forward = camera.forward() * Vec3::new(1.0, 0.0, 1.0);
        direction += forward;
    }
    if keyboard_input.pressed(KeyCode::S) {
        let back = camera.back() * Vec3::new(1.0, 0.0, 1.0);
        direction += back;
    }

    if direction.length() > 0.0 {
        camera.translation += direction.normalize() * CAMERA_MOVE_SPEED * time.delta_seconds();
    }
}

pub fn camera_mouse_sides(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let window = windows.single();

    if let Some(position) = window.cursor_position() {
        let mut direction = Vec3::default();
        let mut camera = camera_query.single_mut();

        if position.x == 0.0 {
            let left = camera.left();
            direction += left;
        }
        if position.x == window.width() - 1.0 {
            let right = camera.right();
            direction += right;
        }
        if position.y == 1.0 {
            let back = camera.back() * Vec3::new(1.0, 0.0, 1.0);
            direction += back;
        }
        if position.y == window.height() {
            let forward = camera.forward() * Vec3::new(1.0, 0.0, 1.0);
            direction += forward;
        }

        if direction.length() > 0.0 {
            camera.translation += direction.normalize() * CAMERA_MOVE_SPEED * time.delta_seconds();
        }
    };
}

pub fn camera_scrolling(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut scroll_event_reader: EventReader<MouseWheel>,
) {
    use bevy::input::mouse::MouseScrollUnit;

    for event in scroll_event_reader.iter() {
        match event.unit {
            MouseScrollUnit::Line => {
                let mut camera = camera_query.single_mut();

                let old_y = camera.translation.y;
                let new_y = old_y - event.y * CAMERA_SCROLL_SPEED;
                let new_y = new_y.clamp(CAMERA_Y_MIN, CAMERA_Y_MAX);
                camera.translation.y = new_y;

                let old_rotation = camera.rotation.x;
                let new_rotation = old_rotation * (new_y - old_y) / old_y;
                camera.rotate_local_x(new_rotation);
            }
            MouseScrollUnit::Pixel => {}
        }
    }
}

pub fn spawn_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 7.0,
            maximum_distance: 100.0,
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(0.0, 2.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 0.8,
    });
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_light)
            .add_startup_system(spawn_camera)
            .add_system(camera_keyboard)
            .add_system(camera_mouse_sides)
            .add_system(camera_scrolling);
    }
}
