use bevy::app::Plugin;

use crate::ui::{main_menu::MainMenuUI, pause::PauseUI, score::ScoreText};

pub mod score;
pub mod pause;
pub mod main_menu;

///UI
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((ScoreText, PauseUI, MainMenuUI));
    }
}