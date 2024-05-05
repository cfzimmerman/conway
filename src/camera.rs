use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};

const MOUSE_SENSITIVITY: f32 = 0.2;
const POSITION_INCR: f32 = 0.06;

#[derive(Component, Default)]
pub struct CameraRotation {
    yaw: f32,
    pitch: f32,
}

/// Handles keybindings and camera movement.
pub fn keyboard_motion(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera: Query<(&mut Transform, &CameraRotation)>,
    windows: Query<&Window>,
) {
    let mut window_focused = false;
    for window in windows.iter() {
        if window.focused {
            window_focused = true;
            break;
        }
    }
    if !window_focused {
        // Only apply movement keybindings if the window is actually focused.
        return;
    }

    // CameraRotation is just here as a marker for the Camera
    for (mut tform, _) in camera.iter_mut() {
        if keys.pressed(KeyCode::KeyW) {
            let mut fwd: Vec3 = tform.forward().into();
            fwd.y = 0.;
            tform.translation += fwd.normalize_or_zero() * POSITION_INCR;
        }
        if keys.pressed(KeyCode::KeyS) {
            let mut back: Vec3 = tform.back().into();
            back.y = 0.;
            tform.translation += back.normalize_or_zero() * POSITION_INCR;
        }
        if keys.pressed(KeyCode::KeyA) {
            let mut left: Vec3 = tform.left().into();
            left.y = 0.;
            tform.translation += left.normalize_or_zero() * POSITION_INCR;
        }
        if keys.pressed(KeyCode::KeyD) {
            let mut right: Vec3 = tform.right().into();
            right.y = 0.;
            tform.translation += right.normalize_or_zero() * POSITION_INCR;
        }
        if keys.pressed(KeyCode::Space) {
            tform.translation += Vec3::new(0., 1., 0.) * POSITION_INCR;
        }
        if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftLeft) {
            tform.translation -= Vec3::new(0., 1., 0.) * POSITION_INCR;
        }
    }
}

/// Moves the camera roll-free Minecraft style.
pub fn ego_camera(
    mut mouse_motion: EventReader<MouseMotion>,
    mut camera: Query<(&mut Transform, &mut CameraRotation)>,
) {
    let delta = mouse_motion
        .read()
        .into_iter()
        .fold(Vec2::ZERO, |acc, pos| acc + pos.delta);
    mouse_motion.clear();

    for (mut tform, mut rotation) in camera.iter_mut() {
        rotation.yaw -= delta.x * MOUSE_SENSITIVITY;
        rotation.pitch -= delta.y * MOUSE_SENSITIVITY;
        rotation.pitch = rotation.pitch.clamp(-89.9f32, 89.9f32);

        let yaw_rotation = Quat::from_axis_angle(Vec3::Y, rotation.yaw.to_radians());
        let pitch_rotation = Quat::from_axis_angle(Vec3::X, rotation.pitch.to_radians());

        tform.rotation = yaw_rotation * pitch_rotation;
    }
}

pub fn print_keybindings() {
    let bindings = r#"
Keybindings:

- w: forward
- a: left
- s: back
- d: right
- space: up
- shift: down
- escape: exit
"#;
    println!("{bindings}");
}

/// Makes the cursor invisible over the main window.
pub fn hide_cursor(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    (&mut primary_window.single_mut()).cursor.visible = false;
}
