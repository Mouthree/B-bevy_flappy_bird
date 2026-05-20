use bevy::{app::{FixedUpdate, Plugin, Update}, ecs::{event::Event, schedule::{IntoScheduleConfigs, SystemSet}}, state::{app::AppExtStates, condition::in_state, state::States}};

use crate::event::{collision::CollisionEventPlugin, pause::PauseEventPlugin, score::ScoreEventPlugin};
pub mod collision;
pub mod score;
pub mod pause;


///标志位: 游戏结束
#[derive(Event)]
pub struct EndGame;

///标志位: 触发加分
#[derive(Event)]
pub struct ScoreAdd;


///标记: 暂停时是否需要运动
#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PausableSys;

///状态: 界面
#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Main,
    Game
}

///状态: 浮窗
#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum Menu {
    #[default]
    None,
    Pause,
    Setting
}


///整合所有event的插件
pub struct EventPlugin;
impl Plugin for EventPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_plugins((CollisionEventPlugin, ScoreEventPlugin, PauseEventPlugin))
            .init_state::<Screen>()
            .init_state::<Menu>()
            .configure_sets(Update, PausableSys
                .run_if(in_state(Menu::None))
                .run_if(in_state(Screen::Game))
            )
            .configure_sets(FixedUpdate, PausableSys
                .run_if(in_state(Menu::None))
                .run_if(in_state(Screen::Game))
            );
    }
}