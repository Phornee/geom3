use crate::line3::Line3;
use list::List;
use vector3::Vector3;

// struct Intersection {
//     lambda: f64,
//     normal: Vector3,
//     barycentric: Option<Vector3>,
// }
pub trait Shape {
    /// Returns the normal (normalized) of the object at a given point.
    fn normal(&self, _point: &Vector3) -> Vector3;

    /// Returns the intersection of the object with a line.
    /// If the line is parallel to the object, it returns an empty list.
    /// If the line is in the object, it returns an empty list as a convention (because really, all lambdas fulfill).
    /// If the line intersects the object, it returns a list with the lambda values.
    /// The lambda values can be used to calculate the points of intersection with the line.calc_point(lambda).
    fn intersects(&self, line: &Line3) -> List<f64>;

    /// Returns the closest positive intersection of the object with a line
    /// ("positive" means, in the direction of the director vector of the Line).
    /// If the line doesn´t collide the object, it returns Option None.
    /// If the line instersects the object, it returns the closest positive intersection lambda
    /// The lambda value is used to calculate the point of intersection with the line.calc_point(lambda).
    fn closest_intersection(&self, line: &Line3) -> Option<f64>;
}
