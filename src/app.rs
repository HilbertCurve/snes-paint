//! Application rendering and state management
//!
//! Keybindings:
//!
//! alt+c: switch sidebar to canvas mode
//! alt+f: switch sidebar to file mode
//! tab: cycle palette forwards
//! shift+tab: cycle palette backwards
//! TODO: MORE!

use std::fs;
use eframe::{App, Frame};
use eframe::egui::{CentralPanel, Color32, ComboBox, Context, Id, Pos2, Sense, SidePanel, TextEdit};
use crate::paint::Canvas;

pub mod shortcut {
    use eframe::egui::{Key, KeyboardShortcut, Modifiers};

    pub(crate) const PALETTE_FORWARD: KeyboardShortcut = KeyboardShortcut::new(Modifiers::SHIFT, Key::J);
    pub(crate) const PALETTE_BACKWARD: KeyboardShortcut = KeyboardShortcut::new(Modifiers::SHIFT, Key::K);
    pub(crate) const SIDEBAR_FILE: KeyboardShortcut = KeyboardShortcut::new(Modifiers::ALT, Key::F);
    pub(crate) const SIDEBAR_CANVAS: KeyboardShortcut = KeyboardShortcut::new(Modifiers::ALT, Key::C);
    pub(crate) const CANVAS_SIZE_FIELD: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::I);
}

pub mod action {
    use eframe::egui::{Key, KeyboardShortcut, Modifiers};

    pub(crate) const CURSOR_LEFT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::L);
    pub(crate) const CURSOR_RIGHT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::H);
    pub(crate) const CURSOR_UP: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::K);
    pub(crate) const CURSOR_DOWN: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::J);
    pub(crate) const CURSOR_PAINT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F);
}

#[derive(Default)]
pub struct SnesPaintApp {
    canvas: Canvas,
    side_bar: SideBar,
}

#[derive(Default, PartialEq, PartialOrd)]
pub enum SideBarType {
    #[default]
    File,
    Canvas,
    Layer,
    // ...
}

#[derive(Default)]
pub struct SideBar {
    side_bar_type: SideBarType,
    canvas_width_field: String,
    canvas_height_field: String,
}

impl SnesPaintApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> SnesPaintApp {
        let mut app = Self::default();
        app.canvas.palette.set_color(1, Color32::BLACK);
        app.canvas.set_pos(Pos2::new(50.0, 50.0));
        app
    }
}

impl App for SnesPaintApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        SidePanel::right(Id::new("SidePanel")).min_width(200.0).max_width(300.0).show(ctx, |ui| {
            ui.separator();
            // display menu bar for selecting functions
            ui.horizontal(|ui| {
                let file_hover = ui.selectable_value(
                    &mut self.side_bar.side_bar_type,
                    SideBarType::File,
                    "File"
                ).interact(Sense::hover());
                if let Some(_) = file_hover.hover_pos() {
                    file_hover.show_tooltip_text("alt+f");
                }

                let canvas_hover = ui.selectable_value(
                    &mut self.side_bar.side_bar_type,
                    SideBarType::Canvas,
                    "Canvas"
                ).interact(Sense::hover());
                if let Some(_) = canvas_hover.hover_pos() {
                    canvas_hover.show_tooltip_text("alt+c");
                }
            });
            ui.separator();

            if ui.input_mut(|mut i| i.consume_shortcut(&shortcut::SIDEBAR_FILE)) {
                self.side_bar.side_bar_type = SideBarType::File;
            }
            if ui.input_mut(|mut i| i.consume_shortcut(&shortcut::SIDEBAR_CANVAS)) {
                self.side_bar.side_bar_type = SideBarType::Canvas;
            }

            // depending on selected menu bar, select certain functionality
            match self.side_bar.side_bar_type {
                SideBarType::Canvas => {
                    // field for changing grid size
                    ui.horizontal(|ui| {
                        ui.label("Size:");
                        let mut width = &mut self.side_bar.canvas_width_field;
                        ui.add(TextEdit::singleline(width).desired_width(25.0));
                        let w_set = width.clone();

                        ui.label("x");

                        let mut height = &mut self.side_bar.canvas_height_field;
                        ui.add(TextEdit::singleline(height).desired_width(25.0));
                        let h_set = height.clone();

                        if ui.button("Apply").clicked() {
                            match self.canvas.set_size(
                                usize::from_str_radix(w_set.as_str(), 10).unwrap(),
                                usize::from_str_radix(h_set.as_str(), 10).unwrap(),
                            ) {
                                Ok(_) => {}
                                Err(_) => {println!("Change this to label!!!")}
                            };
                        }
                    });
                    // field for changing palette type
                    ui.horizontal(|ui| {
                        let mut current_bpp = self.canvas.palette.bpp();
                        ComboBox::from_label("Palette Size:")
                            .selected_text(self.canvas.palette.to_string())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut current_bpp, 2, "2 BPP (4 colors)");
                                ui.selectable_value(&mut current_bpp, 3, "3 BPP (8 colors)");
                                ui.selectable_value(&mut current_bpp, 4, "4 BPP (16 colors)");
                                ui.selectable_value(&mut current_bpp, 8, "8 BPP (256 colors)");
                            }
                        );

                        if current_bpp != self.canvas.palette.bpp() {
                            self.canvas.palette.set_bpp(current_bpp);
                        }
                    });
                },
                SideBarType::File => {
                    // Save file
                    if ui.button("Test BPPS Stuff!").clicked() {
                        let serialized = self.canvas.serialize();
                        println!("Canvas VRAM data:");
                        for c in serialized.0.chunks(2) {
                            println!("{:b} {:b}", c[0], c[1]);
                        }
                        println!("Palette data:");
                        for c in serialized.1.chunks(2) {
                            println!("{:X} {:X}", c[0], c[1]);
                        }
                    }
                    if ui.button("Save...").clicked() {
                        let serialized = self.canvas.serialize();
                        let file = rfd::FileDialog::new().save_file();
                        if let Some(file) = file {
                            fs::write(file, serialized.0).unwrap();
                        }
                    }
                    if ui.button("Save Palette...").clicked() {
                        let serialized = self.canvas.serialize();
                        let file = rfd::FileDialog::new().save_file();
                        if let Some(file) = file {
                            fs::write(file, serialized.1).unwrap();
                        }
                    }
                    // Load file
                }
                _ => {}
            }
        });
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            ui.separator();
            ui.horizontal(|ui| {
                self.canvas.update(ui);
                self.canvas.render(ui);
                let idx = self.canvas.color_idx;
                ui.color_edit_button_srgba(self.canvas.get_palette_mut().get_color_mut(idx));
            });
            ui.separator();
        });
    }
}