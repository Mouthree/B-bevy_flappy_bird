//!分数的显示

use bevy::{app::{App, Plugin}, color::palettes::tailwind::SLATE_50, ecs::{component::Component, resource::Resource, system::Commands}, text::{TextColor, TextFont, TextLayout}, ui::{Node, percent, px, widget::Text}, utils::default};

///分数
#[derive(Resource, Default)]
pub struct Score(pub u32);

///标记文字本体
#[derive(Component)]
pub struct ScoreTextSign;

///分数文本插件
pub struct ScoreText;
impl Plugin for ScoreText {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>();
    }
}

///创建记分的本体
fn score_text_start_up(mut commands: Commands) {
    commands.spawn((
       Node {
           width: percent(100.),
           margin: px(20.).top(),
           ..default()
       },
       Text::new("0"),
       TextLayout::new_with_justify(bevy::text::Justify::Center),
       TextFont {
           font_size: 33.,
           ..default()
       },
       TextColor(SLATE_50.into()),
       ScoreTextSign
    ));
}