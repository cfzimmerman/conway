use std::time::Duration;

use bevy::{
    ecs::component::Component,
    time::{Timer, TimerMode},
};

pub mod interaction;
pub mod sim;
pub mod world;

const MOUSE_SENSITIVITY: f32 = 0.2;
const POSITION_INCR: f32 = 0.25;

const BOARD_SIZE: usize = 2usize.pow(7);
const CUBE_SPACING: f32 = 2.25;

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

#[derive(Component, Default)]
pub struct CameraRotation {
    yaw: f32,
    pitch: f32,
}

#[derive(Component)]
pub struct GameTimer(pub Timer);

impl Default for GameTimer {
    fn default() -> Self {
        GameTimer(Timer::new(Duration::from_millis(500), TimerMode::Repeating))
    }
}

#[derive(Component, Default)]
pub struct CubeInd {
    row: usize,
    col: usize,
}

#[derive(Component)]
pub struct ControlMenu;
