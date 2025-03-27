use crate::base::{Shape, Intersection};
use crate::line3::Line3;
use list::List;
use vector3::Vector3;

use std::fmt;

#[derive(Clone, Copy)]
pub struct Sphere {
    pub c: Vector3, // Center of the sphere
    pub r: f64,     // Radius of the sphere
}

impl Sphere {
    /// Creates a new `Sphere`.
    /// The sphere is defined by the center of the sphere, `c`, and the radius of the sphere, `r`.
    pub fn new(c: &Vector3, r: f64) -> Sphere {
        Self { c: *c, r }
    }


}

impl Shape for Sphere {
    /// Returns the normal (normalized) of the sphere at a given point (that should be in the surface of the sphere).
    fn normal(&self, point: &Vector3) -> Vector3 {
        (*point - self.c).normalize()
    }

    /// Returns the intersections of the sphere with a line.
    /// If the line doesn´t collide the sphere, it returns an empty list.
    /// If the line is tangent to the sphere, it returns a list with the single lambda value.
    /// If the line instersects the sphere, it returns a list with the two intersection lambdas.
    /// The lambda value is used to calculate the point of intersection with the line.calc_point(lambda).
    fn intersects(&self, line: &Line3) -> List<f64> {
        let mut intersections: List<f64> = List::<f64>::new();

        // line.qa is the module2 of the director vector of the line, and has already be verified to be not 0
        // So, no need to check for division by 0

        let o: Vector3 = line.a - self.c;
        let b: f64 = line.v.dot(&o) * 2.0;
        let c: f64 = o.x.powi(2) + o.y.powi(2) + o.z.powi(2) - self.r.powi(2);
        let discrim: f64 = b.powi(2) - 4. * line.qa * c;
        if discrim > 0. {
            let discrim2: f64 = discrim.sqrt();
            intersections.push((-b + discrim2) / (2. * line.qa));
            intersections.push((-b - discrim2) / (2. * line.qa));
        } else if discrim == 0. {
            intersections.push((-b) / (2. * line.qa));
        }

        intersections
    }

    /// Returns the closest positive intersection of the sphere with a line.
    /// If the line doesn´t collide the sphere, it returns Option None.
    /// If the line instersects the sphere, it returns the closest positive intersection lambda
    /// The lambda value is used to calculate the point of intersection with the line.calc_point(lambda).
    fn closest_intersection(&self, line: &Line3) -> Option<Intersection> {
        // line.qa is the module2 of the director vector of the line, and has alrady be verified to be not 0
        // So, no need to check for division by 0

        let o: Vector3 = line.a - self.c;
        let b: f64 = line.v.dot(&o) * 2.0;
        let c: f64 = o.x.powi(2) + o.y.powi(2) + o.z.powi(2) - self.r.powi(2);
        let discrim: f64 = b.powi(2) - 4. * line.qa * c;
        if discrim > 0. {
            let discrim2: f64 = discrim.sqrt();
            let intersection1: f64 = (-b + discrim2) / (2. * line.qa);
            let intersection2: f64 = (-b - discrim2) / (2. * line.qa);
            if intersection1 > 0. && intersection2 > 0. {
                return Some(Intersection::new(intersection1.min(intersection2), None));
            } else if intersection1 > 0. {
                return Some(Intersection::new(intersection1 , None));
            } else if intersection2 > 0. {
                return Some(Intersection::new(intersection2, None));
            }
        } else if discrim == 0. {
            Some((-b) / (2. * line.qa));
        }

        return None;
    }
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}) Radius = {}",
            self.c.x, self.c.y, self.c.z, self.r
        )
    }
}
