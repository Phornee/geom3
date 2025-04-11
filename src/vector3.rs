use std::f64::consts::PI;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Default)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    /// Creates a new `Vector3`.
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Self { x, y, z }
    }

    /// Returns the magnitude (length) of the vector.
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Returns the magnitude (length) of the vector ^2.
    pub fn magnitude_2(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    /// Returns a normalized copy of the vector.
    pub fn normalize(&self) -> Vector3 {
        let magnitude = self.magnitude();
        Vector3 {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    /// Uses `f64::EPSILON` to deduct if the length of the vector is zero.
    /// This is to avoid any small rounding errors.
    pub fn is_zero(&self) -> bool {
        self.magnitude().abs() < f64::EPSILON
    }

    /// Returns the distance from `self` to another vector.
    pub fn distance_to(&self, other: Vector3) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }

    /// Returns the dot product of `self` and another vector.
    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Returns the cross product of `self` and another vector.
    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - other.y * self.z,
            y: -(self.x * other.z - other.x * self.z),
            z: self.x * other.y - other.x * self.y,
        }
    }

    /// Returns the angle between `self` and another vector in radians.
    pub fn angle(&self, other: &Vector3) -> f64 {
        (self.dot(other) / self.magnitude() * other.magnitude()).acos()
    }

    /// Returns the angle between `self` and another vector in degrees.
    pub fn angle_deg(&self, other: &Vector3) -> f64 {
        self.angle(other) * (180.0 / PI)
    }

    // /// Reflects a vector against a normal vector.
    // pub fn reflection(&self, normal: &Vector3) -> Vector3 {
    //     self.angle(other) * (180.0 / PI)
    // }


    /// Creates a vector from three i32-s for your convenience.
    pub fn from_i32(x: i32, y: i32, z: i32) -> Vector3 {
        Vector3 {
            x: x as f64,
            y: y as f64,
            z: z as f64,
        }
    }

    /// Serializes the vector into a string than can later be deserialized.
    pub fn to_string(&self) -> String {
        format!("({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f64) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

// impl Mul<Vector3> for Vector3 {
//     fn mul(self, other: Vector3) -> Vector3 {
//         Vector3 {
//             x: self.x * other.x,
//             y: self.y * other.y,
//             z: self.z * other.z,
//         }
//     }
// }

impl Div<f64> for Vector3 {
    type Output = Vector3;
    fn div(self, scalar: f64) -> Vector3 {
        Vector3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl PartialEq<Self> for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f64::EPSILON
            && (self.y - other.y).abs() < f64::EPSILON
            && (self.z - other.z).abs() < f64::EPSILON
    }
}

impl From<(f64, f64, f64)> for Vector3 {
    fn from(tuple: (f64, f64, f64)) -> Vector3 {
        let (x, y, z) = tuple;

        Vector3 { x, y, z }
    }
}

impl Debug for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}
