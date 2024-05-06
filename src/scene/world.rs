use super::{sim::ConwayGol, CameraRotation, CubeInd, GameTimer, Paused, BOARD_SIZE, CUBE_SPACING};
use bevy::prelude::*;

/// Creates one-time world assets like the camera, sky, and sun.
pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let offset = BOARD_SIZE as f32;
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(8., 8., 0.).looking_at(Vec3::ZERO, Vec3::Y),
            ..Camera3dBundle::default()
        },
        CameraRotation::default(),
    ));

    // A dome around the world to reflect back the sun
    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(offset * 8.)),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.2, 0.2, 0.2),
            perceptual_roughness: 0.08,
            cull_mode: None,
            double_sided: true,
            ..default()
        }),
        ..default()
    });

    // The sun
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::WHITE,
            intensity: 12_000_000_000.,
            range: offset * 8.,
            ..default()
        },
        transform: Transform::from_xyz(-offset, offset, -offset * 2.),
        ..default()
    });
}

/// Builds the Game of Life simulation and sets up the geometries used
/// to render it.
pub fn init_conway_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Oversize the board to make the edges look more alive
    let gol = ConwayGol::build_rand(BOARD_SIZE * 2)
        .expect("Conway grid must initialize in order to continue");

    let cube_mesh = meshes.add(Cuboid::new(2., 2., 2.));
    let cube_mat = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });

    commands
        .spawn_empty()
        .insert(gol)
        .insert(Paused(true))
        .insert(GameTimer::default())
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(InheritedVisibility::default())
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
                            // resource handles have cheap clone
                            mesh: cube_mesh.clone(),
                            material: cube_mat.clone(),
                            transform: Transform::from_xyz(x, 0., z),
                            ..Default::default()
                        },
                        CubeInd { row, col },
                    ));
                }
            }
        });
}

/// Every time the timer completes, computes the next Game of Life board
/// state and updates resources controlled by the simulation.
pub fn next_game_tick(
    mut game_state: Query<(&mut ConwayGol, &Paused, &mut GameTimer)>,
    mut cubes: Query<(&mut Visibility, &CubeInd)>,
    time: Res<Time>,
) {
    let (mut game_state, sim, mut timer) = game_state.single_mut();
    if !timer.0.tick(time.delta()).finished() {
        return;
    }
    if sim.is_paused() {
        return;
    }

    game_state.tick();
    let board = game_state.board();
    for (mut vis, pos) in &mut cubes {
        *vis = if board[pos.row][pos.col] {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
