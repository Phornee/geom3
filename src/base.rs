use crate::line3::Line3;
use list::List;
use crate::vector3::Vector3;

/// Struct that define a line in 3D space, with the pivot point and the director vector of the line.
/// It contains the minimal information of the Intersection with a Shape.
/// 'barycentric' is provided "for free" because its calculation is needed to know if the line intersects the triangle.
#[derive(Clone, Copy)]
pub struct Intersection {
    pub lambda: f64,
    pub barycentric: Option<Vector3>,
}

impl Intersection {
    /// Creates a new `Intersection`.
    /// The intersection is defined by the lambda value and the barycentric coordinates of the intersection.
    pub fn new(lambda: f64, barycentric: Option<&Vector3>) -> Intersection {
        Self {lambda, barycentric: barycentric.cloned()}
    }
}

pub trait Shape {
    /// Returns the normal (normalized) of the shape at a given point.
    fn normal(&self, _point: &Vector3) -> Vector3;

    /// Returns the intersections of the shape with a line with the minimal amount 
    /// of calculations, that is: just the lambda values.
    /// If the line is parallel to the shape, it returns an empty list.
    /// If the line is in the shape, it returns an empty list as a convention (because really, all lambdas fulfill).
    /// If the line intersects the shape, it returns a list with the lambda values.
    /// The lambda values can be used to calculate:
    ///     * The points of intersection with the line.calc_point(lambda).
    ///     * The normal of the shape at the point of intersection with shape.normal(point).
    fn intersects(&self, line: &Line3) -> List<f64>;

    /// Returns the closest positive 'Intersection' of the shape with a line
    /// ("positive" means, in the direction of the director vector of the Line).
    /// This function is more efficient than the 'intersects' function because it doesn't return a list, but just one
    /// 
    /// If the line doesn´t collide the shape, it returns Option None.
    /// If the line instersects the shape, it returns the closest positive 'Intersection'
    /// The lambda value inside the Intersection is used to calculate:
    ///     * The points of intersection with the line.calc_point(lambda).
    ///     * The normal of the shape at the point of intersection with shape.normal(point).
    fn closest_intersection(&self, line: &Line3) -> Option<Intersection>;
}
