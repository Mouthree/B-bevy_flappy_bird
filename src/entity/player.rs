//!玩家所有相关系统

use bevy::{app::{App, FixedUpdate, Plugin, Startup, Update}, asset::AssetServer, ecs::{component::Component, observer::On, query::With, schedule::IntoScheduleConfigs, system::{Commands, Query, Res, ResMut, Single}}, input::{ButtonInput, mouse::MouseButton}, math::{Vec2, Vec3}, sprite::Sprite, state::state::OnEnter, time::Time, transform::components::Transform, utils::default};
use crate::{constants::{CANVAS_SIZE, PLAYER_SIZE}, event::{EndGame, PausableSys, Screen}, ui::score::Score};


///重力
#[derive(Component)]
struct Gravity(f32);

///速度
#[derive(Component, Default)]
struct Velocity(f32);

///玩家实体
#[derive(Component)]
#[require(Gravity(700.), Velocity)]
pub struct Player;
impl Plugin for Player {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(Screen::Game), player_start_up)
            .add_systems(FixedUpdate, gravity.in_set(PausableSys))
            .add_systems(Update, controls.in_set(PausableSys))
            .add_observer(respawn_on_endgame);
    }
}
///创建玩家实体
fn player_start_up(mut commands: Commands, assert_server: Res<AssetServer>) {
    commands.spawn((
       Player,
       Sprite {
           custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
           image: assert_server.load("images/lu.png"),
           ..default()
       },
       Transform::from_xyz(-CANVAS_SIZE.x / 4., 0., 1.) 
    ));
}

///重力系统
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

///按键操作
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

///游戏结束之后的处理
fn respawn_on_endgame(_: On<EndGame>, player: Single<(&mut Transform, &mut Velocity), With<Player>>, mut score: ResMut<Score>) {
    let (mut transform, mut velocity) = player.into_inner();
    transform.translation = Vec3 {
        x: -CANVAS_SIZE.x / 4.,
        y: 0.,
        z: 1.
    };
    velocity.0 = 0.;
    score.0 = 0;
}

