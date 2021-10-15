#![allow(dead_code)]

/// Structure that represents an RGBA color.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

// ====================================
// Implementation
// ====================================

impl Color {
    /// Creates a Color from all four components
    pub const fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Creates a Color from only three components, alpha is 255
    pub const fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color::from_rgba(red, green, blue, 255)
    }
}

// ====================================
// Constants
// ====================================

impl Color {
    pub const BLACK: Color = Color::from_rgb(0, 0, 0);
    pub const WHITE: Color = Color::from_rgb(255, 255, 255);
    pub const CLEAR: Color = Color::from_rgba(0, 0, 0, 0);

    pub const RED: Color = Color::from_rgb(255, 0, 0);
    pub const GREEN: Color = Color::from_rgb(0, 255, 0);
    pub const BLUE: Color = Color::from_rgb(0, 0, 255);

    pub const YELLOW: Color = Color::from_rgb(255, 255, 0);
    pub const MAGENTA: Color = Color::from_rgb(255, 0, 255);
    pub const CYAN: Color = Color::from_rgb(0, 255, 255);
}

// ====================================
// Other Implementations
// ====================================

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "({}, {}, {}, {})",
            self.red, self.green, self.blue, self.alpha
        ))
    }
}

// ====================================
// Overloading Operators
// ====================================

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color::from_rgba(
            self.red.saturating_add(rhs.red),
            self.green.saturating_add(rhs.green),
            self.blue.saturating_add(rhs.blue),
            self.alpha.saturating_add(rhs.alpha),
        )
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color::from_rgba(
            self.red.saturating_sub(rhs.red),
            self.green.saturating_sub(rhs.green),
            self.blue.saturating_sub(rhs.blue),
            self.alpha.saturating_sub(rhs.alpha),
        )
    }
}

impl std::ops::SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color::from_rgba(
            ((self.red as f32) * rhs).clamp(0.0, 255.0) as u8,
            ((self.green as f32) * rhs).clamp(0.0, 255.0) as u8,
            ((self.blue as f32) * rhs).clamp(0.0, 255.0) as u8,
            ((self.alpha as f32) * rhs).clamp(0.0, 255.0) as u8,
        )
    }
}

impl std::ops::MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}
