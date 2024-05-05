use bevy::prelude::*;
use conway::camera::{ego_camera, hide_cursor, keyboard_motion, print_keybindings, CameraRotation};

fn spawn_cube(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_mesh = assets.load("../assets/cube.glb#Mesh0/Primitive0");

    commands.spawn(PbrBundle {
        mesh: cube_mesh.clone(),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        }),
        transform: Transform::from_xyz(2., 0., 0.),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        mesh: cube_mesh.clone(),
        material: materials.add(StandardMaterial {
            base_color: Color::ORANGE_RED,
            ..default()
        }),
        transform: Transform::from_xyz(-2., 0., 0.),
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

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::WHITE,
            intensity: 8_000_000.,
            ..default()
        },
        transform: Transform::from_xyz(-5., 5., -5.),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::WHITE,
            intensity: 8_000_000.,
            ..default()
        },
        transform: Transform::from_xyz(-5., 5., 5.),
        ..default()
    });
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
        /*
        .insert_resource(AmbientLight {
            brightness: 750.,
            ..AmbientLight::default()
        })
        */
        .add_systems(Startup, (hide_cursor, setup_scene, spawn_cube))
        .add_systems(
            Update,
            (
                (ego_camera, keyboard_motion).chain(),
                bevy::window::close_on_esc,
            ),
        )
        .run();
}
