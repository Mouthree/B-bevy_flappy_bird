use bevy::{app::{App, Plugin}, asset::{AssetServer, Handle}, camera::visibility::Visibility, ecs::{children, component::Component, system::{Commands, Res}}, image::{self, Image}, sprite::{BorderRect, SliceScaleMode, Sprite, SpriteImageMode, TextureSlicer}, time::Time, transform::components::Transform, utils::default};

use crate::constants::{CANVAS_SIZE, GAP_SIZE, PIPE_SIZE};

///整个管道
#[derive(Component)]
pub struct Pipe;

///管道顶上
#[derive(Component)]
pub struct PipeTop;

///管道底下
#[derive(Component)]
pub struct PipeBotton;

pub struct PipPlugin;
impl Plugin for PipPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

///创建随机高度的管道
fn pip_start_up(mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>) {
    let image: Handle<Image> = asset_server.load("pipe.png");
    let image_mode = SpriteImageMode::Sliced(
        TextureSlicer {
            border: BorderRect::axes(7., 10.),
            center_scale_mode: SliceScaleMode::Stretch,
            ..default()
        }
    );

    let transform = Transform::from_xyz(CANVAS_SIZE.x, 0., 1.);
    let pip_offset = PIPE_SIZE.y / 2. + GAP_SIZE / 2.;
    let gap_y_position = (time.elapsed_secs() * 4.2309875).sin() * CANVAS_SIZE.y / 4.;

    commands.spawn((
        transform,
        Visibility::Visible,
        Pipe,
        children![(
            Sprite {
                image: image.clone(),
                custom_size: Some(PIPE_SIZE),
                image_mode: image_mode.clone(),
                ..default()
            },
            Transform::from_xyz(0., pip_offset + gap_y_position, 1.),
            PipeTop
        ),
        (
            
        ),
        (
            
        )]
    ));
}
