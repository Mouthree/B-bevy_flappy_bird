//玩家所有相关系统

use bevy::{app::{App, FixedUpdate, Plugin, Startup, Update}, asset::{AssetServer, transformer}, ecs::{component::Component, query::With, system::{Commands, Query, Res, Single}}, input::{ButtonInput, mouse::MouseButton}, math::Vec2, sprite::Sprite, time::Time, transform::components::Transform, utils::default};
use crate::{constants::{CANVAS_SIZE, PLAYER_SIZE}};
use crate::states::*;

///重力
#[derive(Component)]
pub struct Gravity(pub f32);

///速度
#[derive(Component, Default)]
pub struct Velocity(pub f32);

//玩家插件
#[derive(Component)]
#[require(Gravity(700.), Velocity)]
pub struct Player;
impl Plugin for Player {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, player_start_up)
            .add_systems(FixedUpdate, gravity)
            .add_systems(Update, controls);
    }
}
//创建玩家实体
fn player_start_up(mut commands: Commands, assert_server: Res<AssetServer>) {
    commands.spawn((
       Player,
       Sprite {
           custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
           image: assert_server.load("lu.png"),
           ..default()
       },
       Transform::from_xyz(-CANVAS_SIZE.x / 4., 0., 1.) 
    ));
}

//重力
fn gravity(
    //遍历有这三个组件的实体
    mut transforms: Query<(
        &mut Transform,
        &mut Velocity,
        &Gravity
    )>,
    time: Res<Time>
) {
    for (mut transform, mut velocity, gravity) in &mut transforms {
        velocity.0 -= gravity.0 * time.delta_secs();
        transform.translation.y += velocity.0 * time.delta_secs();
    }
}

//按键
fn controls(
    mut velocity: Single<&mut Velocity, With<Player>>,
    buttons: Res<ButtonInput<MouseButton>>
) {
    if buttons.any_just_pressed([
        MouseButton::Left
    ]) {
        velocity.0 = 200.;
    }
}

//碰到边界触发事件
fn check_in_bounds(player: Single<&Transform, With<Player>>, mut commands: Commands) {
    if player.translation.y < -CANVAS_SIZE.y / 2. - PLAYER_SIZE || 
    player.translation.y > CANVAS_SIZE.y / 2. + PLAYER_SIZE {
        commands.trigger(EndGame);
    }
}