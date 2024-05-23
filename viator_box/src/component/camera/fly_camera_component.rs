use bevy::prelude::*;

#[derive(Component)]
pub struct FlyCameraComponent {
    pub speed: f32,
    pub sensitivity: f32,
    pub pitch: f32,
    pub yaw: f32,
}
