use bevy::prelude::*;

#[derive(Component)]
struct CameraMarker;

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
        CameraMarker,
    ));
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
        .run();
}
