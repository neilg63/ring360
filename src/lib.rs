use std::ops::{Add,Sub};

/// Ring360 is a tuple struct encapsulating an f64
#[derive(Debug, Clone, Copy)]
pub struct Ring360(pub f64);

/// Methods
impl Ring360 {

  /// The base is 360.0. All degree values will be modulated by this number
  pub const BASE:f64 = 360.0;

  /// Get the degree value as a 64-bit float
  pub fn to_f64(&self) -> f64 {
    self.0 % Self::BASE
  }

  /// Alias for the above
  pub fn degree(&self) -> f64 {
    self.0 % Self::BASE
  }

  /// Get the number of rotations. If the total is less than base of 360
  pub fn rotations(&self) -> i64 {
    (self.0 / Self::BASE).floor() as i64
  }

  #[deprecated(since="0.1.0", note="please use `rotations()` instead")]
  pub fn turns(&self) -> i64 {
    self.rotations()
  }

  pub fn value(&self) -> f64 {
    self.0
  }

  /// Return a simple tuple pair with the 
  /// 360º degree value and the number of rotations (turns)
  pub fn as_tuple(&self) -> (f64, i64) {
    (self.to_f64(), self.rotations())
  }

  /// Multiply a Ring360 value by a normal f64 value
  pub fn multiply(mut self, multiple: f64) -> Self {
    self.0 *= multiple;
    self
  }

  /// Divide a Ring360 by a normal f64 value
  pub fn divide(mut self, divisor: f64) -> Self {
    self.0 /= divisor;
    self
  }

  /// Calculate the shortest distance in degrees between a Ring360 value
  /// and a float64 representing a degree
  pub fn angle_f64(&self, other_value: f64) -> f64 {
    let diff = other_value - self.to_f64();
    if diff.abs() <= 180f64 {
      diff
    } else {
      if diff < 0f64 {
        0f64 - Self::BASE - diff
      } else {
        Self::BASE - diff
      }
    }
  }

  /// Calculate the shortest distance in degrees between 
  /// two a Ring360 values
  pub fn angle(&self, other_value: Ring360) -> f64 {
    self.angle_f64(other_value.to_f64())
  }

}

/// Implement + (addition) operator with two Ring30 values
impl Add for Ring360 {

  type Output = Ring360;

  fn add(mut self, other: Ring360) -> Self {
    self.0 += other.value();
    self
  }
}

/// Implement - (subtraction) operator with two Ring30 values
impl Sub for Ring360 {

  type Output = Ring360;

  fn sub(mut self, other: Ring360) -> Self {
    self.0 -= other.value();
    self
  }
}

/// trait to convert normal float values to a Ring360 value 
/// and to apply a simple mod_360() returning a 64-bit float
pub trait ToRing360 {
	fn to_360(&self) -> Ring360;
	fn mod_360(&self) -> Self;
}

/// Implement casting methods for f64
impl ToRing360 for f64 {
	fn to_360(&self) -> Ring360 {
		Ring360(*self)
	}

	fn mod_360(&self) -> f64 {
		Ring360(*self).to_f64()
	}
}

