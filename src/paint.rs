use eframe::egui::{Color32, Key, PointerButton, Rect, Response, Stroke, Ui, Widget};
use eframe::emath::Pos2;
use eframe::epaint::RectShape;

pub(crate) enum Palette {
    OneChannel([Color32;2]),
    TwoChannel([Color32;4]),
    ThreeChannel([Color32;8]),
    FourChannel([Color32;16]),
}

impl Default for Palette {
    fn default() -> Self {
        Self::new()
    }
}

impl Palette {
    pub fn new() -> Palette {
        Palette::OneChannel([Color32::WHITE, Color32::BLACK])
    }

    pub fn get_color(&self, idx: usize) -> Color32 {
        match &self {
            Palette::OneChannel(c) => c[idx],
            Palette::TwoChannel(c) => c[idx],
            Palette::ThreeChannel(c) => c[idx],
            Palette::FourChannel(c) => c[idx],
        }
    }

    pub(crate) fn set_color(&mut self, idx: usize, color: Color32) {
        match self {
            &mut Palette::OneChannel(ref mut c) => c[idx] = color,
            &mut Palette::TwoChannel(ref mut c) => c[idx] = color,
            &mut Palette::ThreeChannel(ref mut c) => c[idx] = color,
            &mut Palette::FourChannel(ref mut c) => c[idx] = color,
        }
    }

    pub fn get_count(&self) -> usize {
        match &self {
            Palette::OneChannel(_) => 2,
            Palette::TwoChannel(_) => 4,
            Palette::ThreeChannel(_) => 8,
            Palette::FourChannel(_) => 16,
        }
    }
}

pub(crate) enum CanvasGrid {
    Grid8x8([[usize;8];8]),
    Grid16x16([[usize;16];16]),
    Grid32x32([[usize;32];32]),
    Grid64x64([[usize;64];64]),
}

impl CanvasGrid {
    pub fn set_pixel(&mut self, row: usize, col: usize, idx: usize) {
        if row > self.size() || col > self.size() {
            panic!("Pixel set out of bounds! ({1}, {2}) out of bounds for {0}x{0} grid.",
                   self.size(), row, col);
        }

        match self {
            &mut CanvasGrid::Grid8x8(ref mut grid) => { grid[row][col] = idx; }
            &mut CanvasGrid::Grid16x16(ref mut grid) => { grid[row][col] = idx; }
            &mut CanvasGrid::Grid32x32(ref mut grid) => { grid[row][col] = idx; }
            &mut CanvasGrid::Grid64x64(ref mut grid) => { grid[row][col] = idx; }
        }
    }

    pub fn size(&self) -> usize {
        match &self {
            CanvasGrid::Grid8x8 { .. } => { 8 }
            CanvasGrid::Grid16x16 { .. } => { 16 }
            CanvasGrid::Grid32x32 { .. } => { 32 }
            CanvasGrid::Grid64x64 { .. } => { 64 }
        }
    }
}

impl Default for CanvasGrid {
    fn default() -> Self {
        Self::Grid8x8([[0;8];8])
    }
}


pub(crate) struct Canvas {
    pub(crate) palette: Palette,
    grid: CanvasGrid,
    pos: Pos2,
    pixel_width: u32,
    curr_idx: usize,
}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas {
            palette: Palette::default(),
            grid: CanvasGrid::default(),
            pos: Pos2::new(0.0, 0.0),
            pixel_width: 20,
            curr_idx: 0,
        }
    }

    pub fn set_pos(&mut self, pos: Pos2) {
        self.pos = pos;
    }

    pub fn render(&self, ui: &mut Ui) {
        for i in 0..self.grid.size() {
            for j in 0..self.grid.size() {
                ui.painter().add(RectShape {
                    rect: Rect {
                        min: (self.pos + Pos2::new(
                            i as f32 * self.pixel_width as f32,
                            j as f32 * self.pixel_width as f32,
                        ).to_vec2()).into(),
                        max: (self.pos + Pos2::new(
                            (i + 1) as f32 * self.pixel_width as f32,
                            (j + 1) as f32 * self.pixel_width as f32,
                        ).to_vec2()).into(),
                    },
                    rounding: Default::default(),
                    fill: self.get_pixel_color(i, j),
                    stroke: Stroke::new(1.0, Color32::BLACK),
                    blur_width: 0.0,
                    fill_texture_id: Default::default(),
                    uv: Rect::ZERO,
                });
            }
        }
    }

    pub fn update(&mut self, ui: &mut Ui) {
        // register click
        if ui.input(|i| i.pointer.button_clicked(PointerButton::Primary)) {
            let mut mouse_pos = Pos2::ZERO;
            ui.input(|i| mouse_pos = i.pointer.interact_pos().unwrap());
            let idx = (mouse_pos - self.pos) / self.pixel_width as f32;
            if idx.x < self.grid.size() as f32 && idx.y < self.grid.size() as f32 {
                self.grid.set_pixel(idx.x as usize, idx.y as usize, self.curr_idx);
            }
        }
        // switch palette
        if ui.input(|i| i.key_pressed(Key::ArrowRight)) {
            self.curr_idx += 1;
            if self.curr_idx >= self.palette.get_count() {
                self.curr_idx = 0;
            }
        }
    }

    pub fn get_pixel_color(&self, row: usize, col: usize) -> Color32 {
        match &self.grid {
            CanvasGrid::Grid8x8(grid) => {
                self.palette.get_color(grid[row][col])
            },
            CanvasGrid::Grid16x16(grid) => {
                self.palette.get_color(grid[row][col])
            },
            CanvasGrid::Grid32x32(grid) => {
                self.palette.get_color(grid[row][col])
            },
            CanvasGrid::Grid64x64(grid) => {
                self.palette.get_color(grid[row][col])
            },
        }
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}