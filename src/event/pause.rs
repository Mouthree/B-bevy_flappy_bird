use bevy::{app::{Plugin, Update}, ecs::system::{Res, ResMut}, input::{ButtonInput, keyboard::KeyCode}, state::state::{NextState, State}};

use crate::event::Pause;

///暂停事件

pub struct PauseEventPlugin;
impl Plugin for PauseEventPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, test_esc);
    }
}

fn test_esc(key: Res<ButtonInput<KeyCode>>, current: Res<State<Pause>>, mut next: ResMut<NextState<Pause>>) {
    if key.just_pressed(KeyCode::Escape) {
        next.set(Pause(!current.0));
    }
}