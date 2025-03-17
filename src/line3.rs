use vector3::Vector3;
use std::fmt;

/// Struct that define a line in 3D space, with the pivot point and the director vector of the line
#[derive(Clone, Copy)]
pub struct Line3 {
    pub a: Vector3,  // Pivot point of the line
    pub v: Vector3,  // Director vector of the line
    pub qa: f64      // Precalculated mod^2 for performance when calculating intersections
}

impl Line3 {
    /// Creates a new `Line3`.
    /// The line is defined by two points in the line, `a` and `b`.
    /// With that, we calculate the director vector of the line (v = b - a) and the mod^2 of the director vector for
    /// better performance when calculating intersections later.
    pub fn new(a: &Vector3, b:&Vector3) -> Line3 {
        let v: Vector3 = *b - *a;
        let qa: f64 = v.x.powi(2) + v.y.powi(2) + v.z.powi(2);
        if  qa == 0. {
            panic!("The line cannot be defined by two equal points");
        }
        Self { a: *a, v, qa }
    }

    /// Calculates a point in the line, given the lambda
    pub fn calc_point(&self, lambda: f64) -> Vector3 {
        self.a + self.v * lambda
    }

    /// Calculates the distance between a point and the line
    pub fn dist_point(&self, p: &Vector3) -> f64 {
        let ap: Vector3 = *p - self.a;
        ap.cross(&self.v).magnitude() / self.qa.sqrt()
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