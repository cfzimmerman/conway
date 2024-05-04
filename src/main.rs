use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};

const MOUSE_SENSITIVITY: f32 = 0.2;
const POSITION_INCR: f32 = 0.06;

#[derive(Component, Default)]
struct CameraRotation {
    yaw: f32,
    pitch: f32,
}

fn spawn_cube(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_mesh = assets.load("../assets/cube.glb#Mesh0/Primitive0");

    commands.spawn(PbrBundle {
        mesh: cube_mesh.clone(),
        material: materials.add(StandardMaterial {
            base_color: Color::Rgba {
                red: 1.,
                green: 0.,
                blue: 0.,
                alpha: 1.,
            },
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}

fn setup_scene(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 2., 5.).looking_at(Vec3::ZERO, Vec3::Y),
            ..Camera3dBundle::default()
        },
        CameraRotation::default(),
    ));
}

/// Moves the camera roll-free Minecraft style.
fn ego_camera(
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

fn print_keybindings() {
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

// TODO: Filter out keyboard input if the window is not the primary one

/// Handles keybindings and camera movement.
fn watch_keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera: Query<(&mut Transform, &CameraRotation)>,
) {
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

/// Makes the cursor invisible over the main window.
fn hide_cursor(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    (&mut primary_window.single_mut()).cursor.visible = false;
}

fn main() {
    print_keybindings();
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::Rgba {
            red: 0.,
            green: 0.,
            blue: 0.,
            alpha: 0.5,
        }))
        .insert_resource(AmbientLight {
            brightness: 750.,
            ..AmbientLight::default()
        })
        .add_systems(Startup, (hide_cursor, setup_scene, spawn_cube))
        .add_systems(
            Update,
            (
                (ego_camera, watch_keyboard).chain(),
                bevy::window::close_on_esc,
            ),
        )
        .run();
}
