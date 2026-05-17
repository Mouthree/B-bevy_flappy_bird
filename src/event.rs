//!游戏状态
use bevy::ecs::event::Event;

///标志位: 游戏结束
#[derive(Event)]
pub struct EndGame;

#[derive(Event)]
pub struct ScoreAdd;