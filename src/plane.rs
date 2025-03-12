use crate::line3::Line3;
use list::List;
use std::fmt;
use vector3::Vector3;

pub struct Plane {
    pub a: Vector3, // Pivot point of plane
    pub n: Vector3, // Normal of the Plane (already normalized)
    pub d: f64      //"Independent" term precalculated for performance
}

impl Plane {
    /// Creates a new `Plane`.
    pub fn new(a: &Vector3, n: &Vector3) -> Plane {
        Self {
            a: *a,
            n: n.normalize(),
            d: -n.dot(&a)
        }
    }

    pub fn normal(&self, _point: &Vector3) -> Vector3 {
        self.n
    }

    pub fn intersects(&self, line: &Line3) -> List<f64> {
        let mut intersections = List::<f64>::new();

        let denom = self.n.dot(&line.v);
        if denom != 0. {
            let num = -self.n.x * line.a.x - self.n.y * line.a.y - self.n.z * line.a.z - self.d;
            intersections.push(num / denom);
        }
        intersections
    }
}

// impl fmt::Debug for Plane {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         //f.debug_struct("Line3").field("a", &self.a).field("v", &self.v).finish()
//          f.write_str(&format!("({}, {}, {}) --> ({}, {}, {})", self.a.x, self.a.y, self.a.z, self.v.x, self.v.y, self.v.z))
//     }
// }

impl fmt::Display for Plane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "A ({}, {}, {}) --> N ({}, {}, {})",
            self.a.x, self.a.y, self.a.z, self.n.x, self.n.y, self.n.z
        )
    }
}
