use eframe::{App, Frame};
use eframe::egui::{CentralPanel, Color32, Context, Pos2};
use crate::paint::Canvas;

#[derive(Default)]
pub struct SNESPaintApp {
    canvas: Canvas,
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
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            ui.separator();
            self.canvas.update(ui);
            self.canvas.render(ui);
        });
    }
}