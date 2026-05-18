use B_Flappy_Bird::Game;
use bevy::{DefaultPlugins, app::{App, AppExit, PluginGroup, }, image::ImagePlugin};


fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(Game)
        .run()
}
