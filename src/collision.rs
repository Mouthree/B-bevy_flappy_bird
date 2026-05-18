//!碰撞系统

use bevy::{app::{App, FixedUpdate, Plugin}, ecs::{entity::Entity, error::Result, query::{Or, With}, system::{Commands, Query, Single}}, math::{Vec3Swizzles, bounding::{Aabb2d, BoundingCircle, IntersectsVolume}}, sprite::Sprite, transform::{components::Transform, helper::TransformHelper}};

use crate::{constants::{CANVAS_SIZE, PLAYER_SIZE}, entity::{pipes::{PipeBotton, PipeTop, PointsGate}, player::Player}, event::{EndGame, ScoreAdd}};

///碰撞检测的插件
pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (check_in_bounds, pipe_hit));
    }
}

///玩家碰到边界触发事件
fn check_in_bounds(player: Single<&Transform, With<Player>>, mut commands: Commands) {
    if player.translation.y < -CANVAS_SIZE.y / 2. - PLAYER_SIZE || 
    player.translation.y > CANVAS_SIZE.y / 2. + PLAYER_SIZE {
        commands.trigger(EndGame);
    }
}

///计算对管道的碰撞
fn pipe_hit(
    mut commands: Commands,
    player: Single<(&Sprite, Entity), With<Player>>,
    pipe_segments: Query<(&Sprite, Entity), Or<(With<PipeTop>, With<PipeBotton>)>>,
    pipe_gaps: Query<(&Sprite, Entity), With<PointsGate>>,
    transform_helper: TransformHelper
) -> Result<()> {
    //获取到最新的player位置
    let player_transform = transform_helper.compute_global_transform(player.1)?;
    //创建一个以玩家中心为原点, 玩家一半大的为半径的这个碰撞箱
    let player_collider = BoundingCircle::new(player_transform.translation().xy(), PLAYER_SIZE / 2.);
    //处理所有的管道(上下两部分)
    for (sprite, entity) in &pipe_segments {
        //获取管道实际坐标
        let pipe_transform = transform_helper.compute_global_transform(entity)?;
        //方的碰撞箱
        let pipe_collider = Aabb2d::new(pipe_transform.translation().xy(), sprite.custom_size.unwrap() / 2.);
        //检测是否碰到
        if player_collider.intersects(&pipe_collider) {
            commands.trigger(EndGame);
        }
    }
    //处理中间得分 区域
    for (sprite, entity) in &pipe_gaps {
        //这些处理同上
        let pipe_transform = transform_helper.compute_global_transform(entity)?;
        let pipe_collider = Aabb2d::new(pipe_transform.translation().xy(), sprite.custom_size.unwrap() / 2.);
        //处理一次之后删除掉, 防止重复积分
        if player_collider.intersects(&pipe_collider) {
            commands.trigger(ScoreAdd);
            commands.entity(entity).despawn();
        }
    }
    Ok(())
}
