use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64
}

impl Color {
    pub const WHITE: Color = Color { red: 1f64, green: 1f64, blue: 1f64, alpha: 1f64 };
    pub const BLACK: Color = Color { red: 0f64, green: 0f64, blue: 0f64, alpha: 1f64 };
    pub const ZERO: Color = Color { red: 0f64, green: 0f64, blue: 0f64, alpha: 0f64 };

    pub fn from(red: f64, green: f64, blue: f64, alpha: f64) -> Color {
        Color { red, green, blue, alpha }
    }
    
    pub fn medium_of(colors: Vec<Color>) -> Color {
        if colors.is_empty() {
            return Color::ZERO;
        }
        
        let (color_count, color_sum) = colors.iter()
            .fold((0f64, Color::ZERO), |(color_count, color_sum), color| (color_count + 1f64, Color::from(color_sum.red + color.red, color_sum.green + color.green, color_sum.blue + color.blue, color_sum.alpha + color.alpha)));
        
        Color::from(color_sum.red / color_count, color_sum.green / color_count, color_sum.blue / color_count, color_sum.alpha / color_count)
    }

    pub fn mix_of(colors: Vec<Color>) -> Color {
        colors.iter()
            .fold(Color::WHITE, |color1, color2| Color::from(color1.red * color2.red, color1.green * color2.green, color1.blue * color2.blue, color1.alpha * color2.alpha))
    }

    pub fn inverse(&self) -> Color {
        Color::from(1f64 - self.red, 1f64 - self.green, 1f64 - self.blue, self.alpha)
    }
    
    pub fn as_pbm_color(&self) -> String {
        format!("{} {} {}", (self.red * 255f64).min(255f64) as i64, (self.green * 255f64).min(255f64) as i64, (self.blue * 255f64).min(255f64) as i64)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", (self.red * 255f64) as i64, (self.green * 255f64) as i64, (self.blue * 255f64) as i64)
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
            alpha: self.alpha + rhs.alpha
        }
    }
}

impl Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        Color {
            red: (self.red - rhs.red).max(0f64),
            green: (self.green - rhs.green).max(0f64),
            blue: (self.blue - rhs.blue).max(0f64),
            alpha: (self.alpha - rhs.alpha).max(0f64)
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color { red: (self.red * rhs).max(0f64), green: (self.green * rhs).max(0f64), blue: (self.blue * rhs).max(0f64), alpha: (self.alpha * rhs).max(0f64) }
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1f64 / rhs)
    }
}
