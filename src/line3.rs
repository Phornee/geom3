use vector3::Vector3;

use std::fmt;

pub struct Line3 {
    pub a: Vector3,  // Pivot point of the line
    pub v: Vector3,  // Director vector of the line
    pub qa: f64      // Precalculated mod^2 for performance when calculating intersections
}

impl Line3 {
    /// Creates a new `Line3`.
    /// The line is defined by two points in the line, `a` and `b`.
    /// With that, we canculate the director vector of the line (v = b - a) and the mod^2 of the director vector for
    /// better performance when calculating intersections later.
    pub fn new(a: &Vector3, b:&Vector3) -> Line3 {
        let v = *b - *a;
        let qa = v.x.powi(2) + v.y.powi(2) + v.z.powi(2);
        if  qa == 0. {
            panic!("The line cannot be defined by two equal points");
        }
        Self { a: *a, v, qa }
    }

    /// Calculates a point in the line, given the lambda
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