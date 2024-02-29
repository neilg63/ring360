use std::ops::{Add,Sub};

/// Ring360 is a tuple struct encapsulating an f64
#[derive(Debug, Clone, Copy)]
pub struct Ring360(pub f64);

/// Methods
impl Ring360 {

  /// The base is 360.0. All degree values will be modulated by this number
  pub const BASE:f64 = 360.0;

	/// Alternative constructor if the source degree uses the ±180º system
	pub fn from_180(lng180: f64) -> Ring360 {
    Ring360(lng180 + Self::half_turn())
  }

  /// Alias for the above
  pub fn degrees(&self) -> f64 {
    let deg_val = self.0 % Self::BASE;
		if deg_val < 0.0 { 
			Self::BASE - (0.0 - deg_val)
		} else {
			deg_val
		}
  }

	/// Same as degrees(), but is the default f64 conversion
  pub fn to_f64(&self) -> f64 {
    self.degrees()
  }

	#[deprecated(since="0.1.0", note="please use `degrees()` instead")]
	pub fn degree(&self) -> f64 {
    self.degrees()
  }

	pub fn to_180(&self) -> f64 {
    self.degrees() - Self::half_turn()
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

	pub fn half_turn() -> f64 {
		Self::BASE / 2.0
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
	/// A positive value represents clockwise movement between the first and second longitude
  pub fn angle_f64(&self, other_value: f64) -> f64 {
    let diff = (other_value % Self::BASE) - self.degrees();
    if diff.abs() <= Self::half_turn() {
      diff
    } else {
      let alt_val = (Self::BASE + diff) % Self::BASE;
			if alt_val < Self::half_turn() {
				alt_val
			} else {
				alt_val - Self::BASE
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
  fn angle_360(&self, other_value: f64) -> Self;
}

/// Implement casting methods for f64
impl ToRing360 for f64 {
	fn to_360(&self) -> Ring360 {
		Ring360(*self)
	}

	fn mod_360(&self) -> f64 {
		Ring360(*self).to_f64()
	}

  fn angle_360(&self, other_value: f64) -> f64 {
		Ring360(*self).angle_f64(other_value)
	}

}

