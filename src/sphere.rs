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
    /// The sphere is defined by the center of the sphere, `c`, and the radius of the sphere, `r`.
    pub fn new(c: &Vector3, r: f64) -> Sphere {
        Self { c: *c, r}
    }

    /// Returns the normal (normalized) of the sphere at a given point (that should be in the surface of the sphere).
    pub fn normal(&self, point: &Vector3) -> Vector3 {
        (*point - self.c).normalize()
    }

    /// Returns the intersections of the sphere with a line.
    /// If the line doesn´t collide the sphere, it returns an empty list.
    /// If the line is tangent to the sphere, it returns a list with the single lambda value.
    /// If the line instersects the sphere, it returns a list with the two intersection lambdas.
    /// The lambda value is used to calculate the point of intersection with the line.calc_point(lambda).
    pub fn intersects(&self, line: &Line3) -> List::<f64> {
        let mut intersections = List::<f64>::new();

        // line.qa is the module2 of the director vector of the line, and has alrady be verified to be not 0
        // So, no need to check for division by 0

        let o = line.a - self.c;
        let b = 2. * line.v.x * o.x +  2. * line.v.y * o.y + 2. * line.v.z * o.z;
        let b = line.v.dot(&o)*2.0;
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