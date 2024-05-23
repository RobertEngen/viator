use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::component::camera::fly_camera_component::FlyCameraComponent;

pub fn fly_camera_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button_events: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut FlyCameraComponent), With<Camera>>,
) {
    let delta_time = time.delta_seconds();
    let right_button_pressed = mouse_button_events.pressed(MouseButton::Right);

    for (mut transform, mut camera_fly_component) in query.iter_mut() {
        let camera_speed = camera_fly_component.speed;
        let sensitivity = camera_fly_component.sensitivity;

        let mut delta_x = 0.0;
        let mut delta_y = 0.0;
        if right_button_pressed {
            for event in mouse_motion_events.read() {
                delta_x -= event.delta.x;
                delta_y -= event.delta.y;
            }

            delta_x *= sensitivity;
            delta_y *= sensitivity;

            camera_fly_component.pitch = (camera_fly_component.pitch + delta_y).clamp(-89.0, 89.0);
            camera_fly_component.yaw += delta_x;
        }

        if right_button_pressed {
            transform.rotation =
                Quat::from_axis_angle(Vec3::Y, camera_fly_component.yaw.to_radians())
                    * Quat::from_axis_angle(Vec3::X, camera_fly_component.pitch.to_radians());
        }

        let mut movement_vector = Vec3::ZERO;
        let forward = transform.forward().xyz();
        let right = transform.right().xyz();

        if keyboard_input.pressed(KeyCode::KeyW) {
            movement_vector += forward;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            movement_vector -= forward;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            movement_vector -= right;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            movement_vector += right;
        }

        if movement_vector.length() > 0.0 {
            transform.translation += movement_vector.normalize() * camera_speed * delta_time;
        }
    }
}
