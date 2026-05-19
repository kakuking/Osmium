use egui_winit_vulkano::egui;

use crate::{
    application::{
        ecs::{
            components::osmium_object::OsmiumObject,
            systems::osmium_object_system::OsmiumObjectSystem,
        },
        gui::editor_context::EditorContext,
    },
    engine::ecs::{coordinator::Coordinator, Entity},
};

pub fn draw_scene_tab(
    ui: &mut egui::Ui,
    editor_context: &mut EditorContext,
) {
    ui.heading("Scene");

    let roots: Vec<Entity> = editor_context
        .coordinator
        .get_system::<OsmiumObjectSystem>()
        .roots
        .iter()
        .copied()
        .collect();

    for entity in roots {
        draw_object_node(ui, editor_context.coordinator, entity);
    }
}

fn draw_object_node(
    ui: &mut egui::Ui,
    coordinator: &mut Coordinator,
    entity: Entity,
) {
    let active_entity = coordinator
        .get_system::<OsmiumObjectSystem>()
        .active_entity
        .clone();

    let object = coordinator.get_component::<OsmiumObject>(entity);

    let selected = active_entity == Some(entity);

    let name = object.name.clone();
    let children = object.children.clone();

    let response = if !children.is_empty() {
        egui::CollapsingHeader::new(name)
        .id_source(entity)
        .default_open(true)
        .show(ui, |ui| {
            for child in children {
                draw_object_node(ui, coordinator, child);
            }
        }).header_response
    } else {
        ui.selectable_label(selected, name)
    };

    if response.clicked() {
        coordinator.get_system_mut::<OsmiumObjectSystem>().active_entity = Some(entity);
    }
}