mod new_file_modal;
mod palette;

use crate::change_manager::ChangeManager;
use crate::file_interactions::FileInteractions;
use crate::model::Canvas;
use egui::*;
use new_file_modal::*;
use palette::*;

const CAT_FLAVOR: catppuccin::Flavor = catppuccin::PALETTE.frappe;
const UNDO_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::CTRL, Key::Z);
const REDO_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::CTRL, Key::Y);

#[derive(PartialEq, Debug, Default)]
enum Tool {
    #[default]
    Draw,
    Erase,
    Fill,
    Rectangle(Option<Pos2>),
    Eyedrop,
}

// impl Default for Tool {
//     fn default() -> Self {
//         Tool::Draw(0)
//     }
// }

pub struct Ruxel {
    active_color: eframe::egui::Color32,
    color_pallete: Vec<eframe::egui::Color32>,
    color_pallete_edit: bool,
    canvas: Option<Canvas>,
    active_tool: Tool,
    tool_size: usize,
    file_interactions: FileInteractions,
    new_file_modal: NewFileModal,
    change_manager: Option<ChangeManager>,
}

impl Default for Ruxel {
    fn default() -> Self {
        Self {
            active_color: eframe::egui::Color32::from_rgb(255, 255, 255),
            color_pallete: get_color_palette(CAT_FLAVOR),
            color_pallete_edit: false,
            canvas: None,
            active_tool: Tool::default(),
            tool_size: 1,
            file_interactions: FileInteractions::new(),
            new_file_modal: NewFileModal::default(),
            change_manager: None,
        }
    }
}

impl Ruxel {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self::default()
    }
}

impl Ruxel {
    fn menu_selection(&mut self, ui: &mut eframe::egui::Ui) {
        egui::TopBottomPanel::top("Menu")
            .resizable(false)
            .min_height(32.0)
            .show_inside(ui, |ui| {
                ui.horizontal_centered(|ui| {
                    egui::menu::bar(ui, |ui| {
                        let _file_response = ui.menu_button("File", |ui| {
                            if ui.button("Save").clicked() {
                                self.file_interactions.show_save_dialog = true;
                            }

                            if self.file_interactions.show_save_dialog {
                                let rgba_buff =
                                    Canvas::get_rgba_buffer(self.canvas.as_ref().unwrap());

                                FileInteractions::save_file(&mut self.file_interactions, rgba_buff);
                            }

                            if ui.button("New").clicked() {
                                self.new_file_modal.activate();
                            }

                            if ui.button("Close").clicked() {
                                std::process::exit(0);
                            }
                        });

                        let _edit_response = ui.menu_button("Edit", |ui| {
                            let undo_response = ui.button("Undo");
                            if undo_response.clicked() {
                                match &mut self.canvas {
                                    Some(c) => {
                                        if let Some(cm) = &mut self.change_manager {
                                            if !cm.is_undo_empty() {
                                                let previous_state = cm.undo();

                                                c.load_state(previous_state);
                                            }
                                        }
                                    }
                                    None => {}
                                }
                            }
                            undo_response.on_hover_text("Ctrl+Z");

                            let redo_response = ui.button("Redo");
                            if redo_response.clicked() {
                                match &mut self.canvas {
                                    Some(c) => {
                                        if let Some(cm) = &mut self.change_manager {
                                            if !cm.is_redo_empty() {
                                                let next_state = cm.redo();

                                                c.load_state(next_state);
                                            }
                                        }
                                    }
                                    None => {}
                                }
                            }
                            redo_response.on_hover_text("Ctrl+Y");
                        });

                        // check for Ctrl+Z and Ctrl+Y
                        ui.input_mut(|i| {
                            if i.consume_shortcut(&UNDO_SHORTCUT) {
                                match &mut self.canvas {
                                    Some(c) => {
                                        if let Some(cm) = &mut self.change_manager {
                                            if !cm.is_undo_empty() {
                                                let previous_state = cm.undo();

                                                c.load_state(previous_state);
                                            }
                                        }
                                    }
                                    None => {}
                                }
                            } else if i.consume_shortcut(&REDO_SHORTCUT) {
                                match &mut self.canvas {
                                    Some(c) => {
                                        if let Some(cm) = &mut self.change_manager {
                                            if !cm.is_redo_empty() {
                                                let next_state = cm.redo();

                                                c.load_state(next_state);
                                            }
                                        }
                                    }
                                    None => {}
                                }
                            }
                        });
                    });
                });
            });
    }

    fn tool_selection(&mut self, ui: &mut eframe::egui::Ui) {
        egui::TopBottomPanel::top("Tools")
            .resizable(true)
            .min_height(32.0)
            .show_inside(ui, |ui| {
                ui.horizontal_centered(|ui| {
                    egui::menu::bar(ui, |ui| {
                        let draw_response =
                            ui.selectable_value(&mut self.active_tool, Tool::Draw, "Draw");
                        if draw_response.clicked() {
                            self.active_tool = Tool::Draw;
                            println!("Current tool {:?}", self.active_tool);
                        }
                        let erase_response =
                            ui.selectable_value(&mut self.active_tool, Tool::Erase, "Erase");
                        if erase_response.clicked() {
                            self.active_tool = Tool::Erase;
                            println!("Current tool {:?}", self.active_tool);
                        }
                        let fill_response =
                            ui.selectable_value(&mut self.active_tool, Tool::Fill, "Fill");
                        if fill_response.clicked() {
                            self.active_tool = Tool::Fill;
                            println!("Current tool {:?}", self.active_tool);
                        }
                        let shape_response = ui.selectable_value(
                            &mut self.active_tool,
                            Tool::Rectangle(None),
                            "Rectangle",
                        );
                        if shape_response.clicked() {
                            self.active_tool = Tool::Rectangle(None);
                            println!("Current tool {:?}", self.active_tool);
                        }

                        let eyedrop_response =
                            ui.selectable_value(&mut self.active_tool, Tool::Eyedrop, "Eyedrop");
                        if eyedrop_response.clicked() {
                            self.active_tool = Tool::Eyedrop;
                            println!("Current tool {:?}", self.active_tool);
                        }
                        ui.add(egui::Slider::new(&mut self.tool_size, 1..=50).suffix("px"));
                    });
                });
            });
    }

    fn color_buttons(&mut self, ui: &mut eframe::egui::Ui) {
        let width = ui.available_width();
        let height = width / 2.0;
        if self.color_pallete_edit {
            ui.spacing_mut().interact_size = Vec2 {
                x: height,
                y: height,
            };
            let mut i = 0;
            let mut len = self.color_pallete.len();
            while i < len {
                ui.horizontal(|ui| {
                    let _response = ui.color_edit_button_srgba(&mut self.color_pallete[i]);
                    let button = ui.add_sized(ui.available_size(), egui::Button::new("-"));
                    if button.clicked() {
                        self.color_pallete.remove(i);
                        len -= 1;
                    } else {
                        i += 1;
                    }
                });
            }
            let button = ui.add_sized([width, height], egui::Button::new("+"));
            if button.clicked() {
                self.color_pallete
                    .push(eframe::egui::Color32::from_rgb(200, 200, 200));
            }
        } else {
            self.color_pallete.iter_mut().for_each(|color| {
                let button = ui.add_sized(
                    [width, height],
                    egui::Button::new("")
                        .fill(*color)
                        .stroke(ui.visuals().window_stroke()),
                );
                if button.clicked() {
                    self.active_color = *color;
                }
            });
        }
    }

    fn color_selection(&mut self, ui: &mut eframe::egui::Ui) {
        egui::SidePanel::left("Colors")
            .resizable(true)
            .default_width(50.0)
            .width_range(50.0..=200.0)
            .show_inside(ui, |ui| {
                // NOTE TO THE PROFESSOR: Good Lord! This took way longer than it should
                // have! Trying to convince this annoying little bugger to conform to the
                // size I wanted was miserable, and the fact that this is the actual way it
                // is supposed to be done is criminal!
                ui.spacing_mut().interact_size = Vec2 {
                    x: ui.available_width(),
                    y: ui.available_width() / 2.0,
                };
                let _response = ui.color_edit_button_srgba(&mut self.active_color);

                ui.separator();

                egui::TopBottomPanel::bottom("Edit/Save")
                    .resizable(false)
                    .exact_height(ui.available_width() / 2.0)
                    .show_inside(ui, |ui| {
                        ui.add_space(ui.spacing().item_spacing.y);
                        let button = ui.add_sized(
                            ui.available_size(),
                            if self.color_pallete_edit {
                                egui::Button::new("Save")
                            } else {
                                egui::Button::new("Edit")
                            },
                        );
                        if button.clicked() {
                            self.color_pallete_edit = !self.color_pallete_edit;
                        }
                    });

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        self.color_buttons(ui);
                    });
                });
            });
    }

    fn layer_selection(&mut self, ui: &mut eframe::egui::Ui) {
        egui::SidePanel::right("Layers")
            .resizable(true)
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Layers");

                        // Need to check if canvas is opened yet
                        match &mut self.canvas {
                            Some(c) => {
                                let num_layers = c.get_num_layers();
                                let active_layer = c.get_active_layer();

                                // Iterate over layers
                                for i in 0..num_layers {
                                    ui.horizontal(|ui| {
                                        let button = egui::Button::new(c.get_layer_name(i))
                                            .selected(i == active_layer);

                                        let response = ui.add_sized(
                                            [ui.available_width() / 1.25, ui.available_height()],
                                            button,
                                        );
                                        if response.clicked() {
                                            c.set_active_layer(i);
                                        }

                                        let layers_to_show = c.get_layers_to_show_mut();
                                        let to_show = layers_to_show.get_mut(i).unwrap();

                                        let checkbox = egui::Checkbox::new(to_show, "");

                                        ui.add_sized(ui.available_size(), checkbox);
                                    });
                                }

                                ui.separator();

                                // Add "+" and "-" button
                                ui.horizontal(|ui| {
                                    let plus_button = ui.add_sized(
                                        [ui.available_width() / 2.0, ui.available_height()],
                                        egui::Button::new("+"),
                                    );
                                    if plus_button.clicked() {
                                        c.add_layer();
                                        self.change_manager
                                            .as_mut()
                                            .unwrap()
                                            .push_new_state(c.create_state());
                                    }

                                    let minus_button = ui.add_sized(
                                        [ui.available_width(), ui.available_height()],
                                        egui::Button::new("-"),
                                    );
                                    if minus_button.clicked() {
                                        c.delete_layer(active_layer);
                                        self.change_manager
                                            .as_mut()
                                            .unwrap()
                                            .push_new_state(c.create_state());
                                    }
                                });
                            }
                            None => (),
                        }
                    });
                });
            });
    }

    fn canvas_ui(&mut self, ui: &mut eframe::egui::Ui) {
        CentralPanel::default().show_inside(ui, |ui| {
            if let Some(c) = &mut self.canvas {
                Frame::canvas(ui.style()).show(ui, |ui| {
                    let (response, painter) =
                        ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

                    // Center
                    let clip = painter.clip_rect();
                    let screen_center = Pos2 {
                        x: clip.min.x + (clip.max.x - clip.min.x) / 2.0,
                        y: clip.min.y + (clip.max.y - clip.min.y) / 2.0,
                    };
                    if screen_center != *c.get_screen_center() {
                        *c.get_screen_center_mut() = screen_center;
                        c.update_squares();
                    }

                    let mut squares = c.get_squares_unhidden();
                    let active_layer = c.get_active_layer();

                    if response.dragged_by(PointerButton::Middle) {
                        c.scroll(&response.drag_delta());
                    } else {
                        ui.ctx().input(|i| {
                            for event in &i.events {
                                match event {
                                    Event::Zoom(amount) => c.zoom(amount),
                                    Event::Scroll(amount) => c.scroll(amount),
                                    _ => {}
                                }
                            }
                            match self.active_tool {
                                Tool::Draw => {
                                    if let (true, Some(pointer_pos)) = (
                                        response.clicked() || response.dragged(),
                                        response.interact_pointer_pos(),
                                    ) {
                                        c.set_pixels_from_brush(
                                            &pointer_pos,
                                            self.tool_size,
                                            self.active_color,
                                        );
                                    } else if let Some(pointer_pos) = i.pointer.latest_pos() {
                                        squares.extend(
                                            c.get_circle_brush(&pointer_pos, self.tool_size),
                                        );
                                    }
                                    if response.drag_stopped() {
                                        self.change_manager
                                            .as_mut()
                                            .unwrap()
                                            .push_new_state(c.create_state());
                                    }
                                }
                                Tool::Erase => {
                                    if let (true, Some(pointer_pos)) = (
                                        response.clicked() || response.dragged(),
                                        response.interact_pointer_pos(),
                                    ) {
                                        c.set_pixels_from_brush(
                                            &pointer_pos,
                                            self.tool_size,
                                            Color32::TRANSPARENT,
                                        );
                                        squares.extend(
                                            c.get_circle_brush(&pointer_pos, self.tool_size),
                                        );
                                    } else if let Some(pointer_pos) = i.pointer.latest_pos() {
                                        squares.extend(
                                            c.get_circle_brush(&pointer_pos, self.tool_size),
                                        );
                                    }
                                    if response.drag_stopped() {
                                        self.change_manager
                                            .as_mut()
                                            .unwrap()
                                            .push_new_state(c.create_state());
                                    }
                                }
                                Tool::Fill => {
                                    if let (true, Some(pointer_pos)) =
                                        (response.drag_started(), response.interact_pointer_pos())
                                    {
                                        if let Some(t) =
                                            c.get_pixel_from_screen_cords(pointer_pos, active_layer)
                                        {
                                            let target_color = *t;
                                            if c.fill(
                                                &pointer_pos,
                                                &target_color,
                                                &self.active_color,
                                            )
                                            .is_ok()
                                            {
                                                self.change_manager
                                                    .as_mut()
                                                    .unwrap()
                                                    .push_new_state(c.create_state())
                                            }
                                        }
                                    }
                                }
                                Tool::Rectangle(start) => {
                                    if let (true, None, Some(pointer_pos)) = (
                                        response.clicked() || response.dragged(),
                                        start,
                                        response.interact_pointer_pos(),
                                    ) {
                                        self.active_tool = Tool::Rectangle(Some(pointer_pos));
                                        squares.push(c.get_rect_brush(&pointer_pos, &pointer_pos));
                                    } else if let Some(pointer_pos) = i.pointer.latest_pos() {
                                        let start_pos = start.unwrap_or(pointer_pos);
                                        squares.push(c.get_rect_brush(&start_pos, &pointer_pos));
                                    }
                                    if let (true, Some(start_pos), Some(end_pos)) =
                                        (response.drag_stopped(), start, i.pointer.latest_pos())
                                    {
                                        self.active_tool = Tool::Rectangle(None);
                                        c.set_pixels_from_rect_brush(
                                            &start_pos,
                                            &end_pos,
                                            self.active_color,
                                        );

                                        self.change_manager
                                            .as_mut()
                                            .unwrap()
                                            .push_new_state(c.create_state());
                                    }
                                }
                                Tool::Eyedrop => {
                                    if let (true, Some(pointer_pos)) = (
                                        response.clicked() || response.dragged(),
                                        response.interact_pointer_pos(),
                                    ) {
                                        if let Some(color) =
                                            c.get_pixel_from_screen_cords(pointer_pos, active_layer)
                                        {
                                            match *color {
                                                Color32::TRANSPARENT => {}
                                                _ => self.active_color = *color,
                                            }
                                        }
                                    }
                                }
                            }
                        });
                    }

                    // Draw shapes
                    painter.extend(squares);
                });
            } else {
                ui.vertical_centered(|ui| {
                    let (width, height) = (250.0, 100.0);
                    let button = ui.add_sized(
                        [width, height],
                        egui::Button::new("New").stroke(ui.visuals().window_stroke()),
                    );
                    if button.clicked() {
                        self.new_file_modal.activate();
                    }
                });
            }
        });
    }

    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        self.menu_selection(ui);
        self.color_selection(ui);
        self.layer_selection(ui);
        self.tool_selection(ui);
        self.canvas_ui(ui);
    }

    fn new_file_modal_window(&mut self, ctx: &Context) {
        egui::Window::new("New File")
            .open(&mut self.new_file_modal.show_modal_toggle)
            .collapsible(false)
            .resizable(false)
            .pivot(Align2::CENTER_CENTER)
            .fixed_pos(ctx.screen_rect().center())
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add(
                        egui::DragValue::new(&mut self.new_file_modal.width)
                            .speed(1)
                            .clamp_range(1..=128)
                            .suffix("px"),
                    );
                    ui.label("Width")
                });
                ui.horizontal(|ui| {
                    ui.add(
                        egui::DragValue::new(&mut self.new_file_modal.height)
                            .speed(1)
                            .clamp_range(1..=128)
                            .suffix("px"),
                    );
                    ui.label("Height")
                });

                ui.add_space(ui.spacing().item_spacing.y);

                ui.horizontal(|ui| {
                    let button = ui.add_sized(
                        [100.0, 30.0],
                        egui::Button::new("Cancel").stroke(ui.visuals().window_stroke()),
                    );
                    if button.clicked() {
                        self.new_file_modal.show_modal = false;
                    }
                    let button = ui.add_sized(
                        [100.0, 30.0],
                        egui::Button::new("Create").stroke(ui.visuals().window_stroke()),
                    );
                    if button.clicked() {
                        let canvas =
                            Canvas::new(self.new_file_modal.width, self.new_file_modal.height);
                        self.change_manager = Some(ChangeManager::new(canvas.create_state()));
                        self.canvas = Some(canvas);
                        self.new_file_modal.show_modal = false;
                    }
                });
            });
    }
}

impl eframe::App for Ruxel {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.new_file_modal.is_active() {
            self.new_file_modal_window(ctx)
        }

        ctx.set_visuals(visuals(CAT_FLAVOR, ctx.style().visuals.clone()));
        eframe::egui::CentralPanel::default().show(ctx, |ui| self.ui(ui));
    }
}
