#![allow(dead_code)]

type Float = f32;

#[derive(Debug)]
pub struct Vector2 {
    pub x: Float,
    pub y: Float,
}

// ====================================
// Constructors
// ====================================

impl Vector2 {
    pub fn new_empty() -> Vector2 {
        Vector2 { x: 0f32, y: 0f32 }
    }

    pub fn new(_x: Float, _y: Float) -> Vector2 {
        Vector2 { x: _x, y: _y }
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
            y: self.y + _rhs.y,
        }
    }
}

impl std::ops::AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, _rhs: Self) {
        *self = Vector2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl std::ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, _rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl std::ops::SubAssign<Vector2> for Vector2 {
    fn sub_assign(&mut self, _rhs: Self) {
        *self = Vector2 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl std::ops::Mul<Float> for Vector2 {
    type Output = Vector2;

    fn mul(self, s: Float) -> Self::Output {
        Vector2 {
            x: self.x * s,
            y: self.y * s,
        }
    }
}

impl std::ops::MulAssign<Float> for Vector2 {
    fn mul_assign(&mut self, s: Float) {
        *self = Vector2 {
            x: self.x * s,
            y: self.y * s,
        }
    }
}

impl std::ops::Div<Float> for Vector2 {
    type Output = Vector2;

    fn div(self, s: Float) -> Self::Output {
        Vector2 {
            x: self.x / s,
            y: self.y / s,
        }
    }
}

impl std::ops::DivAssign<Float> for Vector2 {
    fn div_assign(&mut self, s: Float) {
        *self = Vector2 {
            x: self.x / s,
            y: self.y / s,
        }
    }
}

impl std::cmp::PartialEq<Vector2> for Vector2 {
    fn eq(&self, other: &Vector2) -> bool {
        self.x == other.x && self.y == other.y
    }
}

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
        f.write_fmt(format_args!("[{}, {}]", self.x, self.y))
    }
}

// ====================================
// Unit Tests
// ====================================

#[cfg(test)]
mod vector2_tests {

    use super::*;

    #[test]
    fn simple_addition() {
        let result: Vector2 = Vector2 { x: 1f32, y: 1f32 } + Vector2 { x: 1f32, y: 1f32 };
        let expected: Vector2 = Vector2 { x: 2f32, y: 2f32 };
        assert_eq!(result, expected);
    }

    #[test]
    fn addition_infinity() {
        let result: Vector2 = Vector2 {
            x: f32::MAX,
            y: f32::MAX,
        } + Vector2 {
            x: f32::MAX,
            y: f32::MAX,
        };
        let expected: Vector2 = Vector2 {
            x: f32::INFINITY,
            y: f32::INFINITY,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn simple_add_assign() {
        let mut vec1 = Vector2 { x: 1f32, y: 1f32 };
        vec1 += Vector2 { x: 1f32, y: 1f32 };
        let expected: Vector2 = Vector2 { x: 2f32, y: 2f32 };
        assert_eq!(vec1, expected);
    }

    #[test]
    fn simple_subtraction() {
        let result: Vector2 = Vector2 { x: 1f32, y: 1f32 } - Vector2 { x: 2f32, y: 2f32 };
        let expected: Vector2 = Vector2 { x: -1f32, y: -1f32 };
        assert_eq!(result, expected);
    }

    #[test]
    fn subtraction_negative_infinity() {
        let vec1: Vector2 = Vector2 {
            x: f32::MIN,
            y: f32::MIN,
        };
        let vec2: Vector2 = Vector2 {
            x: f32::MAX,
            y: f32::MAX,
        };
        let result: Vector2 = vec1 - vec2;
        let expected: Vector2 = Vector2 {
            x: -f32::INFINITY,
            y: -f32::INFINITY,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn simple_sub_assign() {
        
        let mut vec1 = Vector2 { x: 2f32, y: 2f32 };
        vec1 -= Vector2 { x: 1f32, y: 1f32 };
        let expected: Vector2 = Vector2 { x: 1f32, y: 1f32 };
        assert_eq!(vec1, expected);
    }

    #[test]
    fn simple_multiply() {
        let result: Vector2 = Vector2 { x: 1f32, y: 1f32 } * 7f32;
        let expected: Vector2 = Vector2 { x: 7f32, y: 7f32 };
        assert_eq!(result, expected);
    }

    #[test]
    fn simple_multiple_assign() {
        let mut result: Vector2 = Vector2 { x: 1f32, y: 1f32 };
        result *= 7f32;
        let expected: Vector2 = Vector2 { x: 7f32, y: 7f32 };
        assert_eq!(result, expected);
    }

    #[test]
    fn simple_divide() {
        let result: Vector2 = Vector2 { x: 7f32, y: 7f32 } / 7f32;
        let expected: Vector2 = Vector2 { x: 1f32, y: 1f32 };
        assert_eq!(result, expected);
    }

    #[test]
    fn simple_divide_assign() {
        let mut result: Vector2 = Vector2 { x: 7f32, y: 7f32 };
        result /= 7f32;
        let expected: Vector2 = Vector2 { x: 1f32, y: 1f32 };
        assert_eq!(result, expected);

    }
}