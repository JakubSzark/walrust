#![allow(dead_code)]

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

// ====================================
// Constructors
// ====================================

impl Vector2 {
    pub fn new_empty() -> Vector2 {
        Vector2 { x: 0f64, y: 0f64 }
    }

    pub fn new(_x: f64, _y: f64) -> Vector2 {
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

impl std::ops::Mul<f64> for Vector2 {
    type Output = Vector2;

    fn mul(self, s: f64) -> Self::Output {
        Vector2 {
            x: self.x * s,
            y: self.y * s,
        }
    }
}

impl std::ops::MulAssign<f64> for Vector2 {
    fn mul_assign(&mut self, s: f64) {
        *self = Vector2 {
            x: self.x * s,
            y: self.y * s,
        }
    }
}

impl std::ops::Div<f64> for Vector2 {
    type Output = Vector2;

    fn div(self, s: f64) -> Self::Output {
        Vector2 {
            x: self.x / s,
            y: self.y / s,
        }
    }
}

impl std::ops::DivAssign<f64> for Vector2 {
    fn div_assign(&mut self, s: f64) {
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

impl Vector2 {
    pub fn magnitude(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    pub fn normalized(&self) -> Vector2 {
        let mag = self.magnitude();
        Vector2 { x: self.x / mag, y: self.y / mag }
    }

    pub fn dot(&self, _rhs: Vector2) -> f64 {
        self.x * _rhs.x + self.y * _rhs.y
    }

    pub fn distance(&self, _rhs: Vector2) -> f64 {
        (f64::powf(_rhs.x - self.x, 2f64) + f64::powf(_rhs.y - self.y, 2f64)).sqrt().abs()
    }

    // Projection of vector 'self' onto '_rhs'
    // Returns vector representing component of 'self' parallel to '_rhs'
    pub fn project(&self, _rhs: Vector2) -> Vector2 {
        _rhs * self.dot(_rhs) / dot(_rhs, _rhs)
    }

    // Rejection of vector 'self' onto 'rhs'
    // Returns vector representing component of 'self' perpendicular to '_rhs'
    pub fn reject(&self, _rhs: Vector2) -> Vector2 {
        *self - _rhs * self.dot(_rhs) / dot(_rhs, _rhs)
    }
}

pub fn magnitude(vec: Vector2) -> f64 {
    vec.magnitude()
}

pub fn normalize(vec: Vector2) -> Vector2 {
    vec.normalized()
}

pub fn dot(_lhs: Vector2, _rhs: Vector2) -> f64 {
    _lhs.dot(_rhs)
}

pub fn distance(_lhs: Vector2, _rhs: Vector2) -> f64 {
    _lhs.distance(_rhs)
}

pub fn project(_lhs: Vector2, _rhs: Vector2) -> Vector2 {
    _lhs.project(_rhs)
}

pub fn reject(_lhs: Vector2, _rhs: Vector2) -> Vector2 {
    _lhs.reject(_rhs)
}

// cross product does not produce any useful result in 2D space

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
        let result: Vector2 = Vector2 { x: 1f64, y: 1f64 } + Vector2 { x: 1f64, y: 1f64 };
        let expected: Vector2 = Vector2 { x: 2f64, y: 2f64 };
        assert_eq!(result, expected);
    }

    #[test]
    fn addition_infinity() {
        let result: Vector2 = Vector2 {
            x: f64::MAX,
            y: f64::MAX,
        } + Vector2 {
            x: f64::MAX,
            y: f64::MAX,
        };
        let expected: Vector2 = Vector2 {
            x: f64::INFINITY,
            y: f64::INFINITY,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn simple_add_assign() {
        let mut vec1 = Vector2 { x: 1f64, y: 1f64 };
        vec1 += Vector2 { x: 1f64, y: 1f64 };
        let expected: Vector2 = Vector2 { x: 2f64, y: 2f64 };
        assert_eq!(vec1, expected);
    }

    #[test]
    fn simple_subtraction() {
        let result: Vector2 = Vector2 { x: 1f64, y: 1f64 } - Vector2 { x: 2f64, y: 2f64 };
        let expected: Vector2 = Vector2 { x: -1f64, y: -1f64 };
        assert_eq!(result, expected);
    }

    #[test]
    fn subtraction_negative_infinity() {
        let vec1: Vector2 = Vector2 {
            x: f64::MIN,
            y: f64::MIN,
        };
        let vec2: Vector2 = Vector2 {
            x: f64::MAX,
            y: f64::MAX,
        };
        let result: Vector2 = vec1 - vec2;
        let expected: Vector2 = Vector2 {
            x: -f64::INFINITY,
            y: -f64::INFINITY,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn simple_sub_assign() {
        
        let mut vec1 = Vector2 { x: 2f64, y: 2f64 };
        vec1 -= Vector2 { x: 1f64, y: 1f64 };
        let expected: Vector2 = Vector2 { x: 1f64, y: 1f64 };
        assert_eq!(vec1, expected);
    }

    #[test]
    fn simple_multiply() {
        let result: Vector2 = Vector2 { x: 1f64, y: 1f64 } * 7f64;
        let expected: Vector2 = Vector2 { x: 7f64, y: 7f64 };
        assert_eq!(result, expected);
    }

    #[test]
    fn simple_multiple_assign() {
        let mut result: Vector2 = Vector2 { x: 1f64, y: 1f64 };
        result *= 7f64;
        let expected: Vector2 = Vector2 { x: 7f64, y: 7f64 };
        assert_eq!(result, expected);
    }

    #[test]
    fn simple_divide() {
        let result: Vector2 = Vector2 { x: 7f64, y: 7f64 } / 7f64;
        let expected: Vector2 = Vector2 { x: 1f64, y: 1f64 };
        assert_eq!(result, expected);
    }

    #[test]
    fn simple_divide_assign() {
        let mut result: Vector2 = Vector2 { x: 7f64, y: 7f64 };
        result /= 7f64;
        let expected: Vector2 = Vector2 { x: 1f64, y: 1f64 };
        assert_eq!(result, expected);

    }

    #[test]
    fn simple_magnitude() {
        let result: f64 = Vector2 { x: 3f64, y: 4f64 }.magnitude();
        let expected = 5f64;
        assert_eq!(result, expected);
    }

    #[test]
    fn simple_normalized() {
        let result = Vector2 { x: 3f64, y: 0f64 }.normalized();
        let expected = Vector2 { x: 1f64, y: 0f64 };
        assert_eq!(result, expected)
    }

    #[test]
    fn simple_dot() {
        let vec1 = Vector2 { x: 2f64, y: 2f64};
        let vec2 = Vector2 {x: 1f64, y: 1f64};
        let mut result = vec1.dot(vec2);
        let expected = 4f64;
        assert_eq!(result, expected);
        result = dot(vec1, vec2);
        assert_eq!(result, expected);
    }

    #[test]
    fn simple_distance() {
        let vec1 = Vector2 { x: 0f64, y: 2f64};
        let vec2 = Vector2 {x: 0f64, y: 1f64};
        let result = vec1.distance(vec2);
        let expected = 1f64;
        assert_eq!(result, expected)
    }

    #[test]
    fn simple_project() {
        let vec1 = Vector2 { x: 3f64, y: 5f64 };
        let vec2 = Vector2 { x: 6f64, y: 2f64 };
        let result = vec1.project(vec2);
        let expected = Vector2 { x: 21.0/5.0, y: 7.0/5.0 };
        assert_eq!(result, expected)
    }

    #[test]
    fn simple_reject() {
        let vec1 = Vector2 { x: 3f64, y: 5f64 };
        let vec2 = Vector2 { x: 6f64, y: 2f64 };
        let result = vec1.reject(vec2);
        let expected = vec1 - vec1.project(vec2);
        assert_eq!(result, expected)
    }
}