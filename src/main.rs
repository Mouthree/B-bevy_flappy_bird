use bevy::{DefaultPlugins, app::{App, AppExit, PluginGroup, }, image::ImagePlugin};
use B_Flappy_Bird::{ entity::{camera::Camera, player::Player}};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((Player, Camera))
        .run()
}
