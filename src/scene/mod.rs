use bevy::{
    ecs::component::Component,
    time::{Timer, TimerMode},
};
use std::time::Duration;

pub mod interaction;
pub mod sim;
pub mod world;

/// Units moved per event trigger
const MOUSE_SENSITIVITY: f32 = 0.2;
const POSITION_INCR: f32 = 0.25;

/// Configures the size of the grid of cubes
const BOARD_SIZE: usize = 2usize.pow(7);
const CUBE_SPACING: f32 = 2.25;

/// Whether or not the cube simulation is paused.
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

/// Controls the rotation of the ego camera
#[derive(Component, Default)]
pub struct CameraRotation {
    yaw: f32,
    pitch: f32,
}

/// Tracks when the game state needs to be ticked
#[derive(Component)]
pub struct GameTimer(pub Timer);

impl Default for GameTimer {
    fn default() -> Self {
        GameTimer(Timer::new(Duration::from_millis(500), TimerMode::Repeating))
    }
}

/// Associates a cube with its row, col pair in the underlying simulation grid
#[derive(Component, Default)]
pub struct CubeInd {
    row: usize,
    col: usize,
}

/// Marker struct for help menu text
#[derive(Component)]
pub struct ControlMenu;
