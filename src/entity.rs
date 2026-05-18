use bevy::app::{Plugin};

use crate::{ entity::{camera::Camera, pipes::Pipe, player::Player}};

pub mod background;
pub mod pipes;
pub mod player;
pub mod camera;

///实体
pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((Player, Camera, Pipe));
    }
}