//!存全部常量
use bevy::prelude::Vec2;

///窗口大小
pub const CANVAS_SIZE: Vec2 = Vec2::new(480., 270.);
///角色位置
pub const PLAYER_SIZE: f32 = 32.0;
///管道尺寸
pub const PIPE_SIZE: Vec2 = Vec2::new(32., CANVAS_SIZE.y);
///缝隙尺寸
pub const GAP_SIZE: f32 = 100.;
///管道移动速度
pub const PIPE_SPEED: f32 = 200.;