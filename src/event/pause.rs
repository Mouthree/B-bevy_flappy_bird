use bevy::{app::{Plugin, Update}, ecs::{schedule::IntoScheduleConfigs, system::{Res, ResMut}}, input::{common_conditions::input_just_pressed, keyboard::KeyCode}, state::{condition::in_state, state::{NextState, State}}};

use crate::event::{Menu, Screen};

///暂停事件

pub struct PauseEventPlugin;
impl Plugin for PauseEventPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, pause_toggle
            .run_if(input_just_pressed(KeyCode::Escape))
            .run_if(in_state(Screen::Game))
        );
    }
}
///按下esc切换暂停状态
fn pause_toggle(current: Res<State<Menu>>, mut next: ResMut<NextState<Menu>>) {
    match current.get() {
        Menu::None => next.set(Menu::Pause),
        Menu::Pause => next.set(Menu::None),
        Menu::Setting => next.set(Menu::None),
    }
}