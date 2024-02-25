[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/ring360)
[![crates.io](https://img.shields.io/crates/v/ring360.svg)](https://crates.io/crates/ring360)
[![docs.rs](https://docs.rs/ring360/badge.svg)](https://docs.rs/ring360)

# Ring360: Modular Arithmetic around a 360º circle

This crate provides a simple wrapper struct for 64-bit floats representing degrees around a circle. The type lets you perform basic arithmetic operations with +, -, * and / and calculate the shortest distances between two degrees on a circle.

```rust
/// Add and.or subtract degrees on a circle
/// The degree value represents the longitude, while the intrinsic f64 value 
/// represents the distance travelled around the circumference
let longitude_1 = Ring360(74.7);
let longitude_2 = Ring360(291.4);
let longitude_3 = Ring360(126.1);

let result_1 = longitude_1 + longitude_2 + longitude_3;

println!(
  "Degrees: {}º, intrinsic value is {}, rotations: {}",
  result_1,
  result_1.value(),
  result_1.turns()
);

let result_2 = longitude_1 - longitude_2 + longitude_3;

println!("Degree value: {}º", result_2);

```

64-bit float values can be easily coverted to and from Ring360 values
```rust
let value_1 = 74.7;
let longitude_1 = value_1.to_360();

let longitude_2 = 291.4.to_360();

let result_1_f64 = (longitude_1 + longitude_2).to_f64();

println!(
  "{}º +  {}º equals {}º",
  longitude_1,
  longitude_2,
  result_1_f64
);

```

Multiplication and divsion are implemented for degrees as Ring360 values, but in practice multiplying by float values is more useful.
```rust
let value_1 = 74.7;
let multiple = 4.0;

let result_1 = value_1.to_360() * multiple.to_360();

let result_2 = value_1.to_360().multiply(multiple);

println!(
  "{}º *  {} equals {}º",
  value_1,
  multiple,
  result_1
);

println!(
  "Both results should be the same: {} == {}",
  result_2,
  result_1
);

```

The distance() and distance_f64() methods calculate the shortest distance in degrees between two longitudes in a circle.
```rust
let longitude_1 = 297.4.to_360();
let value_2: f64 = 36.2;
let longitude_2 = value_2.to_360();

let result_1 = value_1.distance(longitude_2);

let result_2 = value_1.distance_f64(value_2);

println!(
  "Both results should be the same: {} == {}",
  result_2,
  result_1
);

```

### Dev Notes
This is crate is in its alpha stage. it's a proof of concept only.