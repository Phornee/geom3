use vector3::Vector3;
use crate::base::{Shape, Intersection};
use crate::plane::Plane;
use crate::line3::Line3;
use list::List;

// Struct that define a triangle in 3D space, with the vertex defined in an
// anticlockwise order.
#[derive(Clone, Copy)]
pub struct Triangle3 {
    pub a: Vector3, // Fist point of the triangle
    pub b: Vector3, // Second point, in anticlockwise from a
    pub c: Vector3, // Third point, in anticlockwise from b

    pub plane: Plane, // Plane that contains the triangle

    ab: Vector3, // Factors precalculated to speed up barycentric calculus
    ac: Vector3,
    d00: f64,
    d01: f64,
    d11: f64,
    denom: f64,
}

impl Triangle3 {
    /// Creates a new `Triangle3` from three points.
    /// # Arguments
    /// * `a` - A reference to the first vertex of the triangle.
    /// * `b` - A reference to the second vertex of the triangle.
    /// * `c` - A reference to the third vertex of the triangle.
    /// a, b, c must be defined in anticlockwise order (looked from the visible side)
    /// # Returns
    /// A new `Triangle3` with the given vertices.
    pub fn new(a: &Vector3, b: &Vector3, c: &Vector3) -> Triangle3 {
        let ab: Vector3 = *b - *a;
        let ac: Vector3 = *c - *a;

        let n: Vector3 = ab.cross(&ac).normalize();
        let plane: Plane = Plane::new(&a, &n);

        let d00: f64 = ab.dot(&ab);
        let d01: f64 = ab.dot(&ac);
        let d11: f64 = ac.dot(&ac);
        let denom: f64 =  d00 * d11 - d01 * d01;

        if  denom == 0. {
            panic!("The triangle cannot be defined by three aligned points.");
        }

        Self { a: *a, b: *b, c: *c, plane, ab, ac, d00, d01, d11, denom }
    }

    /// Returns the barycentric coordinates of a point in the triangle.
    /// # Arguments
    /// * `p` - A reference to the point to calculate the barycentric coordinates.
    /// # Returns
    /// A new `Vector3` with the barycentric coordinates of the point.
    /// The coordinates are in the order of the vertices of the triangle (a, b, c).
    /// The sum of the coordinates is always 1.
    /// If the point is on the side of the triangle, one of the coordinates is zero.
    /// If the point is inside the triangle, the coordinates are between 0 and 1.
    /// The point (or any other value associated to the vertexes) is calculated as `p = a * bar_a + b * bar_b + c * bar_c`.
    pub fn barycentric(&self, p: &Vector3) -> Vector3 {
        let ap: Vector3 = *p - self.a;
        let d20: f64 = ap.dot(&self.ab);
        let d21: f64 = ap.dot(&self.ac);
        let bar_b: f64 = (self.d11 * d20 - self.d01 * d21) / self.denom;
        let bar_c: f64 = (self.d00 * d21 - self.d01 * d20) / self.denom;
        let bar_a: f64 = 1.0 - bar_b - bar_c;
        
        Vector3{x: bar_a, y: bar_b, z: bar_c }
    }

}

impl Shape for Triangle3 {
    /// Returns the normal (normalized) of the triangle at a given point.
    fn normal(&self, _point: &Vector3) -> Vector3 {
        self.plane.n
    }

    /// Returns the intersection of the triangle with a line.
    /// If the line is parallel to the triangle, it returns an empty list.
    /// If the line is in the plane of the triangle, it returns an empty list as a convention (because really, all lambdas fulfill).
    /// If the line intersects the triangle, it returns a list with the lambda value.
    fn intersects(&self, line: &Line3) -> List::<f64> {
        let intersections: List<f64> = self.plane.intersects(line);
        if intersections.iter().count() == 0 {
            return intersections;
        }
        else {
            let p: Vector3 = line.calc_point(*intersections.iter().next().unwrap());
            let b: Vector3 = self.barycentric(&p);
            if  1.0 >= b.x && b.x >= 0.0 && 
                1.0 >= b.y && b.y >= 0.0 && 
                1.0 >= b.z && b.z >= 0.0 {
                return intersections;
            }
            else {
                return List::<f64>::new();
            }
        }
    }

    /// Returns the intersection of the triangle with a line.
    /// If the line is parallel to the triangle, it returns an empty list.
    /// If the line is in the plane of the triangle, it returns an empty list as a convention (because really, all lambdas fulfill).
    /// If the line intersects the triangle, it returns a list with the lambda value.
    fn closest_intersection(&self, line: &Line3) -> Option<Intersection> {
        let intersection: Option<Intersection> = self.plane.closest_intersection(line);
        if intersection.is_none() {
            return None;
        }
        else {
            let p: Vector3 = line.calc_point(intersection.unwrap().lambda);
            let bar: Vector3 = self.barycentric(&p);
            if  1.0 >= bar.x && bar.x >= 0.0 && 
                1.0 >= bar.y && bar.y >= 0.0 && 
                1.0 >= bar.z && bar.z >= 0.0 {
                return intersection;
            }
            else {
                return None;
            }
        }
    }
}

// impl fmt::Display for Triangle3 {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         //f.debug_struct("Line3").field("a", &self.a).field("v", &self.v).finish()
//          write!(f, "A {} | B {} | C {}", self.a, self.b, self.c)
//     }
// }