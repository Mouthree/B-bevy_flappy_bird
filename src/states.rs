//游戏状态
use bevy::ecs::event::Event;

///游戏结束标志位
#[derive(Event)]
pub struct EndGame;