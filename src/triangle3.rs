use vector3::Vector3;

use std::fmt;

pub struct VertexData {
    pub vertex: Vector3,
    pub normal: Vector3,
    pub uv: Vector3
}

/// Struct to define a poligon point, along with its matching side (anticlockwise)
/// You can inherit from this struct to add more information to the vertex, such as UV coordinates, normals, etc.
pub struct Vertex3 {
    pub side: Line3 // Segment that defines the side of the triangle    
}

// Struct that define a triangle in 3D space, with the vertex defined in an
pub struct Triangle3 {
    pub a: Vertex3, 
    pub b: Vertex3, 
    pub c: Vertex3
}

impl Triangle3 {
    /// Creates a new `Line3`.
    /// The line is defined by two points in the line, `a` and `b`.
    /// With that, we calculate the director vector of the line (v = b - a) and the mod^2 of the director vector for
    /// better performance when calculating intersections later.
    pub fn new(a: &Vertex3, b: &Vertex3, c: &Vertex3) -> Triangle3 {
        Self { a, b, c }
    }

    /// Calculates a point in the line, given the lambda
    pub fn calc_point(&self, lambda: f64) -> Vector3 {
        self.a + self.v * lambda
    }

    /// Calculates the distance between a point and the line
    pub fn dist_point(&self, p: &Vector3) -> f64 {
        let ap = *p - self.a;
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