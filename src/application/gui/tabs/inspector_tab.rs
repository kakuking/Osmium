use egui_winit_vulkano::egui;

use crate::application::{ecs::{components::osmium_object::OsmiumObject, systems::osmium_object_system::OsmiumObjectSystem}, gui::editor_context::EditorContext};

pub fn draw_inspector_tab(
    ui: &mut egui::Ui,
    editor_context: &mut EditorContext,
) {
    let coordinator = &editor_context.coordinator;

    ui.heading("Inspector");
    ui.label("Nothing selected.");
    
    let active_entity = coordinator
        .get_system::<OsmiumObjectSystem>()
        .active_entity;

    if let Some(active_entity) = active_entity {
        let object = coordinator.get_component::<OsmiumObject>(active_entity);

        ui.label(format!("{}", object.name));
    }
}