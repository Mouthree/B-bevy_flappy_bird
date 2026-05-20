use bevy::{app::{Plugin, Update}, ecs::{schedule::IntoScheduleConfigs, system::{Res, ResMut}}, input::{common_conditions::input_just_pressed, keyboard::KeyCode}, state::state::{NextState, State}};

use crate::event::Pause;

///暂停事件

pub struct PauseEventPlugin;
impl Plugin for PauseEventPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, pause_toggle.run_if(input_just_pressed(KeyCode::Escape)));
    }
}
///按下esc切换暂停状态
fn pause_toggle(current: Res<State<Pause>>, mut next: ResMut<NextState<Pause>>) {
    next.set(Pause(!current.0));
}