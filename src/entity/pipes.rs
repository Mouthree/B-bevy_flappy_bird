//!管道
use std::time::Duration;

use bevy::{app::{App, FixedUpdate, Plugin, Update, ctrlc::Signal}, asset::{AssetServer, Handle}, camera::visibility::Visibility, color::Color, ecs::{children, component::Component, entity::Entity, error::Result, observer::On, query::{Or, With}, schedule::IntoScheduleConfigs, system::{Commands, Query, Res, ResMut, Single}}, gizmos::{gizmos::Gizmos, retained::Gizmo}, image::{self, Image}, log::info_span, math::{Vec2, Vec3Swizzles, bounding::{Aabb2d, BoundingCircle, IntersectsVolume}}, sprite::{BorderRect, SliceScaleMode, Sprite, SpriteImageMode, TextureSlicer}, time::{Fixed, Time, common_conditions::on_timer}, transform::{self, components::Transform, helper::TransformHelper}, utils::default};
use crate::{constants::{CANVAS_SIZE, GAP_SIZE, PIPE_SIZE, PIPE_SPEED, PLAYER_SIZE}, entity::{pipes, player::Player}, event::{EndGame, ScoreAdd}, ui::score::Score};

///整个管道
#[derive(Component)]
pub struct PipeAll;

///管道顶上
#[derive(Component)]
pub struct PipeTop;

///管道底下
#[derive(Component)]
pub struct PipeBotton;

///管道中间
#[derive(Component)]
pub struct PointsGate;

///管道插件
pub struct Pipe;
impl Plugin for Pipe {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (
           pipe_start_up.run_if(on_timer(Duration::from_millis(1000))),
           pipe_del,
           pipe_hit
        ));
        app.add_systems(Update, pipe_move);
        app.add_observer(score_add);
    }
}

///创建随机高度的管道
fn pipe_start_up(mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>) {
    let image: Handle<Image> = asset_server.load("pipe.png");
    //设置图片显示模式为九宫格
    let image_mode = SpriteImageMode::Sliced(
        //具体切割逻辑
        TextureSlicer {
            //左右7px, 上下10px不变
            border: BorderRect::axes(7., 10.),
            //其余直接拉伸
            center_scale_mode: SliceScaleMode::Stretch,
            ..default()
        }
    );
    //新生成管道位置
    let transform = Transform::from_xyz(CANVAS_SIZE.x, 0., 1.);
    //移动管道, 布局为管道下-空隙-管道上
    let pipe_offset = PIPE_SIZE.y / 2. + GAP_SIZE / 2.;
    //随机的移动
    let gap_y_position = (time.elapsed_secs() * 4.2309875).sin() * CANVAS_SIZE.y / 4.;
    //创建管道实体, 由上管, 空隙, 下管组成
    commands.spawn((
        transform,
        Visibility::Visible,
        PipeAll,
        children![(
            Sprite {
                image: image.clone(),
                custom_size: Some(PIPE_SIZE),
                image_mode: image_mode.clone(),
                ..default()
            },
            Transform::from_xyz(0., pipe_offset + gap_y_position, 1.),
            PipeTop
        ),
        (
            //设置为不可见
            Visibility::Hidden,
            Sprite {
                //这个色在调试的时候可能会用到
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10., GAP_SIZE)),
                ..default()
            },
            Transform::from_xyz(0., gap_y_position, 1.),
            PointsGate
        ),
        (
            Sprite {
                image,
                custom_size: Some(PIPE_SIZE),
                image_mode,
                ..default()
            },
            Transform::from_xyz(0., -pipe_offset + gap_y_position, 1.),
            PipeBotton
        )]
    ));
}

///设置管道移动
fn pipe_move(mut pipes: Query<&mut Transform, With<PipeAll>>, time: Res<Time>) {
    for mut pipe in &mut pipes {
        //TODO: 这个管道的显示依旧不清晰, 不知道为什么, 到时候研究一下怎么改
        pipe.translation.x -= PIPE_SPEED * time.delta_secs();
        pipe.translation.x = pipe.translation.x.round();
    }
}

///销毁出界管道
fn pipe_del(mut commands: Commands, pipes: Query<(Entity, &Transform), With<PipeAll>>) {
    for (entity, transform) in pipes.iter() {
        if transform.translation.x < -(CANVAS_SIZE.x / 2. + PIPE_SIZE.x) {
            commands.entity(entity).despawn();
        }
    }
}

///计算对管道的碰撞
fn pipe_hit(
    mut commands: Commands,
    player: Single<(&Sprite, Entity), With<Player>>,
    pipe_segments: Query<(&Sprite, Entity), Or<(With<PipeTop>, With<PipeBotton>)>>,
    pipe_gaps: Query<(&Sprite, Entity), With<PointsGate>>,
    mut gizmos: Gizmos,
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

fn score_add(_: On<ScoreAdd>, mut score: ResMut<Score>) {
    score.0 += 1;
}
