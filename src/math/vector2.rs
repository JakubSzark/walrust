#![allow(dead_code)]

type Float = f32;

pub struct Vector2 {
    pub x: Float,
    pub y: Float
}

// ====================================
// Constructors
// ====================================

impl Vector2 {
    pub fn new_empty() -> Vector2 {
        Vector2 {
            x: 0f32,
            y: 0f32
        }
    }

    pub fn new(_x: Float, _y: Float) -> Vector2 {
        Vector2 {
            x: _x,
            y: _y
        }
    }
}

// ====================================
// Overloading Operators
// ====================================


impl std::ops::Add<Vector2> for Vector2 {
    type Output = Vector2;
    fn add(self, _rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y
        }
    }
}

// impl std::ops::AddAssign<Vector2> for Vector2 {
//     fn add_assign(self, _rhs: Self) {
//         *self = Vector2 {
//             x: self.x.saturating_add(_rhs.x),
//             y: self.y.saturating_add(_rhs.y)
//         }
//     }
// }

// impl std::ops::Sub<Vector2> for Vector2 {
//     type Output = Vector2;

//     fn sub(self, _rhs: Self) -> Self::Output {
//         Vector2 {
//             x: self.x.saturating_sub(_rhs.x),
//             y: self.y.saturating_sub(_rhs.y)
//         }
//     }
// }

// impl std::ops::SubAssign<Vector2> for Vector2 {
//     fn sub(self, _rhs: Self) {
//         *self = Vector2 {
//             x: self.x.saturating_sub(_rhs.x),
//             y: self.y.saturating_sub(_rhs.y)
//         }
//     }
// }

// impl std::ops::Mul<Float> for Vector2 {
//     type Output = Vector2;

//     fn mul(self, s: Float) -> Self::Output {
//         Vector2 {
//             x: self.x.saturating_mul(s),
//             y: self.y.saturating_mul(s)
//         }
//     }
// }

// impl std::ops::MulAssign<Float> for Vector2 {
//     fn mul_assign(self, s: Float) {
//         *self = Vector2 {
//             x: self.x.saturating_mul(s),
//             y: self.y.saturating_mul(s)
//         }
//     }
// }

// impl std::ops::Div<Float> for Vector2 {
//     type Output = Vector2;

//     fn div(self, s: Float) -> Self::Output {
//         Vector2 {
//             x: self.x.saturating_div(s),
//             y: self.y.saturating_div(s)
//         }
//     }
// }

// impl std::ops::DivAssign<Float> for Vector2 {
//     fn div_assign(self, s: Float) {
//         *self = Vector2 {
//             x: self.x.saturating_div(s),
//             y: self.y.saturating_div(s)
//         }
//     }
// }

// ====================================
// Operations
// ====================================

// impl Vector2 {
//     pub const fn Magnitude(&self) -> Float {
//         (num::checked_pow(self.x, 2)).sqrt()
//     }
// }

// ====================================
// Other Implementations
// ====================================

impl std::fmt::Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[{}, {}]",
            self.x, self.y
        ))
    }
}

// ====================================
// Unit Tests
// ====================================

#[cfg(test)]
mod vector2_tests {

    #[test]
    fn test_true() {
        assert!(true)
    }
}