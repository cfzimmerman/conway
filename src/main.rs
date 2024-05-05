use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use conway::{
    camera::{ego_camera, hide_cursor, keyboard_motion, print_keybindings, CameraRotation},
    gol::ConwayGol,
};

const BOARD_SIZE: usize = 2usize.pow(3);
const CUBE_SPACING: f32 = 2.25;

#[derive(Component, Default)]
pub struct CubeInd {
    row: usize,
    col: usize,
}

impl CubeInd {
    #[inline]
    fn row(&self) -> usize {
        self.row
    }

    #[inline]
    fn col(&self) -> usize {
        self.col
    }
}

fn init_conway_grid(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_mesh = assets.load("../assets/cube.glb#Mesh0/Primitive0");
    commands
        .spawn_empty()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(InheritedVisibility::default())
        .with_children(|parent| {
            // Oversize the board to make the edges look more alive
            let gol = ConwayGol::build_rand(BOARD_SIZE * 2)
                .expect("Conway grid must initialize in order to continue");
            let board = gol.board();

            let middle_cube = BOARD_SIZE as f32 / 2.;
            let board_offset = BOARD_SIZE / 2;

            let live_region = board_offset..(BOARD_SIZE + board_offset);
            for row in live_region.clone() {
                for col in live_region.clone() {
                    let color = if board[row][col] {
                        Color::ORANGE_RED
                    } else {
                        Color::WHITE
                    };
                    let x = CUBE_SPACING * (middle_cube - (row - board_offset) as f32);
                    let z = CUBE_SPACING * (middle_cube - (col - board_offset) as f32);
                    parent.spawn((
                        PbrBundle {
                            mesh: cube_mesh.clone(),
                            material: materials.add(StandardMaterial {
                                base_color: color,
                                ..default()
                            }),
                            transform: Transform::from_xyz(x, 0., z),
                            ..Default::default()
                        },
                        CubeInd { row, col },
                    ));
                }
            }
            parent.spawn(gol);
        });
}

fn conway_tick() {}

fn setup_scene(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 2., 5.).looking_at(Vec3::ZERO, Vec3::Y),
            ..Camera3dBundle::default()
        },
        CameraRotation::default(),
    ));

    let light_offset = BOARD_SIZE as f32;

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::WHITE,
            intensity: 8_000_000.,
            ..default()
        },
        transform: Transform::from_xyz(light_offset, light_offset, light_offset),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::WHITE,
            intensity: 8_000_000.,
            ..default()
        },
        transform: Transform::from_xyz(-light_offset, light_offset, -light_offset),
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
        .insert_resource(AmbientLight {
            brightness: 750.,
            ..AmbientLight::default()
        })
        .add_systems(Startup, (hide_cursor, setup_scene, init_conway_grid))
        .add_systems(
            Update,
            (
                (ego_camera, keyboard_motion).chain(),
                conway_tick.run_if(on_timer(Duration::from_secs(1))),
                bevy::window::close_on_esc,
            ),
        )
        .run();
}
