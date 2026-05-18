#![allow(non_snake_case)]

use bevy::app::Plugin;

use crate::{entity::EntityPlugin, event::EventPlugin, ui::UIPlugin};
pub mod constants;
pub mod resources;
pub mod entity;
pub mod ui;
pub mod event;

pub struct Game;
impl Plugin for Game {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((EntityPlugin, EventPlugin, UIPlugin));
    }
}