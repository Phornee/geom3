# vector3

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
```rust
let a = Vector3::from_i32(1, 2, 3);
let b = Vector3::from_i32(1,2,3);

assert_eq!(a.dot(&b), 14.0);
```

```rust
let a = Vector3::from_i32(1, 2, 3);
let b = Vector3::from_i32(3, 2, 1);

assert_eq!(a.cross(&b), Vector3::from_i32(-4, 8, -4));
```

```rust
let a = Vector3::from_i32(1, 0, 0);
let b = Vector3::from_i32(0, 0, 1);

assert_eq!(a.angle(b) * (180.0 / PI), 90.0);
```