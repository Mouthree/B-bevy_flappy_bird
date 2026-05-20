use bevy::app::Plugin;

use crate::ui::score::ScoreText;

pub mod score;
pub mod pause;

///UI
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(ScoreText);
    }
}