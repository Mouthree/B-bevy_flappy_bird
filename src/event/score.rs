use bevy::{app::Plugin, ecs::{observer::On, system::ResMut}};

use crate::{event::ScoreAdd, ui::score::Score};


///分数事件
pub struct ScoreEventPlugin;
impl Plugin for ScoreEventPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_observer(score_add);
    }
}

///触发加分事件的时候加分
fn score_add(_: On<ScoreAdd>, mut score: ResMut<Score>) {
    score.0 += 1;
}