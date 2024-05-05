use bevy::{prelude::*, time::common_conditions::on_timer};
use conway::{
    camera::{ego_camera, hide_cursor, keyboard_motion, print_keybindings, CameraRotation},
    gol::ConwayGol,
};
use std::time::Duration;

const BOARD_SIZE: usize = 2usize.pow(6);
const CUBE_SPACING: f32 = 2.25;
const TICK_SPEED: Duration = Duration::from_millis(250);

#[derive(Component, Default)]
pub struct CubeInd {
    row: usize,
    col: usize,
}

fn init_conway_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Oversize the board to make the edges look more alive
    let gol = ConwayGol::build_rand(BOARD_SIZE * 2)
        .expect("Conway grid must initialize in order to continue");

    let cube_mesh = meshes.add(Cuboid::new(2., 2., 2.));
    commands
        .spawn_empty()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(InheritedVisibility::default())
        .insert(gol)
        .with_children(|parent| {
            let middle_cube = BOARD_SIZE as f32 / 2.;
            let board_offset = BOARD_SIZE / 2;

            let live_region = board_offset..(BOARD_SIZE + board_offset);
            for row in live_region.clone() {
                for col in live_region.clone() {
                    let x = CUBE_SPACING * (middle_cube - (row - board_offset) as f32);
                    let z = CUBE_SPACING * (middle_cube - (col - board_offset) as f32);
                    parent.spawn((
                        PbrBundle {
                            mesh: cube_mesh.clone(),
                            material: materials.add(StandardMaterial {
                                base_color: Color::BLACK,
                                ..default()
                            }),
                            transform: Transform::from_xyz(x, 0., z),
                            ..Default::default()
                        },
                        CubeInd { row, col },
                    ));
                }
            }
        });
}

fn next_game_tick(
    mut game_state: Query<&mut ConwayGol>,
    mut cubes: Query<(&Handle<StandardMaterial>, &CubeInd)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut game_state = game_state.single_mut();
    game_state.tick();
    let board = game_state.board();

    for (handle, pos) in &mut cubes {
        let Some(mat) = materials.get_mut(handle) else {
            continue;
        };
        mat.base_color = if board[pos.row][pos.col] {
            Color::ORANGE_RED
        } else {
            Color::WHITE
        };
    }
}

fn setup_scene(mut commands: Commands) {
    let offset = BOARD_SIZE as f32;
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., offset / 2., offset).looking_at(Vec3::ZERO, Vec3::Y),
            ..Camera3dBundle::default()
        },
        CameraRotation::default(),
    ));
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
            color: Color::WHITE,
            brightness: 1000.,
        })
        .add_systems(Startup, (hide_cursor, setup_scene, init_conway_grid))
        .add_systems(
            Update,
            (
                (ego_camera, keyboard_motion).chain(),
                next_game_tick.run_if(on_timer(TICK_SPEED)),
                bevy::window::close_on_esc,
            ),
        )
        .run();
}
