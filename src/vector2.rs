use std::f64::consts::PI;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Default)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    /// Creates a new `Vector2`.
    pub fn new(x: f64, y: f64) -> Vector2 {
        Self { x, y}
    }

    /// Returns the magnitude (length) of the vector.
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) ).sqrt()
    }

    /// Returns a normalized copy of the vector.
    pub fn normalize(&self) -> Vector2 {
        let magnitude = self.magnitude();
        Vector2 {
            x: self.x / magnitude,
            y: self.y / magnitude
        }
    }

    /// Uses `f64::EPSILON` to deduct if the length of the vector is zero.
    /// This is to avoid any small rounding errors.
    pub fn is_zero(&self) -> bool {
        self.magnitude().abs() < f64::EPSILON
    }

    /// Returns the distance from `self` to another vector.
    pub fn distance_to(&self, other: Vector2) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    /// Returns the dot product of `self` and another vector.
    pub fn dot(&self, other: &Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Returns the angle between `self` and another vector in radians.
    pub fn angle(&self, other: &Vector2) -> f64 {
        (self.dot(other) / self.magnitude() * other.magnitude()).acos()
    }

    /// Returns the angle between `self` and another vector in degrees.
    pub fn angle_deg(&self, other: &Vector2) -> f64 {
        self.angle(other) * (180.0 / PI)
    }

    // /// Reflects a vector against a normal vector.
    // pub fn reflection(&self, normal: &Vector3) -> Vector3 {
    //     self.angle(other) * (180.0 / PI)
    // }

    /// Creates a vector from three i32-s for your convenience.
    pub fn from_i32(x: i32, y: i32) -> Vector2 {
        Vector2 {x: x as f64, y: y as f64}
    }

    /// Serializes the vector into a string than can later be deserialized.
    pub fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        Vector2 {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Vector2) -> Vector2 {
        Vector2 {x: self.x - other.x, y: self.y - other.y}
    }
}

impl Mul<f64> for Vector2 {
    type Output = Vector2;

    fn mul(self, scalar: f64) -> Vector2 {
        Vector2 {x: self.x * scalar, y: self.y * scalar}
    }
}

impl Div<f64> for Vector2 {
    type Output = Vector2;
    fn div(self, scalar: f64) -> Vector2 {
        Vector2 {x: self.x / scalar, y: self.y / scalar}
    }
}

impl PartialEq<Self> for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f64::EPSILON
            && (self.y - other.y).abs() < f64::EPSILON
    }
}

impl From<(f64, f64)> for Vector2 {
    fn from(tuple: (f64, f64)) -> Vector2 {
        let (x, y) = tuple;
        Vector2 { x, y }
    }
}

impl Debug for Vector2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}
