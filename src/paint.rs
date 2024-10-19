use std::ops::Index;
use eframe::egui::{Color32, PointerButton, Rect, Rgba, Rounding, Stroke, Ui, Vec2};
use eframe::emath::Pos2;
use eframe::epaint::RectShape;
use crate::app::action;
use crate::Error;

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

    pub fn get_color_mut(&mut self, idx: usize) -> &mut Color32 {
        match self {
            &mut Palette::OneChannel(ref mut c) => &mut c[idx],
            &mut Palette::TwoChannel(ref mut c) => &mut c[idx],
            &mut Palette::ThreeChannel(ref mut c) => &mut c[idx],
            &mut Palette::FourChannel(ref mut c) => &mut c[idx],
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

    pub fn size(&self) -> usize {
        match &self {
            Palette::OneChannel(_) => 2,
            Palette::TwoChannel(_) => 4,
            Palette::ThreeChannel(_) => 8,
            Palette::FourChannel(_) => 16,
        }
    }
}

pub(crate) struct CanvasGrid<const W: usize, const H: usize> {
    grid: [[usize;W];H]
}

impl<const W: usize, const H: usize> CanvasGrid<W, H> {
    pub fn set_pixel(&mut self, row: usize, col: usize, idx: usize) {
        if row > W || col > H {
            panic!("Pixel set out of bounds! ({row}, {col}) out of bounds for {W}x{H} grid.");
        }

        self.grid[row][col] = idx;
    }

    #[inline]
    pub const fn width(&self) -> usize { W }
    #[inline]
    pub const fn height(&self) -> usize { H }
}

/// A trait for being a two-dimensional grid of elements.
trait Grid<T: Clone> {
    fn get(&self, row: usize, col: usize) -> T;
    fn set(&mut self, row: usize, col: usize, v: T);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

macro_rules! impl_grid_on_canvas {
    ($w:expr, $h:expr) => {
        impl Grid<usize> for CanvasGrid<$w, $h> {
            fn get(&self, row: usize, col: usize) -> usize {
                self.grid[row][col]
            }
            fn set(&mut self, row: usize, col: usize, v: usize) {
                self.grid[row][col] = v;
            }
            fn width(&self) -> usize {
                $w
            }
            fn height(&self) -> usize {
                $h
            }
        }
    };
}

impl_grid_on_canvas!(8, 8);
impl_grid_on_canvas!(16, 16);
impl_grid_on_canvas!(32, 32);
impl_grid_on_canvas!(64, 64);

impl<const W: usize, const H: usize> Index<usize> for CanvasGrid<W, H> {
    type Output = [usize];

    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[index]
    }
}

impl<const W: usize, const H: usize> Default for CanvasGrid<W, H> {
    fn default() -> Self {
        CanvasGrid {
            grid: [[0usize;W];H]
        }
    }
}

pub(crate) struct Canvas {
    pub(crate) palette: Palette,
    grid: Box<dyn Grid<usize>>,
    pos: Pos2,
    cursor: (usize, usize),
    pixel_width: u32,
    pub(crate) color_idx: usize,
}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas {
            palette: Palette::default(),
            grid: Box::new(CanvasGrid::<8, 8>::default()),
            pos: Pos2::new(0.0, 0.0),
            cursor: Default::default(),
            pixel_width: 20,
            color_idx: 0,
        }
    }

    pub(crate) fn set_size(&mut self, width: usize, height: usize) -> Result<(), Error> {
        if width == self.grid.width() && height == self.grid.height() {
            return Ok(());
        }

        let copy_width = Ord::min(self.grid.width(), width);
        let copy_height = Ord::min(self.grid.height(), height);
        match (width, height) {
            (8, 8) => {
                let mut grid = [[0usize;8];8];

                for i in 0..copy_width {
                    for j in 0..copy_height {
                        grid[i][j] = self.grid.get(i, j);
                    }
                }

                self.grid = Box::new(CanvasGrid::<8, 8> { grid });

                Ok(())
            }
            (16, 16) => {
                let mut grid = [[0usize;16];16];

                for i in 0..copy_width {
                    for j in 0..copy_height {
                        grid[i][j] = self.grid.get(i, j);
                    }
                }

                self.grid = Box::new(CanvasGrid::<16, 16> { grid });

                Ok(())
            }
            (32, 32) => {
                unimplemented!()
            }
            (64, 64) => {
                unimplemented!()
            }
            _ => {
                Err(Error::InvalidCanvasSize(width, height))
            }
        }
    }

    pub(crate) fn set_pos(&mut self, pos: Pos2) {
        self.pos = pos;
    }

    pub fn palette_pos(&self) -> Vec2 {
        Vec2 {
            x: self.pos.x + self.grid.width() as f32 * self.pixel_width as f32,
            y: self.pos.y,
        }
    }

    pub(crate) fn get_palette_mut(&mut self) -> &mut Palette {
        &mut self.palette
    }

    pub fn render(&self, ui: &mut Ui) {
        // render grid
        for i in 0..self.grid.width() {
            for j in 0..self.grid.height() {
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
        // render cursor
        let (x, y) = self.cursor;
        let foo = self.pos + (Pos2::new(x as f32, y as f32) * self.pixel_width as f32).to_vec2();

        ui.painter().add(RectShape {
            rect: Rect {
                min: foo,
                max: foo + (Pos2::new(1.0, 1.0) * self.pixel_width as f32).to_vec2(),
            },
            rounding: Default::default(),
            fill: Color32::from(Rgba::from_black_alpha(0.0)),
            stroke: Stroke::new(2.0, Color32::GOLD),
            blur_width: 0.0,
            fill_texture_id: Default::default(),
            uv: Rect::ZERO,
        });
        // render palette
        let palette_pos = self.palette_pos();
        let mut draw_order: Vec<usize> = (0..self.palette.size()).filter(|x| *x != self.color_idx).collect();
        draw_order.push(self.color_idx);
        for i in draw_order {
            ui.painter().add(RectShape {
                rect: Rect {
                    min: (palette_pos + Pos2::new(
                        0.0,
                        i as f32 * self.pixel_width as f32,
                    ).to_vec2()).to_pos2(),
                    max: (palette_pos + Pos2::new(
                        self.pixel_width as f32,
                        (i + 1) as f32 * self.pixel_width as f32,
                    ).to_vec2()).to_pos2(),
                },
                rounding: if i == self.color_idx {
                    Rounding::from(3.0)
                } else {
                    Rounding::ZERO
                },
                fill: self.palette.get_color(i),
                stroke: if i == self.color_idx {
                    Stroke::new(2.0, Color32::GOLD)
                } else {
                    Stroke::new(1.0, Color32::BLACK)
                },
                blur_width: 0.0,
                fill_texture_id: Default::default(),
                uv: Rect::ZERO,
            });
        }
    }

    pub fn update(&mut self, ui: &mut Ui) {
        self.pos = ui.next_widget_position();
        // get area we're gonna draw in
        let draw_bounds = Rect {
            min: self.pos,
            max: (self.pos + Pos2 {
                x: self.pixel_width as f32 * (self.grid.width() as f32 + 3.0),
                y: self.pixel_width as f32 * self.grid.height() as f32,
            }.to_vec2()).into(),
        };
        ui.advance_cursor_after_rect(draw_bounds);
        ui.set_clip_rect(draw_bounds);

        // register click
        if ui.input(|i| i.pointer.button_clicked(PointerButton::Primary)) {
            let mut mouse_pos = Pos2::ZERO;
            ui.input(|i| mouse_pos = i.pointer.interact_pos().unwrap());

            // select palette
            let idx = (mouse_pos - self.palette_pos()) / self.pixel_width as f32;
            let x_bounds = idx.x <= 1.0 && idx.x >= 0.0;
            let y_bounds = idx.y < self.palette.size() as f32 && idx.y >= 0.0;
            if x_bounds && y_bounds {
                self.color_idx = idx.y as usize;
            }
        }

        // register button held down
        if ui.input(|i| i.pointer.button_down(PointerButton::Primary)) {
            let mut mouse_pos = Pos2::ZERO;
            ui.input(|i| mouse_pos = i.pointer.interact_pos().unwrap());

            // paint on canvas
            let idx = (mouse_pos - self.pos) / self.pixel_width as f32;
            let x_bounds = idx.x < self.grid.width() as f32 && idx.x >= 0.0;
            let y_bounds = idx.y < self.grid.height() as f32 && idx.y >= 0.0;
            if x_bounds && y_bounds {
                self.grid.set(idx.x as usize, idx.y as usize, self.color_idx);
            }
        }

        // switch palette
        if ui.input_mut(|i| i.consume_shortcut(&crate::app::shortcut::PALETTE_FORWARD)) {
            self.color_idx += 1;
            if self.color_idx == self.palette.size() {
                self.color_idx = 0;
            }
        }
        if ui.input_mut(|i| i.consume_shortcut(&crate::app::shortcut::PALETTE_BACKWARD)) {
            if self.color_idx == 0 {
                self.color_idx = self.palette.size();
            }
            self.color_idx -= 1;
        }

        // move cursor
        if ui.input_mut(|mut i| i.consume_shortcut(&action::CURSOR_LEFT)) {
            self.cursor.0 += 1;
            if self.cursor.0 >= self.grid.width() {
                self.cursor.0 = 0;
            }
        }
        if ui.input_mut(|mut i| i.consume_shortcut(&action::CURSOR_RIGHT)) {
            if self.cursor.0 == 0 {
                self.cursor.0 = self.grid.width();
            }
            self.cursor.0 -= 1;
        }
        if ui.input_mut(|mut i| i.consume_shortcut(&action::CURSOR_UP)) {
            if self.cursor.1 == 0 {
                self.cursor.1 = self.grid.width();
            }
            self.cursor.1 -= 1;
        }
        if ui.input_mut(|mut i| i.consume_shortcut(&action::CURSOR_DOWN)) {
            self.cursor.1 += 1;
            if self.cursor.1 >= self.grid.width() {
                self.cursor.1 = 0;
            }
        }
        // paint with cursor
        if ui.input_mut(|mut i| i.consume_shortcut(&action::CURSOR_PAINT)) {
            self.grid.set(self.cursor.0, self.cursor.1, self.color_idx);
        }

        // reset draw bounds
        ui.set_clip_rect(Rect::EVERYTHING);
    }

    pub fn get_pixel_color(&self, row: usize, col: usize) -> Color32 {
        self.palette.get_color(self.grid.get(row, col))
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}