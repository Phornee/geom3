# geom3

## What is it?
`geom3` is a rust library for dealing with 3D basic geometric calculations, such as Lines, Spheres or Planes.

## What does it do?
You can create/operate with 3D geometric shapes, and calculate intersections, reflections, etc
It uses `vector3` project for managing 3d vectors (https://docs.rs/crate/vector3/latest)

## How do I get started?
Make sure you have a project set up using `cargo` then:

### If using `cargo-edit`: 
`cd` into the said project directory and execute
```
cargo add geom3
```

### If not using `cargo-edit`:
Go to this crate's crates.io page and look right

## Pending:
Proper unit tests

## Examples:
`geom3` uses the vectorial form of the line to calculate intersections. It will calculate only the lambda of each intersection This provides:
### Better performace: If you just want to know if there is intersection, It Will be faster
### If you need the point of each intersection, you can easily calculate it with the calc_point function of the `Line3`.
Please, take a look to the unittests for more examples
```