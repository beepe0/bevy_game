pub(crate) mod prelude {
    //pub(super) use super::;
}

use crate::prelude::*;

struct Console {
    input: String,
}

pub fn plugin(app: &mut App) {
    app.add_plugins(EguiPlugin);
    app.add_systems(Startup, startup);
    app.add_systems(Update, (entities_inspector_ui, console_ui));
}

fn startup() {

}

fn entities_inspector_ui(world: &mut World) {    
    let Ok(egui_context) = world.query_filtered::<&mut EguiContext, With<PrimaryWindow>>().get_single(world) else { return; };
    let mut egui_context = egui_context.clone();
    
    egui::Window::new("Entities").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);
        });
    });
}

fn console_ui(world: &mut World) {
    let Ok(egui_context) = world.query_filtered::<&mut EguiContext, With<PrimaryWindow>>().get_single(world) else { return; };
    let mut egui_context = egui_context.clone();

    egui::Window::new("Console").show(egui_context.get_mut(), |ui| {
        let input_field_id = egui::Id::new("game_console_input_field");
        let grid_id = egui::Id::new("game_console_grid");

        let grid = egui::Grid::new(grid_id);

        let mut string = ui.memory_mut(|mem| {
            let string: &mut String = mem.data.get_persisted_mut_or_default(input_field_id);
            string.clone()
        });

        ui.text_edit_singleline(&mut string);

        ui.memory_mut(|mem| {
            *mem.data.get_persisted_mut_or_default(input_field_id) = string.clone();
        });

        if ui.button("submit").clicked() {
            ui.label("asd");    
        }
        egui::ScrollArea::vertical().show(ui, |ui| {
   
            ui.label("asd");

         
        });
    });
}