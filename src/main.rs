use bevy::prelude::*;
use conway::scene::{
    interaction::{display_controls, ego_camera, handle_click, hide_cursor, keyboard_motion},
    world::{init_conway_grid, next_game_tick, setup_world},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::Rgba {
            red: 0.,
            green: 0.,
            blue: 0.,
            alpha: 0.5,
        }))
        .add_systems(
            Startup,
            (hide_cursor, setup_world, init_conway_grid, display_controls),
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
