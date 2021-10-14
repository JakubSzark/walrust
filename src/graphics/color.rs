#![allow(dead_code)]

/// Structure that represents an RGBA color.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

// ====================================
// Implementation
// ====================================

impl Color {
    /// Creates a Color from all four components
    pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Creates a Color from only three components, alpha is 255
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color::from_rgba(red, green, blue, 255)
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
