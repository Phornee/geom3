use crate::base::{Shape, Intersection};
use crate::line3::Line3;
use list::List;
use std::fmt;
use crate::vector3::Vector3;

/// A plane in 3D space.
#[derive(Clone, Copy)]
pub struct Plane {
    pub a: Vector3, // Pivot point of plane
    pub n: Vector3, // Normal of the Plane (already normalized)
    pub d: f64,     //"Independent" term precalculated for performance
}


impl Plane {
    /// Creates a new `Plane`.
    /// The plane is defined by a point in the plane, `a`, and the normal of the plane, `n`.
    pub fn new(a: &Vector3, n: &Vector3) -> Plane {
        if n.is_zero() {
            panic!("The normal of the plane cannot be zero");
        }
        Self {
            a: *a,
            n: n.normalize(),
            d: -n.dot(&a),
        }
    }


}

impl Shape for Plane {
    /// Returns the normal (normalized) of the plane at a given point.
    fn normal(&self, _point: &Vector3) -> Vector3 {
        self.n
    }

    /// Returns the intersection of the plane with a line.
    /// If the line is parallel to the plane, it returns an empty list.
    /// If the line is in the plane, it returns an empty list as a convention (because really, all lambdas fulfill).
    /// If the line intersects the plane, it returns a list with the lambda value.
    /// The lambda value is used to calculate the point of intersection with the line.calc_point(lambda).
    fn intersects(&self, line: &Line3) -> List<f64> {
        let mut intersections: List<f64> = List::<f64>::new();

        let denom: f64 = self.n.dot(&line.v);
        if denom != 0. {
            let num: f64 = -self.n.dot(&line.a) - self.d;
            intersections.push(num / denom);
        }
        intersections
    }

    /// Returns the intersection of the plane with a line.
    /// If the line is parallel to the plane, it returns Option None.
    /// If the line is in the plane, it returns Option None as a convention (because really, all lambdas fulfill).
    /// If the line intersects the plane, it returns the lambda value.
    /// The lambda value is used to calculate the point of intersection with the line.calc_point(lambda).
    fn closest_intersection(&self, line: &Line3) -> Option<Intersection> {
        let denom: f64 = self.n.dot(&line.v);
        if denom != 0. {
            let num: f64 = -self.n.dot(&line.a) - self.d;
            return Some(Intersection::new(num / denom, None));
        }
        return None;
    }
}

impl fmt::Display for Plane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "A ({}, {}, {}) --> N ({}, {}, {})",
            self.a.x, self.a.y, self.a.z, self.n.x, self.n.y, self.n.z
        )
    }
}
