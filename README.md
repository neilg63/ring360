[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/ring360)
[![crates.io](https://img.shields.io/crates/v/ring360.svg)](https://crates.io/crates/ring360)
[![docs.rs](https://docs.rs/ring360/badge.svg)](https://docs.rs/ring360)

# Ring360: Modular Arithmetic around a 360º circle

This crate provides a simple wrapper struct for 64-bit floats representing degrees around a circle. The type lets you perform basic arithmetic operations with +, - and calculate the shortest angles between two degrees on a circle. Multiplication and division are only supported with float-64 values.

A *Ring360* object is just tuple struct encapsulating the raw the f64 value with methods to expose its *degree()*, *rotations()* or raw value(), e.g. Ring360(400.0) would have a raw value of *400.0* but *40º* and *1 rotation*.

### Add and subtract degree values
```rust
/// Add and/or subtract degrees on a circle
/// The degree value represents the longitude, while the intrinsic f64 value 
/// represents the angle travelled around the circumference
let longitude_1 = 74.7.to_360() // or Ring360(74.7);
let longitude_2 = 291.4.to_360()
let longitude_3 = 126.1.to_360();

let result_1 = longitude_1 + longitude_2 + longitude_3;

println!(
  "Degrees: {}º, intrinsic value is {}, rotations: {}",
  result_1,
  result_1.value(),
  result_1.rotations()
);

let result_2 = longitude_1 - longitude_2 + longitude_3;

println!("Degree value: {}º", result_2);

```

### Convert from and to f64
```rust
let value_1 = 74.7;
let value_2 = 291.4;

let longitude_1 = value_1.to_360();
let longitude_2 = value_2.to_360();

let result_1_f64 = (longitude_1 + longitude_2).degree();

println!(
  "{}º +  {}º equals {}º",
  longitude_1,
  longitude_2,
  result_1_f64
);

```

Multiplication and divsion are not implemented directly for degrees as Ring360 values, but only with normal f64 values via the multiply() and divide() methods.
```rust
let value_1 = 74.7;
let factor = 4.0;

let result_1 = value_1.to_360().multiply(factor);

let result_2 = value_1.to_360().divide(factor);

println!(
  "{}º multiplied by {} equals {}º",
  value_1,
  factor,
  result_1
);

println!(
  "{}º divided by {} equals {}",
  result_2,
  result_1
);

```

The angle() and angle_f64() methods calculate the shortest angle in degrees between two longitudes in a circle.
```rust
let longitude_1 = 297.4.to_360();
let value_2: f64 = 36.2;
let longitude_2 = value_2.to_360();

let result_1 = value_1.angle(longitude_2);

let result_2 = value_1.angle_f64(value_2);

println!(
  "Both results should be the same: {} == {}",
  result_2,
  result_1
);

```

### Dev Notes
This is crate is in its alpha stage. it's a proof of concept only.