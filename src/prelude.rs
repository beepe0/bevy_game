pub(crate) mod inspector {
    //pub(crate) use crate::inspector::prelude::*;
}
pub(crate) mod bullet {
    pub(crate) use crate::bullet::prelude::*;
}
pub(crate) mod player {
    pub(crate) use crate::player::prelude::*;
}
pub(crate) mod enemy {
    pub(crate) use crate::enemy::prelude::*;
}
pub(crate) mod color_palette {
    pub(crate) use crate::color_palette::*;
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
    bevy_egui::EguiContext, egui
};



