#[cfg(test)]
mod tests {
    use crate::{Line3, Plane, Shape, Sphere, Triangle3, Intersection};
    use list::List;
    use vector3::Vector3;

    #[test]
    fn line_tests() {
        let a: Vector3 = Vector3::new(0.0, 0.0, 0.0);
        let b: Vector3 = Vector3::new(0.0, 0.0, 10.0);

        let result = std::panic::catch_unwind(|| {
            let _wrong_line = Line3::new(&a, &a);
        });
        assert!(result.is_err());

        let line: Line3 = Line3::new(&a, &b);
        assert_eq!(line.calc_point(0.), a);
        assert_eq!(line.calc_point(1.), b);

        let dist: f64 = line.dist_point(&Vector3::new(0.0, 0.0, 5.0));
        assert_eq!(dist, 0.0);

        let dist: f64 = line.dist_point(&Vector3::new(3.0, 7.0, 5.0));
        assert_eq!(dist, 7.6157731058639087);
    }

    #[test]
    fn sphere_tests() {
        let a: Vector3 = Vector3::new(0.0, 0.0, 0.0);
        let b: Vector3 = Vector3::new(0.0, 0.0, 10.0);

        let sphere: Sphere = Sphere::new(&a, 2.0);

        let line_from_origin: Line3 = Line3::new(&a, &b);
        let line_from_outside: Line3 = Line3::new(&Vector3::new(0.0, 0.0, -10.0), &b);
        let line_from_beyond_sphere: Line3 = Line3::new(&b, &Vector3::new(0.0, 0.0, 20.0));
        let tangent_line: Line3 =
            Line3::new(&Vector3::new(0.0, 2.0, 10.0), &Vector3::new(0.0, 2.0, 0.0));
        let outer_line: Line3 =
            Line3::new(&Vector3::new(0.0, 2.1, 10.0), &Vector3::new(0.0, 2.1, 0.0));

        // secant line
        let instersection: List<f64> = sphere.intersects(&line_from_origin);
        assert_eq!(instersection.iter().count(), 2);
        assert_eq!(
            sphere.closest_intersection(&line_from_outside).unwrap().lambda,
            0.4
        );
        assert_eq!(sphere.closest_intersection(&line_from_origin).unwrap().lambda, 0.2);

        // tangent line
        let tangent_instersection: List<f64> = sphere.intersects(&tangent_line);
        assert_eq!(tangent_instersection.iter().count(), 1);

        // non-crossing line
        let outer_instersection: List<f64> = sphere.intersects(&outer_line);
        assert_eq!(outer_instersection.iter().count(), 0);
        assert_eq!(
            sphere
                .closest_intersection(&line_from_beyond_sphere)
                .is_none(),
            true
        );
    }

    #[test]
    fn plane_tests() {
        let plane: Plane = Plane::new(&Vector3::new(0.0, 0.0, 0.0), &Vector3::new(1.0, 1.0, 0.0));

        let horizontal_line: Line3 = Line3::new(
            &Vector3::new(0.0, 1.0, 0.0), 
            &Vector3::new(0.0, 1.0, 10.0)
        );
        let tilted_line: Line3 = Line3::new(
            &Vector3::new(0.0, -1.0, 0.0),
            &Vector3::new(0.0, 1.0, 10.0)
        );
        let in_plane_line: Line3 = Line3::new(
            &Vector3::new(0.0, 0.0, 0.0),
            &Vector3::new(-13.5e39, 13.5e39, 83810.16789),
        );

        let mut instersection: List<f64> = plane.intersects(&horizontal_line);
        assert_eq!(instersection.iter().count(), 0);
        assert_eq!(plane.closest_intersection(&horizontal_line).is_none(), true);
        assert_eq!(plane.closest_intersection(&tilted_line).unwrap().lambda, 0.5);
        assert_eq!(plane.closest_intersection(&in_plane_line).is_none(), true);

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
        let triangle: Triangle3 = Triangle3::new(&a, &b, &c);

        // barycentric function: Point matching vertexes
        let mut barycentric: Vector3 = triangle.barycentric(&a);
        assert_eq!(barycentric, Vector3::new(1.0, 0.0, 0.0));
        barycentric = triangle.barycentric(&b);
        assert_eq!(barycentric, Vector3::new(0.0, 1.0, 0.0));
        barycentric = triangle.barycentric(&c);
        assert_eq!(barycentric, Vector3::new(0.0, 0.0, 1.0));

        // Point inside the triangle
        barycentric = triangle.barycentric(&Vector3::new(0.0, 5.0, 5.0));
        assert_eq!(barycentric, Vector3::new(0.0, 0.5, 0.5));
        let ray_inside: Line3 = Line3::new(
            &Vector3::new(10.0, 5.0, 5.0), 
            &Vector3::new(0.0, 5.0, 5.0)
        );
        let intersections: List<f64> = triangle.intersects(&ray_inside);
        assert_eq!(intersections.iter().count(), 1);
        let mut intersection: Option<Intersection> = triangle.closest_intersection(&ray_inside);
        assert_eq!(intersection.unwrap().lambda, 1.0);

        // Point outside triangle
        barycentric = triangle.barycentric(&Vector3::new(0.0, 8.0, 1.0));
        assert_eq!(barycentric, Vector3::new(0.09999999999999998, 0.1, 0.8));
        let ray_outside: Line3 = Line3::new(
            &Vector3::new(10.0, 15.0, 15.0),
            &Vector3::new(0.0, 15.0, 15.0),
        );
        let intersections: List<f64> = triangle.intersects(&ray_outside);
        assert_eq!(intersections.iter().count(), 0);
        intersection = triangle.closest_intersection(&ray_outside);
        assert_eq!(intersection.is_none(), true);

        // Point outside triangle plane, but inside if projected on plane
        barycentric = triangle.barycentric(&Vector3::new(0.0, 5.0, 5.0));
        assert_eq!(barycentric, Vector3::new(0.0, 0.5, 0.5));
    }
}
