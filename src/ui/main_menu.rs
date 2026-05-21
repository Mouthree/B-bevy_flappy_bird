use bevy::{app::{Plugin, Update}, ecs::{schedule::IntoScheduleConfigs, system::{Res, ResMut}}, input::{common_conditions::input_just_pressed, keyboard::KeyCode}, state::{condition::in_state, state::{NextState, State}}};

use crate::event::Screen;

pub struct MainMenuUI;
impl Plugin for MainMenuUI {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, start_game
            .run_if(input_just_pressed(KeyCode::Space))
            .run_if(in_state(Screen::Main))
        );
    }
}

///主界面和游戏界面的切换
fn start_game(current: Res<State<Screen>>, mut next: ResMut<NextState<Screen>>) {
    match current.get() {
        Screen::Main => next.set(Screen::Game),
        Screen::Game => {},
    }
}