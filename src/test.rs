#[cfg(test)]
mod tests {
    use crate::Line3;
    use vector3::Vector3;

    #[test]
    fn new() {
        let _new = Line3::new(Vector3::new(0., 0., 0.), Vector3::new(1., 1., 1.));
        println!("{}", _new);
        assert_eq!(true, true);
    }
}
