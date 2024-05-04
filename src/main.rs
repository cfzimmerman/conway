use bevy::{input::mouse::MouseMotion, prelude::*};

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

const MOUSE_SENSITIVITY: f32 = 0.2;

fn ego_camera(
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut CameraRotation)>,
) {
    let delta = mouse_motion
        .read()
        .into_iter()
        .fold(Vec2::ZERO, |acc, pos| acc + pos.delta);
    mouse_motion.clear();

    for (mut tform, mut rotation) in query.iter_mut() {
        rotation.yaw -= delta.x * MOUSE_SENSITIVITY;
        rotation.pitch += delta.y * MOUSE_SENSITIVITY;
        rotation.pitch = rotation.pitch.clamp(-89.9f32, 89.9f32);

        let yaw_rotation = Quat::from_axis_angle(Vec3::Y, rotation.yaw.to_radians());
        let pitch_rotation = Quat::from_axis_angle(Vec3::X, rotation.pitch.to_radians());

        tform.rotation = yaw_rotation * pitch_rotation;
    }
}

fn main() {
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
        .add_systems(Startup, (setup_scene, spawn_cube))
        .add_systems(Update, ego_camera)
        .run();
}
