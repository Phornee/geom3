# geom3

## What is it?
`geom3` is a rust library for dealing with 3D basic geometric calculations, such as Lines, Spheres, Planes or Triangles (including barycentric coordinates).
This low level library focuses only in the mathematical side of things, to be as flexible as possible: if you intend to use it for example for a raytracer, this library provides the mathematical base for it, but WONT provide specific concepts like "Rays", "Collisions", "UV coordinates", "Materials" of "Meshes". You can use this library and build your own specific classes for that.
The main focus of this package is performance. All formulas try to be optimal in terms of performance, using the most efficient algorithms i could find, and precalculating everything that is suitable to be extracted from the critical functions.
If you find something that could be optimized, please don´t hesitate to contact me.
Its the perfect toolset if you plan to start your own raytracer.

## What does it do?
You can create/operate with 3D geometric shapes, and calculate intersections, reflections, etc
It uses `vector3` project for managing 3d vectors (https://docs.rs/crate/vector3/latest)

## How do I get started?
Make sure you have a project set up using `cargo` then:
```
cargo add geom3
```
## Changelog
* 1.0.1: Breaking changes: changed interface to get input parameters as references instead of values
* 1.0.3: Better documentation
* 1.1.0: 
    * Triangle3: Class to detect insterections with 3D triangles. 
    * Support for barycentric coordinates for the Triangle3
    * Line3::dist_point (Calculates the minimum distance between a point and a line)

## Pending:
* Documentation with explanation of formulas in all functions
* Performance profiling with 'Vector3' and 'List': Are there better options?.


## Examples:
`geom3` uses the vectorial form of the line to calculate intersections. It will calculate only the lambda of each intersection.
```rust
let sphere = Sphere::new(&Vector3::new(0.0, 0.0, 0.0), 2.0);
let line = Line3::new(&Vector3::new(0.0, 0.0, 0.0), &Vector3::new(0.0, 0.0, 10.0));
let instersection: List::<f64> = sphere.intersects(&line);
for (_i, &lambda) in instersection.iter().enumerate() {
     let point = line.calc_point(lambda);
}
```
This provides:
* Better performace: If you just want to know if there is intersection, It will be faster
* If you need the point of each intersection, you can easily calculate it with the calc_point function of the `Line3`.
* More flexibility: if you want not only to know if it intersects, but also if it intersects "behind" or "beyond" the line director vector, you can easily do it checking if lambda is negative or higher than 1.0

Please, take a look to the unittests for more examples

