[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/ring360)
[![crates.io](https://img.shields.io/crates/v/ring360.svg)](https://crates.io/crates/ring360)
[![docs.rs](https://docs.rs/ring360/badge.svg)](https://docs.rs/ring360)

# Ring360: Modular Arithmetic around a 360º circle

This crate provides a versatile wrapper struct for representing degrees in a circular range, designed around 64-bit floats. The primary type, `Ring360`, supports intuitive degree manipulation with `+` and `-` operators, and provides directional calculations for the shortest angle between two degrees. This angle calculation returns a positive value for clockwise movement and a negative value for counterclockwise, making it ideal for applications where direction matters. Additionally, an `abs()` method can be used to get the absolute shortest distance regardless of direction.

`Ring360` is implemented as a tuple struct, encapsulating an `f64` value with methods for accessing `degrees()`, `rotations()`, and `value()`. For example, `Ring360(400.0)` would have a raw value of `400.0`, representing `40º` in degrees with `1` rotation.

Values are kept within a circular 360º range, so negative values are automatically converted to their positive counterparts in the opposite direction (e.g., `-60º` is represented as `300º`). Recognizing that geographic coordinates often use a ±180º range, the crate includes GIS-compatible conversions. The `to_360_gis()` method converts ±180º to the 360º format, and `Ring360::from_gis(lng180: f64)` performs the reverse conversion. This way, the crate seamlessly adapts to GIS systems such as [QGIS](https://qgis.org/), where angles between 180º and 360º are often expressed in a ±180º format.

These conversions preserve the actual angle (e.g., `-60º` in ±180º converts directly to `300º`), though they may adjust the rotation count. Standard methods like `to_360()` and `degrees()` offer consistent handling of degrees within the 360º range.

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

### Multiplication and Division

These are not implemented directly for degrees as Ring360 values, but only with primitive f64 values via the multiply() and divide() methods.

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

### Angular Distance

The _angle()_ and _angle_f64()_ methods calculate the shortest angle in degrees between two longitudes in a circle. Negative values indicate an anticlockwise offset from A to B, e.g. from 340º to 320º would be negative, while 350º to 10º would be positive.

```rust
let value_1 = 297.4;
let longitude_1 = value_1.to_360();
let value_2: f64 = 36.2;
let longitude_2 = value_2.to_360();

let result_1 = longitude_1.angle(longitude_2);

let result_2 = longitude_1.angle_f64(value_2);

println!(
  "Both results should be the same: {} == {}",
  result_2,
  result_1
);

```

### Absolute undirectional angles

The `angle_abs()` and `angle_f64_abs()` methods calculate the absolute angle in degrees in a clockwise direction between two longitudes in a circle. Values over 180.0 represent more than half a turn. For `f64` value you may use `.angle_360_abs(other_value: f64)`.

```rust
let value_1 = 270.0;
let longitude_1 = value_1.to_360();
let value_2: f64 = 30.0;
let longitude_2 = value_2.to_360();

let result_1 = longitude_1.angle_abs(longitude_2);
let result_2 = longitude_2.angle_abs(longitude_1);

println!(
  "The angles may be anywhere in the 0 to 359.9999º range: {} and {}",
  result_2,
  result_1
);
```

### Calculate sine, cosine and tangent directly

```rust
let value_1 = 45.0;
  let degree_1 = value_1.to_360();

println!(
  "The sine of {}º is {:.12}",
  degree_1.degrees(),
  degree_1.sin()
);
// Should print: The sine of 45º is 0.707106781187

let value_2 = 60.0;
let degree_2 = value_2.to_360();

println!(
  "The cosine of {}º is {:.1}",
  degree_2.degrees(),
  degree_2.cos()
);
// Should print: The cosine of 60º is 0.5

```

### Instance Methods

- _degrees()-> f64_ Degree value from 0º to 360º.
- _to_f64() -> f64_ Alias for degrees() and the default display value
- _to_gis() -> f64_ Convert the internal 0-360º scale to the -180º to +180º GIS scale
- _rotations() -> i64_ Number of rotations required to reach the current raw value, e.g. 730.0 would require 2 rotations with a degree value of 10.0.
- _progress() -> f64_ the instrinsic value expressed as decimal fractions of progress around a circle, i.e. 180º translates to 0.5 and 450º to 1.25
- _value() -> f64_ Raw f64 value
- _as_tuple() -> (f64, i64)_ Return a tuple with degrees as f64 and rotations as i64
- _multiply(multiple: f64) -> Self_ Multiply a Ring360 value by a normal f64 value
- _divide(divisor: f64) -> Self_ Divide a Ring360 by a normal f64 value
- _angle_f64(other_value: f64) -> f64_ Calculate the shortest distance in degrees between a Ring360 value and a 64-bit float representing a degree. A positive value represents clockwise movement between the first and second longitude
- _angle(other_value: Ring360) -> f64_ Angular distance between to Ring360 values
- _to_radians() -> f64_ convert to radians for interoperability
- _sin() -> f64_ sine (with implicit radian conversion, e.g. 30.to_360().sin() yields 0.5)
- _cos() -> f64_ cosine
- _tan() -> f64_ tangent
- _asin() -> f64_ inverse sine
- _acos() -> f64_ inverse cosine
- _atan() -> f64_ inverse tangent

#### Static methods

- _half_turn() -> f64_ Half of the 360º base, i.e. 180.0

#### Constants

- _BASE:f64 = 360.0_

## Traits

### ToRing360

This is implemented only for _f64_ with the following methods:

- _to_360() -> Ring360_ converts any 64-bit float representing a degree in the 360º system to Ring360 value;
- _to_360_gis() -> Ring360_ converts any 64-bit float representing a degree in the ±180º system and normalises on the 0-360º scale.
- _mod_360() -> f64_ A convenience method for _% 360.0_, but treating negative values as positive degrees in reverse, e.g. _(-20.0).mod_360_ equals _340º_
- _angle_360(other_value: f64) -> f64_: Calculates the shortest angle between two f64 values as degrees around a circle.
- _angle_360_abs(other_value: f64) -> f64_: Calculates the absolute unidirectional angle between two f64 values as degrees around a circle.

### Dev Notes

This is crate is in development, but implements all core features.

### Version notes

#### 0.2.11

- _angle_abs() -> f64_ Returns the absolute undirectional angle, with related `angle_f64_abs()` and `angle_360_abs()` methods

#### 0.2.8

- _progress() -> f64_ Returns the rotations as decimal progress around a circle

#### 0.2.6

The following methods have been added to Ring360

- to_radians() for interoperability with other maths functions

The core trigonometric methods, sin(), cos(), tan(), asin(), acos() and atan() are calculated from degrees without explicitly converting to radians first:

- _.sin() -> f64_ calculates sine
- _.cos() -> f64_ calculates cosine
- _.tan() -> f64_ calculates tangent
- _.asin() -> f64_ calculates inverse sine (arcsine, asin)
- _.acos() -> f64_ calculates inverse cosine (arccoine, acos)
- _.atan() -> f64_ calculates inverse tangent (arctan, atan)

#### 0.2.5

The following deprecated methods were removed from version 0.2.5:

- _degree()_ => use _degrees()_ instead.
- _turns()_ => use _rotations()_ instead.

### 0.2.10

- the deprecated _Ring360::from_180(-90.0)_ constructor and _Ring360.to_180()_ method have been removed. Use _Ring360::from_gis(-90.0)_ and _Ring360.to_360_gis()_ instead. NB: This does not affect the calculated 360º degree value. It only affects only the calculated number of rotations, where _-30_ would convert to 330º with both to_360() and to_360_gis(), but have -1 rotations with the default cast method, but 0 rotations with the f64.to_360_gis().

### 0.2.12

- Simplified the logic in ring360's `angle_f64()` method and added a new static method `minus_half_turn()` (-180).
