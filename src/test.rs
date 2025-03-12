#[cfg(test)]
mod tests {
    use crate::{Line3, Sphere, Plane};
    use vector3::Vector3;
    use list::List;

    #[test]
    fn line_tests() {
        let a = Vector3::new(0.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 0.0, 10.0);

        let line = Line3::new(&a, &b);
        assert_eq!(line.calc_point(0.), a);
        assert_eq!(line.calc_point(1.), b);
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

        // for (_i, &value) in instersection.iter().enumerate() {
        //     let point = line.calc_point(value);
        // }

        let tangent_instersection: List::<f64> = sphere.intersects(&tangent_line);
        assert_eq!(tangent_instersection.iter().count(), 1);

        let outer_instersection: List::<f64> = sphere.intersects(&outer_line);
        assert_eq!(outer_instersection.iter().count(), 0);
    }

    #[test]
    fn plane_tests() {
        let plane = Plane::new(&Vector3::new(0.0, 0.0, 0.0), &Vector3::new(1.0, 1.0, 0.0));

        let horizontal_line = Line3::new(&Vector3::new(0.0, 1.0, 0.0), &Vector3::new(0.0, 1.0, 0.0));
        let tilted_line = Line3::new( &Vector3::new(0.0, 0.0, 0.0), &Vector3::new(0.0, 1.0, 10.0));
        let in_plane_line = Line3::new(&Vector3::new(0.0, 0.0, 0.0), &Vector3::new(-13.5e39, 13.5e39, 83810.16789));

        let mut instersection: List::<f64> = plane.intersects(&horizontal_line);
        assert_eq!(instersection.iter().count(), 0);

        instersection = plane.intersects(&tilted_line);
        assert_eq!(instersection.iter().count(), 1);

        instersection = plane.intersects(&in_plane_line);
        assert_eq!(instersection.iter().count(), 0);
    }

}
