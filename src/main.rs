mod prelude;
mod player;
mod bullet;
mod enemy;

use std::iter::{Filter, Map};

use bevy::{ecs::query, window::PrimaryWindow};
use bevy_inspector_egui::{bevy_egui::{EguiContext, EguiPlugin}, egui};

use crate::prelude::*;

pub fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(RenderPlugin { render_creation: RenderCreation::Automatic(WgpuSettings {backends: Some(Backends::VULKAN), ..Default::default()}), ..Default::default()}));
    app.add_plugins(EguiPlugin);
    app.add_systems(Update, inspector_ui);

    Player::main(&mut app);
    Enemy::main(&mut app);
    Bullet::main(&mut app);

    app.run();

}

fn inspector_ui(world: &mut World) {
    let Ok(egui_context) = world.query_filtered::<&mut EguiContext, With<PrimaryWindow>>().get_single(world)
        
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // equivalent to `WorldInspectorPlugin`
            //bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);

            // egui::CollapsingHeader::new("Materials").show(ui, |ui| {
            //     bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
            // });

            ui.heading("Entities");
            bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);
        });
    });
}

