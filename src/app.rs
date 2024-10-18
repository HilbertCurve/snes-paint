use eframe::{App, Frame};
use eframe::egui::{CentralPanel, Color32, Context, Id, Pos2, SidePanel, TextEdit};
use crate::Error;
use crate::paint::Canvas;

#[derive(Default)]
pub struct SNESPaintApp {
    canvas: Canvas,
    side_bar: SideBar,
}

#[derive(Default, PartialEq, PartialOrd)]
pub enum SideBarType {
    #[default]
    File,
    Canvas,
    // ...
}

#[derive(Default)]
pub struct SideBar {
    side_bar_type: SideBarType,
    canvas_width_field: String,
    canvas_height_field: String,
}

impl SNESPaintApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> SNESPaintApp {
        let mut app = Self::default();
        app.canvas.palette.set_color(1, Color32::BLACK);
        app.canvas.set_pos(Pos2::new(50.0, 50.0));
        app
    }
}

impl App for SNESPaintApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        SidePanel::right(Id::new("SidePanel")).min_width(200.0).max_width(300.0).show(ctx, |ui| {
            ui.separator();
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.side_bar.side_bar_type, SideBarType::File, "File");
                ui.selectable_value(&mut self.side_bar.side_bar_type, SideBarType::Canvas, "Canvas");
            });
            ui.separator();

            match self.side_bar.side_bar_type {
                SideBarType::Canvas => {
                    // value for changing texture format
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
                    // value for changing canvas size
                },
                SideBarType::File => {
                    // Save file
                    // Load file
                }
            }
        });
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            ui.separator();
            ui.horizontal(|ui| {
                self.canvas.update(ui);
                self.canvas.render(ui);
                let idx = self.canvas.curr_idx;
                ui.color_edit_button_srgba(self.canvas.get_palette_mut().get_color_mut(idx));
            });
            ui.separator();
        });
    }
}