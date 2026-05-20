//!暂停时的界面, 包含暗色遮罩以及暂停ui

use bevy::{app::{Plugin, Update}, color::Color, ecs::system::Commands, state::{state::OnEnter, state_scoped::DespawnOnExit}, ui::{BackgroundColor, GlobalZIndex, Node, percent}, utils::default};

use crate::event::Pause;

pub struct PauseUI;
impl Plugin for PauseUI {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(Pause(true)), pause_dark_start_up);
    }
}

fn pause_dark_start_up(mut commands: Commands) {
    commands.spawn((
       Node {
           width: percent(100),
           height: percent(100),
           ..default()
       },
       GlobalZIndex(1),
       BackgroundColor(Color::srgba(0., 0., 0., 0.5)),
       DespawnOnExit(Pause(true))
    ));
}