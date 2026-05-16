#![allow(unused, non_camel_case_types, non_snake_case)]
pub mod constants;
pub mod resources;
pub mod entity;
pub mod ui;
pub mod states;

use std::default;
use crate::constants::*;
use crate::resources::*;
use crate::entity::*;
use crate::ui::*;
use crate::states::*;
use bevy::camera::Camera2d;
use bevy::camera::OrthographicProjection;
use bevy::camera::Projection;
use bevy::camera::ScalingMode;
use bevy::{asset::{Asset, AssetServer, Assets}, ecs::{component::Component, system::{Commands, Res, ResMut}}, gizmos::config::GizmoConfigStore, light::light_consts::lux::CLEAR_SUNRISE, math::Vec2, mesh::Mesh, sprite::Sprite, transform::components::Transform, utils::default};
