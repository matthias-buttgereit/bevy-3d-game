use bevy::prelude::*;

#[derive(Component)]
pub struct Minion {
    pub life: u16,
    pub attack_damage: u16,
}

#[derive(Component)]
pub struct Targeting(pub Option<Entity>);

#[derive(Component)]
pub struct Targetable;

#[derive(Component)]
pub struct Movable(pub f32);

#[derive(Component)]
pub struct Moving;

#[derive(Component)]
pub struct PreviewFigur;

#[derive(Component)]
pub struct CameraController {
    pub direction: Vec3,
    pub position: Vec3,
    pub speed: f32,
    pub looking_at: Vec3,
}

#[derive(Component)]
pub struct Figure {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}
