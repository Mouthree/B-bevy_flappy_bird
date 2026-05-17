//!分数的显示

use bevy::{app::{App, Plugin, Startup, Update}, asset::AssetServer, color::palettes::tailwind::SLATE_50, ecs::{change_detection::DetectChanges, component::Component, query::With, resource::Resource, schedule::{IntoScheduleConfigs, common_conditions::resource_changed}, system::{Commands, Query, Res}}, text::{TextColor, TextFont, TextLayout}, ui::{Node, percent, px, widget::Text}, utils::default};

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
        app.add_systems(Startup, score_text_start_up);
        app.add_systems(Update, score_update.run_if(resource_changed::<Score>));
    }
}

///创建记分的本体
fn score_text_start_up(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
       Node {
           width: percent(100.),
           margin: px(20.).top(),
           ..default()
       },
       Text::new("0"),
       TextLayout::new_with_justify(bevy::text::Justify::Center),
       TextFont {
           font: asset_server.load("fonts/ark-pixel-12px-proportional-zh_cn.ttf"),
           font_size: 33.,
           ..default()
       },
       TextColor(SLATE_50.into()),
       ScoreTextSign
    ));
}

///更新分数
fn score_update(mut query: Query<&mut Text, With<ScoreTextSign>>, score: Res<Score>) {
    if score.is_changed() {
        for mut span in &mut query {
            span.0 = score.0.to_string();
        }
    }
}