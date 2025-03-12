use vector3::Vector3;
use list::List;
use crate::line3::Line3;

use std::fmt;

pub struct Sphere {
    pub c: Vector3,  // Center of the sphere
    pub r: f64       // Radius of the sphere
}

impl Sphere {
    /// Creates a new `Sphere`.
    pub fn new(c: &Vector3, r: f64) -> Sphere {
        Self { c: *c, r}
    }

    pub fn normal(&self, point: &Vector3) -> Vector3 {
        (*point - self.c).normalize()
    }

    pub fn intersects(&self, line: &Line3) -> List::<f64> {
        let mut intersections = List::<f64>::new();

        if line.qa != 0. {
            let o = line.a - self.c;
            let b = 2. * line.v.x * o.x +  2. * line.v.y * o.y + 2. * line.v.z * o.z;
            let c = o.x.powi(2) + o.y.powi(2) + o.z.powi(2) - self.r.powi(2);
            let discrim: f64 = b.powi(2) - 4. * line.qa * c;
            if discrim > 0. {
                let discrim2 = discrim.sqrt();
                intersections.push((-b + discrim2) / (2. * line.qa));
                intersections.push((-b - discrim2) / (2. * line.qa));
            }
            else if discrim == 0. {
                intersections.push((-b) / (2. * line.qa));
            }
        }
        intersections
    }

}

// impl fmt::Debug for Line3 {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         //f.debug_struct("Line3").field("a", &self.a).field("v", &self.v).finish()
//          f.write_str(&format!("({}, {}, {}) --> ({}, {}, {})", self.a.x, self.a.y, self.a.z, self.v.x, self.v.y, self.v.z))
//     }
// }

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         write!(f, "({}, {}, {}) Radius = {}", self.c.x, self.c.y, self.c.z, self.r)
    }
}