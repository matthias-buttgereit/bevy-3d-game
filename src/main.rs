mod camera;
mod components;
mod loading;
mod spawning;

// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::*;
use components::*;

use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use loading::{LoadingPlugin, SceneAssets, TextureAssets};
use spawning::SpawningPlugin;

// use rand::random;

pub const CUBES_AMOUNT: usize = 5;
pub const CUBE_SIZE: f32 = 1.0;
pub const PLANE_SIZE: f32 = 20.0;

pub const MOVING_SPEED: f32 = 0.2;
pub const STOPPING_DISTANCE: f32 = 1.0;
pub const SWITCHING_DELTA: f32 = 0.1;
pub const SEEING_DISTANCE: f32 = 5.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(CameraPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_state::<GameState>()
        .add_plugin(LoadingPlugin)
        .add_plugin(SpawningPlugin)
        .add_startup_systems((non_resizable, lock_cursor))
        .add_systems((spawn_plane, spawn_wizard).in_schedule(OnEnter(GameState::GameStart)))
        .add_system(bevy::window::close_on_esc)
        .add_systems((
            update_targets.run_if(in_state(GameState::GameStart)),
            face_target.run_if(in_state(GameState::GameStart)),
            update_figure_models.run_if(in_state(GameState::GameStart)),
        ))
        .add_system(handle_moving_component.run_if(in_state(GameState::GameStart)))
        .add_system(move_movables.run_if(in_state(GameState::GameStart)))
        .run();
}

// ---- SYSTEM SETUP ----

pub fn lock_cursor(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window.single_mut();
    window.cursor.grab_mode = CursorGrabMode::Confined;
}

pub fn non_resizable(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window.single_mut();
    window.resizable = false;
}

// ---- WORLD ----

pub fn spawn_plane(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_assets.ground.clone()),
        perceptual_roughness: 0.9,
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(PLANE_SIZE).into()),
        material: material_handle,
        ..default()
    });
}

// ---- ENTITIES ----

pub fn spawn_wizard(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    let figure = Figure {
        position: Vec3::new(2.0, 0.0, 1.0),
        rotation: Quat::from_rotation_y(0.0),
        scale: Vec3::ONE,
    };

    commands
        .spawn(SceneBundle {
            scene: scene_assets.wizard.clone(),
            transform: Transform::from_translation(figure.position)
                .mul_transform(Transform::from_rotation(figure.rotation)),
            ..default()
        })
        .insert(Minion {
            life: 100,
            attack_damage: 15,
        })
        .insert(Targeting(None))
        .insert(Targetable)
        .insert(Movable(MOVING_SPEED))
        .insert(figure);

    let figure = Figure {
        position: Vec3::new(-2.0, 0.0, 1.0),
        rotation: Quat::from_rotation_y(0.0),
        scale: Vec3::ONE,
    };

    commands
        .spawn(SceneBundle {
            scene: scene_assets.wizard.clone(), //scene_assets.wizard.clone(),
            transform: Transform::from_translation(figure.position)
                .mul_transform(Transform::from_rotation(figure.rotation)),
            ..default()
        })
        .insert(Minion {
            life: 100,
            attack_damage: 15,
        })
        .insert(Targeting(None))
        .insert(Targetable)
        .insert(Movable(MOVING_SPEED))
        .insert(Moving)
        .insert(figure);
}

// ---- ANIMATION ----

pub fn handle_animations() {}

// ---- UTILITY ----

fn get_ray_into_world(
    window: &Window,
    camera_transform: Query<(&Camera, &Transform), With<Camera3d>>,
) -> Option<Ray> {
    if let Some(position) = window.cursor_position() {
        let (camera, &transform) = camera_transform.single();
        let global = GlobalTransform::IDENTITY.mul_transform(transform);
        camera.viewport_to_world(&global, position)
    } else {
        None
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    GameStart,
    Running,
}

pub fn update_targets(
    targetable: Query<(Entity, &Figure), With<Targetable>>,
    mut targeting: Query<(Entity, &Figure, &mut Targeting)>,
    global_transform: Query<&GlobalTransform>,
) {
    for (me_entity, hunter, mut target) in targeting.iter_mut() {
        for (entity, targetable) in targetable.iter() {
            if me_entity != entity {
                if let Some(current_target) = target.0 {
                    let position = hunter.position;
                    let target_pos = global_transform.get(current_target).unwrap().translation();

                    let distance_to_current = position.distance(target_pos);
                    let distance_to_potential = targetable.position.distance(position);

                    if (distance_to_current - distance_to_potential) > SWITCHING_DELTA {
                        target.0 = Some(entity);
                    }
                } else {
                    target.0 = Some(entity);
                }
            }
        }
    }
}

pub fn face_target(
    mut minion_query: Query<(&mut Figure, &Targeting)>,
    global_transform: Query<&GlobalTransform>,
) {
    for (mut transform, target) in minion_query.iter_mut() {
        if let Some(target) = target.0 {
            let target_transform = global_transform.get(target).unwrap().translation();
            let direction = (target_transform - transform.position).normalize();

            transform.rotation = Quat::from_rotation_arc(Vec3::Z, direction)
        }
    }
}

pub fn handle_moving_component(
    mut commands: Commands,
    targeting: Query<(Entity, &Transform, &Targeting)>,
    global_transform: Query<&GlobalTransform>,
) {
    for (entity, transform, targeting) in targeting.iter() {
        if let Some(target) = targeting.0 {
            let target_position = global_transform.get(target).unwrap().translation();
            let distance = (target_position - transform.translation).length();

            if (STOPPING_DISTANCE..SEEING_DISTANCE).contains(&distance) {
                commands.entity(entity).insert(Moving);
            } else {
                commands.entity(entity).remove::<Moving>();
            }
        }
    }
}

pub fn update_figure_models(mut models: Query<(&mut Transform, &Figure)>) {
    for (mut transform, figure) in models.iter_mut() {
        *transform = Transform::from_translation(figure.position)
            .mul_transform(Transform::from_rotation(figure.rotation))
            .mul_transform(Transform::from_scale(figure.scale));
    }
}

pub fn move_movables(
    mut movable_query: Query<(&mut Figure, &Movable), With<Moving>>,
    time: Res<Time>,
) {
    for (mut figure, movable) in movable_query.iter_mut() {
        let speed = movable.0;
        let forward = figure.rotation * Vec3::Z;

        figure.position += forward * speed * time.delta_seconds();
    }
}
