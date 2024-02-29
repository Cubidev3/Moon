use std::fmt::format;
use crate::color::Color;

pub struct Screen {
    pixels: Vec<Color>,
    width: usize,
    height: usize
}

impl Screen {
    pub fn from(width: usize, height: usize, default_color: Color) -> Screen {
        Screen { pixels: vec![default_color; width * height], width, height }
    }

    pub fn paint_at(&mut self, x: usize, y: usize, color: Color) -> Result<(), ()> {
        if !self.is_valid_point(x, y) {
            return Err(());
        }

        self.pixels[self.width * y + x] = color;
        Ok(())
    }

    pub fn get_color_at(&self, x: usize, y: usize) -> Option<Color> {
        if !self.is_valid_point(x, y) {
            return None;
        }

        Some(self.pixels[self.width * y + x])
    }

    pub fn pixel_positions(&self) -> ScreenPixelPositions {
        ScreenPixelPositions { x: 0, y: 0, width: self.width, height: self.height }
    }

    pub fn is_valid_point(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn resolution(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn to_pbm(&self) -> String {
        let colors: Vec<_> = self.pixels.iter().map(|color| format!("{}\n", color.as_pbm_color())).collect();
        let colors = colors.concat();
        
        format!("P3\n{} {}\n255\n{}", self.width, self.height, colors)
    }
}

pub struct ScreenPixelPositions {
    pub x: usize,
    pub y: usize,
    width: usize,
    height: usize,
}

impl Iterator for ScreenPixelPositions {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }

        if self.y >= self.height {
            return None;
        }

        let current: (usize, usize) = (self.x, self.y);
        self.x += 1;
        Some(current)
    }
}