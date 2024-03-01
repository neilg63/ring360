[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/ring360)
[![crates.io](https://img.shields.io/crates/v/ring360.svg)](https://crates.io/crates/ring360)
[![docs.rs](https://docs.rs/ring360/badge.svg)](https://docs.rs/ring360)

# Ring360: Modular Arithmetic around a 360º circle

This crate provides a simple wrapper struct for 64-bit floats representing degrees around a circle. The type lets you perform basic arithmetic operations with +, - and calculate the shortest angles between two degrees on a circle. Multiplication and division are only supported with float-64 values.

A *Ring360* type is just tuple struct encapsulating the raw the f64 value with methods to expose its *degrees()*, *rotations()* or raw value(), e.g. Ring360(400.0) would have a raw value of *400.0* but *40º* and *1 rotation*.

As this data type works around a 360º circle, negative raw values yield positive degrees in the opposite direction, e.g. *-60* is *300º* . Degree longitudes around the Earth's circumference are customarily expressed as ±180º. In this case, longitudes between 0º would be 180º are the same but between 180º and 360º, they descend from -180º to º0. This means -120º on the ±180 system is 240º in the 360º system. To this end, the crate has methods to convert from and to the GIS ±180 system, i.e. *value_f64.to_360_gis()* or *Ring360_from_gis(lng180: f64)*. 

### Add and subtract degree values
```rust
/// Add and/or subtract degrees on a circle
/// The degree value represents the longitude, while the intrinsic f64 value 
/// represents the angle travelled around the circumference
let longitude_1 = 74.7.to_360(); // or Ring360(74.7);
let longitude_2 = 291.4.to_360();
let longitude_3 = 126.1.to_360();

let result_1 = longitude_1 + longitude_2 + longitude_3;

println!(
  "Degrees: {:.1}º, intrinsic value is {:.1}, rotations: {}",
  result_1.degrees(),
  result_1.value(),
  result_1.rotations()
);
/// Should read: Degrees: 132.2º, intrinsic value is 492.2, rotations: 1

let result_2 = longitude_1 - longitude_2 + longitude_3;

println!("Degree value: {}º", result_2);
/// Should read: Degree value: 269.4º
```

### Use ±180 GIS system, -180º to 180º
```rust
/// Declare two degrees on the GIS scale
let gis_lng_1 = 143.32;
let gis_lng_2 = -111.4;

/// Shift ±180 degrees to a 0º to 360º scale
let lng_360_1 = gis_lng_1.to_360_gis(); // or Ring360::rom_gis(74.7);
let lng_360_2 = gis_lng_2.to_360_gis();

let angle_3 = lng_360_1.angle(lng_360_2);

println!("The longitudinal distance between {} and {} is {}", lng_360_1.to_gis(), lng_360_2.to_gis(), angle_3);
/// The longitudinal distance between 143.32 and -111.4 is 105.28

```

### Convert from and to f64
```rust
let value_1 = 74.7;
let value_2 = 291.4;

let longitude_1 = value_1.to_360();
let longitude_2 = value_2.to_360();

let result_1_f64 = (longitude_1 + longitude_2).degrees();

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

println!(
  "{}º multiplied by {} equals {}º",
  value_1,
  factor,
  result_1
);

let result_2 = value_1.to_360().divide(factor);

println!(
  "{}º divided by {} equals {}",
  value_1,
  factor,
  result_2
);

```

The *angle()* and *angle_f64()* methods calculate the shortest angle in degrees between two longitudes in a circle. Negative value indicate an anticlockwise offset from A to B, e.g. from 340º to 320º would be negative, while 350º to 10º would be positive.
```rust
let value_1 = 297.4;
let longitude_1 = value_1.to_360();
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

## Traits

### ToRing360

This is implemented only for *f64* with the following methods:

- *to_360(&self) -> Ring360* converts any 64-bit float representing a degree in tne 360º system to Ring360 value;
- *to_360_gis(&self) -> Ring360* converts any 64-bit float representing a degree in tne ±180º system and normalises on the 0-360º scale.
- *mod_360(&self) -> f64* A convenience method for *% 360.0*
- *angle_360(&self, other_value: f64) -> f64*: Calculates the shortest angle between two f64 values as degrees around a circle.

### Dev Notes
This is crate is in its alpha stage, but feature-complete.