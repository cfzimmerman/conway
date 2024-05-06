use bevy::{prelude::*, render::mesh::shape::Cube};
use conway::{
    camera::{
        display_controls, ego_camera, hide_cursor, keyboard_motion, CameraRotation, GameTimer,
    },
    gol::ConwayGol,
};

const BOARD_SIZE: usize = 2usize.pow(6);
const CUBE_SPACING: f32 = 2.25;

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
    let cube_mat = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });
    commands
        .spawn_empty()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(InheritedVisibility::default())
        .insert(gol)
        .insert(Paused(false))
        .insert(GameTimer::default())
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

#[derive(Component)]
pub struct Paused(bool);

impl Paused {
    pub fn toggle(&mut self) {
        self.0 = !self.0;
    }

    #[inline]
    pub fn is_paused(&self) -> bool {
        self.0
    }
}

fn handle_click(
    buttons: Res<ButtonInput<MouseButton>>,
    mut game_state: Query<(&ConwayGol, &mut Paused)>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (_, mut sim) = game_state.single_mut();
        sim.toggle();
    }
}

fn next_game_tick(
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

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let offset = BOARD_SIZE as f32;
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., offset / 2., offset).looking_at(Vec3::ZERO, Vec3::Y),
            ..Camera3dBundle::default()
        },
        CameraRotation::default(),
    ));

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

    let sun_pos = Transform::from_xyz(-offset * 2., offset * 2., -offset * 2.);

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color: Color::WHITE,
            intensity: 10_000_000_000.,
            range: offset * 8.,
            radius: 0.4,
            // shadows_enabled: true,
            ..default()
        },
        transform: sun_pos,
        ..default()
    });
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
        /*
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1000.,
        })
        */
        .add_systems(
            Startup,
            (hide_cursor, setup_scene, init_conway_grid, display_controls),
        )
        .add_systems(
            Update,
            (
                (ego_camera, keyboard_motion).chain(),
                next_game_tick,
                handle_click,
                bevy::window::close_on_esc,
            ),
        )
        .run();
}
