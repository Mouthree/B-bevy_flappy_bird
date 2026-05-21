//!背景相关

use bevy::{app::{Plugin, Startup, Update}, asset::{Asset, AssetServer, Assets, Handle}, ecs::{schedule::IntoScheduleConfigs, system::{Commands, Res, ResMut}}, image::{Image, ImageLoaderSettings}, math::primitives::Rectangle, mesh::{Mesh, Mesh2d}, reflect::TypePath, render::render_resource::AsBindGroup, shader::ShaderRef, sprite_render::{AlphaMode2d, Material2d, Material2dPlugin, MeshMaterial2d}, state::{condition::in_state, state::{OnEnter, State}}, time::Time, transform::components::Transform};

use crate::{constants::CANVAS_SIZE, event::{Menu, Screen}};


///背景实体
pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_systems(Startup, background_start_up)
            .add_plugins(Material2dPlugin::<BackgroundMaterial>::default())
            .add_systems(Update, background_timer_tick.run_if(in_state(Menu::None)))
            .add_systems(Update, update_blur)
        ;
    }
}

///创建背景实体
fn background_start_up(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(CANVAS_SIZE.x, CANVAS_SIZE.y))),
        MeshMaterial2d(materials.add(BackgroundMaterial {
            bg_texture: asset_server.load_with_settings(
                "images/background1.png",
                |s: &mut ImageLoaderSettings| {
                    s.sampler.get_or_init_descriptor()
                        .set_address_mode(bevy::image::ImageAddressMode::Repeat);
                },
            ),
            cloud_texture: asset_server.load_with_settings(
                "images/background2.png",
                |s: &mut ImageLoaderSettings| {
                    s.sampler.get_or_init_descriptor()
                        .set_address_mode(bevy::image::ImageAddressMode::Repeat);
                },
            ),
            speed1: 0.15,
            speed2: 0.04,
            offset1: 0.,
            offset2: 0.,
            blur: 3.,
        })),
        Transform::from_xyz(0., 0., -2.),
    ));
}

//TODO: 没太懂
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BackgroundMaterial {
    #[texture(0)]
    #[sampler(2)]
    pub bg_texture: Handle<Image>,

    #[texture(1)]
    pub cloud_texture: Handle<Image>,

    pub speed1: f32,
    pub speed2: f32,

    #[uniform(3)]
    pub offset1: f32,

    #[uniform(4)]
    pub offset2: f32,

    #[uniform(5)]
    pub blur: f32,
}

//TODO: 没太懂
impl Material2d for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/background.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

///背景刷新
fn background_timer_tick(time: Res<Time>, mut materials: ResMut<Assets<BackgroundMaterial>>) {
    let dt = time.delta_secs();
    for (_, material) in materials.iter_mut() {
        material.offset1 += dt * material.speed1;
        material.offset2 += dt * material.speed2;
    }
}

///消除虚化
fn update_blur(
    screen: Res<State<Screen>>,
    time: Res<Time>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    let target = match screen.get() {
        Screen::Main => 3.0,
        Screen::Game => 0.0,
    };
    for (_, material) in materials.iter_mut() {
        let step = 3.0 * time.delta_secs();
        if material.blur < target {
            material.blur = (material.blur + step).min(target);
        } else if material.blur > target {
            material.blur = (material.blur - step).max(target);
        }
    }
}