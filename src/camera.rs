use std::time::Duration;

use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};

use crate::gol::ConwayGol;

const MOUSE_SENSITIVITY: f32 = 0.2;
const POSITION_INCR: f32 = 0.25;

#[derive(Component, Default)]
pub struct CameraRotation {
    yaw: f32,
    pitch: f32,
}

#[derive(Component)]
pub struct GameTimer(pub Timer);

impl Default for GameTimer {
    fn default() -> Self {
        GameTimer(Timer::new(Duration::from_millis(500), TimerMode::Repeating))
    }
}

/// Handles keybindings and camera movement.
pub fn keyboard_motion(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera: Query<(&mut Transform, &CameraRotation)>,
    windows: Query<&Window>,
    mut ctrl_menu: Query<(&mut Visibility, &ControlMenu)>,
    mut game_timer: Query<(&ConwayGol, &mut GameTimer)>,
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
        if keys.just_pressed(KeyCode::KeyH) {
            let (mut vis, _) = ctrl_menu.single_mut();
            *vis = if *vis == Visibility::Hidden {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
        if keys.just_pressed(KeyCode::ArrowUp) {
            let (_, mut timer) = game_timer.single_mut();
            let new_duration = timer.0.duration() / 2;
            if new_duration < Duration::from_millis(125) {
                return;
            }
            timer.0.set_duration(new_duration);
        }
        if keys.just_pressed(KeyCode::ArrowDown) {
            let (_, mut timer) = game_timer.single_mut();
            let new_duration = timer.0.duration() * 2;
            timer.0.set_duration(new_duration);
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

#[derive(Component)]
pub struct ControlMenu;

pub fn display_controls(mut commands: Commands) {
    let bindings = r#"
- h: hide/show this menu

- w: forward
- a: left
- s: back
- d: right
- space: up
- shift: down
- left click: pause/play
- up arrow: speed 2x
- down arrow: speed 0.5x
- escape: exit
"#;

    commands.spawn((
        TextBundle {
            text: Text::from_section(
                bindings,
                TextStyle {
                    font_size: 18.,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            visibility: Visibility::Visible,
            ..default()
        }
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(18.),
            left: Val::Px(18.),
            ..default()
        }),
        ControlMenu,
    ));
}

/// Makes the cursor invisible over the main window.
pub fn hide_cursor(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    (&mut primary_window.single_mut()).cursor.visible = false;
}
