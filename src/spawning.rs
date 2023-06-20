use bevy::{prelude::*, window::PrimaryWindow};

use crate::{components::*, get_ray_into_world, loading::SceneAssets, GameState, MOVING_SPEED};

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SpawningState>()
            .add_system(
                show_spawn_preview
                    .run_if(in_state(SpawningState::Spawning))
                    .run_if(in_state(GameState::GameStart)),
            )
            .add_system(
                actuallyspawn_preview
                    .run_if(in_state(SpawningState::Spawning))
                    .run_if(in_state(GameState::GameStart)),
            )
            .add_system(
                switch_spawning_state
                    .after(show_spawn_preview)
                    .run_if(in_state(GameState::GameStart)),
            );
    }
}

pub fn switch_spawning_state(
    mut commands: Commands,
    key_input: Res<Input<KeyCode>>,
    state: Res<State<SpawningState>>,
    mut next_state: ResMut<NextState<SpawningState>>,
    preview: Query<Entity, With<PreviewFigur>>,
    scene_assets: Res<SceneAssets>,
) {
    let scene;

    if key_input.just_pressed(KeyCode::Key1) {
        scene = Some(scene_assets.wizard.clone());
    } else if key_input.just_pressed(KeyCode::Key2) {
        scene = Some(scene_assets.old.clone());
    } else if key_input.just_pressed(KeyCode::Key3) {
        scene = Some(scene_assets.young.clone());
    } else if key_input.just_pressed(KeyCode::Key4) {
        scene = Some(scene_assets.hat.clone());
    } else {
        scene = None;
    }

    if let Some(scene) = scene {
        match state.0 {
            SpawningState::Spawning => {
                for entity in preview.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                next_state.set(SpawningState::None);
            }
            SpawningState::None => {
                spawn(commands, scene);
                next_state.set(SpawningState::Spawning);
            }
        }
    }
}

fn spawn(mut commands: Commands, scene: Handle<Scene>) {
    commands.spawn((
        SceneBundle { scene, ..default() },
        PreviewFigur,
        Figure {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::from_rotation_y(0.0),
            scale: Vec3::ONE,
        },
    ));
}

pub fn show_spawn_preview(
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &Transform), With<Camera3d>>,
    mut preview: Query<&mut Figure, (With<PreviewFigur>, Without<Camera3d>)>,
) {
    let ray = get_ray_into_world(window.single(), camera).unwrap();
    let distance = ray.intersect_plane(Vec3::ZERO, Vec3::Y).unwrap();
    let point = ray.get_point(distance);

    if let Ok(mut preview) = preview.get_single_mut() {
        preview.position.x = point.x;
        preview.position.z = point.z;
    };
}

pub fn actuallyspawn_preview(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    mut preview: Query<Entity, With<PreviewFigur>>,
    mut next_state: ResMut<NextState<SpawningState>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Ok(preview) = preview.get_single_mut() {
            commands
                .entity(preview)
                .remove::<PreviewFigur>()
                .insert(Targetable)
                .insert(Targeting(None))
                .insert(Movable(MOVING_SPEED))
                .insert(Minion {
                    life: 100,
                    attack_damage: 15,
                })
                .insert(Targeting(None))
                .insert(Targetable);
            next_state.set(SpawningState::None);
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum SpawningState {
    Spawning,
    #[default]
    None,
}
