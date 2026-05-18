//!管道
use std::time::Duration;

use bevy::{app::{App, FixedUpdate, Plugin, Update}, asset::{AssetServer, Handle}, camera::visibility::Visibility, color::Color, ecs::{children, component::Component, entity::Entity, query::{With}, schedule::IntoScheduleConfigs, system::{Commands, Query, Res}}, image::{Image}, math::{Vec2}, sprite::{BorderRect, SliceScaleMode, Sprite, SpriteImageMode, TextureSlicer}, time::{Time, common_conditions::on_timer}, transform::{ components::Transform}, utils::default};
use crate::{constants::{CANVAS_SIZE, GAP_SIZE, PIPE_SIZE, PIPE_SPEED}};

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
           pipe_del
        ));
        app.add_systems(Update, pipe_move);
        
    }
}

///创建随机高度的管道
fn pipe_start_up(mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>) {
    let image: Handle<Image> = asset_server.load("images/pipe.png");
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
        pipe.translation.x -= PIPE_SPEED * time.delta_secs();
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