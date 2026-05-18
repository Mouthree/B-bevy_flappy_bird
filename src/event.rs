use bevy::{app::Plugin, ecs::event::Event};

use crate::event::{collision::CollisionEventPlugin, score::ScoreEventPlugin};
pub mod collision;
pub mod score;


///标志位: 游戏结束
#[derive(Event)]
pub struct EndGame;

///标志位: 触发加分
#[derive(Event)]
pub struct ScoreAdd;

pub struct EventPlugin;
impl Plugin for EventPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((CollisionEventPlugin, ScoreEventPlugin));
    }
}