pub(crate) mod prelude {
    //pub(super) use super::;
}

use crate::prelude::*;

#[derive(Reflect, Resource)]
struct ConsoleConfiguration {
    title: String,
    collapsible: bool,
    resizeble: bool,
    movable: bool,
    title_bar: bool,
    default_pos: Vec2,
    default_size: Vec2
}

impl Default for ConsoleConfiguration {
    fn default() -> Self {
        Self { 
            title: String::from("Console"), 
            collapsible: true, 
            resizeble: false, 
            movable: true,
            title_bar: true, 
            default_pos: Vec2 {x: 0f32, y: 0f32}, 
            default_size: Vec2 {x: 600f32, y: 400f32} 
        }
    }
}

#[derive(Reflect, Default, Resource)]
struct ConsoleState {
    is_open: bool,
    content_buffer: String,
    input_buffer: String,
}

impl ConsoleState {
    fn submit_input_field(&mut self) {
        self.content_buffer.push_str(&format!("> {}\n", self.input_buffer.as_str()));
        self.input_buffer.clear();
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<ConsoleState>();
    app.init_resource::<ConsoleConfiguration>();
    app.register_type::<ConsoleState>();
    app.register_type::<ConsoleConfiguration>();
    // app.add_plugins(ResourceInspectorPlugin::<ConsoleConfiguration>::new());
    // app.add_plugins(ResourceInspectorPlugin::<ConsoleState>::new());
    app.add_plugins(EguiPlugin);
    app.add_systems(Update, (update_entities_inspector_ui, update_console_ui));
}

fn update_entities_inspector_ui(
    world: &mut World
) {    
    let Ok(egui_context) = world.query_filtered::<&mut EguiContext, With<PrimaryWindow>>().get_single(world) else { return; };
    let mut egui_context = egui_context.clone();
    
    egui::Window::new("Entities").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);
        });
    });
}

fn update_console_ui(
    mut console_state: ResMut<ConsoleState>,
    console_resource: Res<ConsoleConfiguration>,
    egui_context: Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    let Ok(egui_context) = egui_context.get_single() else { return; };
    let mut egui_context = egui_context.clone();
    
    egui::Window::new(console_resource.title.as_str())
        .collapsible(console_resource.collapsible)
        .resizable(console_resource.resizeble)
        .movable(console_resource.movable)
        .title_bar(console_resource.title_bar)
        .default_pos(console_resource.default_pos.to_array())
        .default_size(console_resource.default_size.to_array())
        .show(egui_context.get_mut(), |ui| {
            ui.style_mut().visuals.extreme_bg_color = egui::Color32::BLACK;
            
            ui.vertical(|ui| {
                let scroll_height = ui.available_height() - 150f32;

                egui::ScrollArea::vertical()
                    .auto_shrink(false)
                    .stick_to_bottom(true)
                    .max_height(scroll_height)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.style_mut().visuals.override_text_color = Some(egui::Color32::GOLD);
                            ui.label(console_state.content_buffer.clone());        
                        });
                    });

                ui.separator();
                
                ui.horizontal(|ui| {
                    let input_field = egui::TextEdit::singleline(&mut console_state.input_buffer)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY);

                    let input_field_response = ui.add(input_field);

                    if input_field_response.lost_focus() && ui.input(|is| is.key_pressed(egui::Key::Enter)) {
                        console_state.submit_input_field();
                    }
                });
            });
    });
}