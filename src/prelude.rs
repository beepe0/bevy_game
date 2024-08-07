pub(crate) mod inspector {
    //pub(crate) use crate::inspector::prelude::*;
}
pub(crate) mod player {
    pub(crate) use crate::game::player::prelude::*;
}
pub(crate) mod enemy {
    pub(crate) use crate::game::enemy::prelude::*;
}
pub(crate) mod color_palette {
    pub(crate) use crate::game::color_palette::*;
}
pub(crate) mod weapon {
    pub(crate) use crate::game::weapon::prelude::*;
}

pub(crate) use std::time::Duration;
pub(crate) use rand::Rng;
pub(crate) use bevy::{
    window::PrimaryWindow,
    math::{
        I64Vec2, 
        bounding::{
            IntersectsVolume, 
            Aabb2d
        }}, 
        prelude::*, 
        render::{
            settings::{
                Backends, 
                RenderCreation, 
                WgpuSettings
            }, 
            RenderPlugin
        }};
pub(crate) use bevy_inspector_egui::{
    quick::ResourceInspectorPlugin, 
    bevy_egui::{
        EguiContext, 
        EguiPlugin
    }, 
    egui
};