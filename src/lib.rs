use std::ops::{Add,Sub};
use std::fmt;

/// Ring360 is a tuple struct encapsulating an f64
#[derive(Debug, Clone, Copy)]
pub struct Ring360(pub f64);

/// Methods
impl Ring360 {

  /// The base is 360.0. All degree values will be modulated by this number
  pub const BASE:f64 = 360.0;

	/// Alternative constructor if the source degree uses the ±180º GIS system
  pub fn from_gis(lng180: f64) -> Ring360 {
    if lng180 < 0.0 {
			Ring360(Self::BASE + lng180)
		} else {
			Ring360(lng180)
		}
  }

	
  #[deprecated(since="0.2.3", note="please use `from_gis()` instead")]
	pub fn from_180(lng180: f64) -> Ring360 {
    Self::from_gis(lng180)
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

	/// Convert the internal 0-360º scale back to the -180º to +180º GIS scale
	pub fn to_gis(&self) -> f64 {
    if self.degrees() <= Self::half_turn() {
			self.degrees()
		} else {
			self.degrees() - Self::BASE
		}
  }

  #[deprecated(since="0.2.3", note="please use `to_gis()` instead")]
	pub fn to_180(&self) -> f64 {
    self.to_gis()
  }

  /// Get the number of rotations. If the total is less than base of 360
  pub fn rotations(&self) -> i64 {
    (self.0 / Self::BASE).floor() as i64
  }

	/// Returns the raw internal f64 value on a 0-360º scale. 
	/// Values under 0 or over 360 represent negative or positive rotations
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

	/// Convert to radians for use with cos(), sin(), tan(), atan() etc.
  pub fn to_radians(&self) -> f64 {
    self.degrees().to_radians()
  }

	/// Calculate sine in the 360º system without explicity converting to radians
  pub fn sin(&self) -> f64 {
    self.to_radians().sin()
  }

	/// Calculate cosine in the 360º system without explicity converting to radians
  pub fn cos(&self) -> f64 {
    self.to_radians().cos()
  }

	/// Calculate tangent in the 360º system without explicity converting to radians
  pub fn tan(&self) -> f64 {
    self.to_radians().tan()
  }

	/// Calculate inverse cosine in the 360º system without explicity converting to and from radians
  pub fn asin(&self) -> f64 {
    self.to_radians().asin()
  }

	/// Calculate inverse cosine in the 360º system without explicity converting to and from radians
  pub fn acos(&self) -> f64 {
    self.to_radians().acos()
  }

	/// Calculate inverse tangent (arctan, aatan) in the 360º system without explicity converting to radians
  pub fn atan(&self) -> f64 {
    self.to_radians().atan()
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

impl fmt::Display for Ring360 {
  /// By default display the circular degree value
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.degrees())
  }
}

/// trait to convert normal float values to a Ring360 value 
/// and to apply a simple mod_360() returning a 64-bit float
pub trait ToRing360 {
	fn to_360(&self) -> Ring360;
  fn to_360_gis(&self) -> Ring360;
	fn mod_360(&self) -> Self;
  fn angle_360(&self, other_value: f64) -> Self;
}

/// Implement casting methods for f64
impl ToRing360 for f64 {
	fn to_360(&self) -> Ring360 {
		Ring360(*self)
	}

  fn to_360_gis(&self) -> Ring360 {
		Ring360::from_gis(*self)
	}

	fn mod_360(&self) -> f64 {
		Ring360(*self).to_f64()
	}

  fn angle_360(&self, other_value: f64) -> f64 {
		Ring360(*self).angle_f64(other_value)
	}

}

