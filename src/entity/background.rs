//!背景相关

use bevy::{app::{Plugin, Startup}, asset::{Asset, AssetServer, Assets, Handle}, ecs::system::{Commands, Res, ResMut}, image::{Image, ImageLoaderSettings}, math::primitives::Rectangle, mesh::{Mesh, Mesh2d}, reflect::TypePath, render::render_resource::AsBindGroup, shader::ShaderRef, sprite_render::{AlphaMode2d, Material2d, Material2dPlugin, MeshMaterial2d}, transform::components::Transform};

use crate::constants::CANVAS_SIZE;

///背景实体
pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_systems(Startup, background_start_up)
            .add_plugins(Material2dPlugin::<BackgroundMaterial>::default());
    }
}

///创建背景实体
fn background_start_up(mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
    asset_server: Res<AssetServer>
) {
    //创建背景一
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(CANVAS_SIZE.x, CANVAS_SIZE.y))),
        MeshMaterial2d(materials.add(BackgroundMaterial {
            color_texture: asset_server.load_with_settings(
                "images/background1.png", 
                |settings: &mut ImageLoaderSettings| {
                    settings
                        .sampler
                        .get_or_init_descriptor()
                        .set_address_mode(bevy::image::ImageAddressMode::Repeat);
                }
            ),
            speed: 0.2
        })),
        Transform::from_xyz(0., 0., -2.)
    ));
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BackgroundMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub color_texture: Handle<Image>,
    #[uniform(2)]
    pub speed: f32
}

impl Material2d for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/background.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}