use vector3::Vector3;

use std::fmt;

pub struct Line3 {
    pub a: Vector3,  // Pivot point of the line
    pub v: Vector3,  // Direction point of the line
    pub qa: f64      // Precalculated mod^2 for performance when calculating intersections
}

impl Line3 {
    /// Creates a new `Line3`.
    pub fn new(a: Vector3, b:Vector3) -> Line3 {
        let v = b - a;
        let qa = v.x.powi(2) + v.y.powi(2) + v.z.powi(2);
        Self { a, v, qa }
    }

    pub fn calc_point(&self, lambda: f64) -> Vector3 {
        self.a + self.v * lambda
    }
}

impl fmt::Debug for Line3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //f.debug_struct("Line3").field("a", &self.a).field("v", &self.v).finish()
         f.write_str(&format!("({}, {}, {}) --> ({}, {}, {})", self.a.x, self.a.y, self.a.z, self.v.x, self.v.y, self.v.z))
    }
}

impl fmt::Display for Line3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //f.debug_struct("Line3").field("a", &self.a).field("v", &self.v).finish()
         write!(f, "A ({}, {}, {}) --> V ({}, {}, {})", self.a.x, self.a.y, self.a.z, self.v.x, self.v.y, self.v.z)
    }
}