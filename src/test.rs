#[cfg(test)]
mod tests {
    use crate::Line3;
    use vector3::Vector3;

    #[test]
    fn new() {
        let _new = Line3::new(Vector3::new(0., 0., 0.), Vector3::new(1., 1., 1.));
        println!("{}", _new);
        assert_eq!(true, true);


        let a = Vector3::new(0.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 0.0, 10.0);

        let sphere = Sphere::new(a, 2.0);

        let line = Line3::new(a, b);
        let tangent_line = Line3::new( Vector3::new(0.0, 2.0, 10.0),  Vector3::new(0.0, 2.0, 0.0));

        let instersection: List::<f64> = sphere.intersects(&line);

        for (_i, &value) in instersection.iter().enumerate() {
            let point = line.calc_point(value);
        }

        let tangent_instersection: List::<f64> = sphere.intersects(&tangent_line);
        for (_i, &value) in tangent_instersection.iter().enumerate() {
            let point = line.calc_point(value);
        }

    }
}
