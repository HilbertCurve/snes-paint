use std::fmt::{Display, Formatter};
use crate::app::SnesPaintApp;

mod app;
mod paint;
mod serde;

#[derive(Debug)]
pub enum Error {
    InvalidCanvasSize(usize, usize),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error received!")
    }
}

impl std::error::Error for Error { }

fn main() -> Result<(), Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("SNES Paint", native_options, Box::new(|cc| Ok(Box::new(SnesPaintApp::new(cc))))).unwrap();
    Ok(())
}