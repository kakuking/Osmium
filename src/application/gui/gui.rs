use std::sync::Arc;

use egui_winit_vulkano::{
    egui,
    Gui,
    GuiConfig,
};
use vulkano::{
    command_buffer::SecondaryAutoCommandBuffer,
    render_pass::Subpass,
};
use winit::{
    event::WindowEvent,
    event_loop::EventLoop,
};

use crate::{
    application::gui::{
        editor_context::EditorContext,
        tabs::{
            tab::OsmiumTab,
            tab_viewer::OsmiumTabViewer,
        },
    },
    engine::{
        ecs::coordinator::Coordinator,
        renderer::renderer::Renderer,
        window::window_manager::WindowManager,
    },
};

pub struct OsmiumGUI {
    gui: Gui,
    dock_state: egui_dock::DockState<OsmiumTab>,
}

impl OsmiumGUI {
    pub fn new(
        event_loop: &EventLoop<()>,
        window_manager: &WindowManager,
        renderer: &Renderer,
    ) -> Self {
        let gui = Gui::new_with_subpass(
            event_loop,
            window_manager.get_surface(),
            renderer.vulkan_context.get_queue(),
            Subpass::from(renderer.render_pass.clone(), 1).unwrap(),
            renderer.swapchain_manager.get_image_format(),
            GuiConfig::default(),
        );

        Self {
            gui,
            dock_state: egui_dock::DockState::new(vec![
                OsmiumTab::Scene,
                OsmiumTab::Inspector,
            ]),
        }
    }

    pub fn update(&mut self, event: &WindowEvent) {
        self.gui.update(event);
    }

    pub fn generate_ui(&mut self, coordinator: &mut Coordinator) {
        let dock_state = &mut self.dock_state;

        self.gui.immediate_ui(|gui| {
            let ctx = gui.context();

            Self::draw_top_bar(&ctx);
            Self::draw_dock_area(&ctx, dock_state, coordinator);
        });
    }

    fn draw_top_bar(ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_bar")
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Toolbar");
                });
            });
    }

    fn draw_dock_area(
        ctx: &egui::Context,
        dock_state: &mut egui_dock::DockState<OsmiumTab>,
        coordinator: &mut Coordinator,
    ) {
        egui::SidePanel::left("left_dock_area")
            .resizable(true)
            .default_width(300.0)
            .show(ctx, |ui| {
                let editor_context = EditorContext {
                    coordinator,
                };

                let mut tab_viewer = OsmiumTabViewer {
                    editor_context,
                };

                egui_dock::DockArea::new(dock_state)
                    .show_inside(ui, &mut tab_viewer);
            });
    }

    pub fn render(
        &mut self,
        framebuffer_dimensions: [u32; 2],
    ) -> Arc<SecondaryAutoCommandBuffer> {
        self.gui.draw_on_subpass_image(framebuffer_dimensions)
    }

    pub fn wants_pointer_input(&self) -> bool {
        self.gui.context().wants_pointer_input()
    }

    pub fn wants_keyboard_input(&self) -> bool {
        self.gui.context().wants_keyboard_input()
    }
}