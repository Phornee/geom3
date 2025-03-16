#[cfg(test)]
mod tests {
    use crate::{Line3, Sphere, Plane, Triangle3};
    use vector3::Vector3;
    use list::List;

    #[test]
    fn line_tests() {
        let a = Vector3::new(0.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 0.0, 10.0);

        let result = std::panic::catch_unwind(|| {
            let _wrong_line = Line3::new(&a, &a);
        });
        assert!(result.is_err());

        let line = Line3::new(&a, &b);
        assert_eq!(line.calc_point(0.), a);
        assert_eq!(line.calc_point(1.), b);

        let dist = line.dist_point(&Vector3::new(0.0, 0.0, 5.0));
        assert_eq!(dist, 0.0);

        let dist = line.dist_point(&Vector3::new(3.0, 7.0, 5.0));
        assert_eq!(dist, 7.6157731058639087);
    }

    #[test]
    fn sphere_tests() {
        let a = Vector3::new(0.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 0.0, 10.0);

        let sphere = Sphere::new(&a, 2.0);

        let line = Line3::new(&a, &b);
        let tangent_line = Line3::new( &Vector3::new(0.0, 2.0, 10.0),  &Vector3::new(0.0, 2.0, 0.0));
        let outer_line = Line3::new( &Vector3::new(0.0, 2.1, 10.0),  &Vector3::new(0.0, 2.1, 0.0));

        let instersection: List::<f64> = sphere.intersects(&line);
        assert_eq!(instersection.iter().count(), 2);

        let tangent_instersection: List::<f64> = sphere.intersects(&tangent_line);
        assert_eq!(tangent_instersection.iter().count(), 1);

        let outer_instersection: List::<f64> = sphere.intersects(&outer_line);
        assert_eq!(outer_instersection.iter().count(), 0);
    }

    #[test]
    fn plane_tests() {
        let plane = Plane::new(&Vector3::new(0.0, 0.0, 0.0), &Vector3::new(1.0, 1.0, 0.0));

        let horizontal_line = Line3::new(&Vector3::new(0.0, 1.0, 0.0), &Vector3::new(0.0, 1.0, 10.0));
        let tilted_line = Line3::new( &Vector3::new(0.0, 0.0, 0.0), &Vector3::new(0.0, 1.0, 10.0));
        let in_plane_line = Line3::new(&Vector3::new(0.0, 0.0, 0.0), &Vector3::new(-13.5e39, 13.5e39, 83810.16789));

        let mut instersection: List::<f64> = plane.intersects(&horizontal_line);
        assert_eq!(instersection.iter().count(), 0);

        instersection = plane.intersects(&tilted_line);
        assert_eq!(instersection.iter().count(), 1);

        instersection = plane.intersects(&in_plane_line);
        assert_eq!(instersection.iter().count(), 0);
    }
    
    #[test]
    fn triangle_tests() {
        let a: Vector3 = Vector3::new(0.0, 0.0, 0.0);
        let b: Vector3 = Vector3::new(0.0, 0.0, 10.0);
        let c: Vector3 = Vector3::new(0.0, 10.0, 0.0);
        let triangle = Triangle3::new(&a, &b, &c);

        // Point inside the triangle
        let mut barycentric = triangle.barycentric(&Vector3::new(0.0, 5.0, 5.0));
        assert_eq!(barycentric, Vector3::new(0.0, 0.5, 0.5));
        let intersections = triangle.intersects(&Line3::new(&Vector3::new(10.0, 5.0, 5.0), &Vector3::new(0.0, 5.0, 5.0)));
        assert_eq!(intersections.iter().count(), 1);

        // Point matching vertexes
        barycentric = triangle.barycentric(&a);
        assert_eq!(barycentric, Vector3::new(1.0, 0.0, 0.0));
        barycentric = triangle.barycentric(&b);
        assert_eq!(barycentric, Vector3::new(0.0, 1.0, 0.0));
        barycentric = triangle.barycentric(&c);
        assert_eq!(barycentric, Vector3::new(0.0, 0.0, 1.0));

        // Point outside triangle
        barycentric = triangle.barycentric(&Vector3::new(0.0, 8.0, 1.0));
        assert_eq!(barycentric, Vector3::new(0.09999999999999998, 0.1, 0.8));
        let intersections = triangle.intersects(&Line3::new(&Vector3::new(10.0, 15.0, 15.0), &Vector3::new(0.0, 15.0, 15.0)));
        assert_eq!(intersections.iter().count(), 0);


        // Point outside triangle plane, but inside if projected on plane
        barycentric = triangle.barycentric(&Vector3::new(0.0, 5.0, 5.0));
        assert_eq!(barycentric, Vector3::new(0.0, 0.5, 0.5));
    }
}
