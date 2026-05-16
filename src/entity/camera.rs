//!相机的初始化

use bevy::{app::{App, Plugin, Startup}, camera::{Camera2d, OrthographicProjection, Projection, ScalingMode}, ecs::system::Commands};

use crate::constants::CANVAS_SIZE;

pub struct Camera;
impl Plugin for Camera {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, camera_start_up);
    }
}

///创建摄像头实体
fn camera_start_up(mut commands: Commands) {
    commands.spawn((
       Camera2d,
      Projection::Orthographic(OrthographicProjection {
          scaling_mode: ScalingMode::AutoMax { max_width: CANVAS_SIZE.x, max_height: CANVAS_SIZE.y },
          ..OrthographicProjection::default_2d()
      }) 
    ));
}