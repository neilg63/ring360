use std::ops::{Add,Sub,Mul,Div};
use std::fmt;

/// Ring360 is a tuple struct encapsulating an f64
#[derive(Debug, Clone, Copy)]
pub struct Ring360(f64);

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
  pub fn turns(&self) -> i64 {
    (self.0 / Self::BASE).floor() as i64
  }

  pub fn value(&self) -> f64 {
    self.0
  }

  /// Return a simple tuple pair with the 
  /// 360º degree value and the number of rotations (turns)
  pub fn as_tuple(&self) -> (f64, i64) {
    (self.to_f64(), self.turns())
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
  pub fn distance_f64(&self, other_value: f64) -> f64 {
    let diff = self.to_f64() - other_value;
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
  pub fn distance(&self, other_value: Ring360) -> f64 {
    self.distance_f64(other_value.to_f64())
  }

}

impl Add for Ring360 {

  type Output = Ring360;

  fn add(mut self, other: Ring360) -> Self {
    self.0 += other.value();
    self
  }
}

impl Sub for Ring360 {

  type Output = Ring360;

  fn sub(mut self, other: Ring360) -> Self {
    self.0 -= other.value();
    self
  }
}


impl Mul for Ring360 {

  type Output = Ring360;

  fn mul(mut self, other: Ring360) -> Self {
    self.0 *= other.value();
    self
  }
}

impl Div for Ring360 {

  type Output = Ring360;

  fn div(mut self, other: Ring360) -> Self {
    self.0 /= other.value();
    self
  }
}

impl fmt::Display for Ring360 {
  /// By default display the circular degree value
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", self.to_f64())
  }
}

pub trait ToRing360 {
    fn to_360(&self) -> Ring360;
    fn mod_360(&self) -> Self;
}

impl ToRing360 for f64 {
    fn to_360(&self) -> Ring360 {
        Ring360(*self)
    }

    fn mod_360(&self) -> f64 {
        Ring360(*self).to_f64()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operations() {
        let v1 = 271.893635;
        let v2 = 134.635893;
        let expected_result_f64 = (v1 + v2) % Ring360::BASE;
        let d1 = Ring360(v1);
        let d2 = Ring360(v2);
        let d3 = d1 + d2;
        assert_eq!(d3.to_f64(), expected_result_f64);

        let d4 = d2 - d1;
        let expected_result_f64 = (v2 - v1) % Ring360::BASE;
        assert_eq!(d4.to_f64(), expected_result_f64);

        let m1 = 4.0;
        let d5 = d2 * m1.to_360();
        let expected_result_f64 = (v2 * m1) % Ring360::BASE;
        assert_eq!(d5.to_f64(), expected_result_f64);

        let d6 = d2 / m1.to_360();
        let expected_result_f64 = (v2 / m1) % Ring360::BASE;
        assert_eq!(d6.to_f64(), expected_result_f64);

        let m2 = 5.0;
        let d7 = d2.multiply(m2);
        let expected_result_f64 = (v2 * m2) % Ring360::BASE;
        assert_eq!(d7.to_f64(), expected_result_f64);


    }

    #[test]
    fn test_conversion() {
        let v1 = 271.893635;
        let v2 = 134.635893;
        let v3 = 99.056;
        let new_degree_f64 = (v1 + v2 + v3).mod_360();
        let expected_result_f64= (v1 + v2 + v3) % Ring360::BASE;
        let sum_as_ring_360 = (v1 + v2 + v3).to_360();
        let expected_rotations = ((v1 + v2 + v3) / Ring360::BASE).floor() as i64;
        assert_eq!(new_degree_f64, expected_result_f64);
        assert_eq!(sum_as_ring_360.as_tuple(), (expected_result_f64, expected_rotations));
    }

    #[test]
    fn test_distance() {
        let v1 = 271.893635;
        let v2 = 24.635893;
        
        let d1 = Ring360(v1);
        let d2 = Ring360(v2);
        
        let expected_distance_d1_to_d2 = (v2 + Ring360::BASE - v1) % Ring360::BASE;

        assert_eq!(d1.distance(d2), expected_distance_d1_to_d2);
        assert_eq!(d1.distance_f64(v2), expected_distance_d1_to_d2);
        // the reverse direction should be negative
        let expected_distance_d2_to_d1 = 0f64 - expected_distance_d1_to_d2;
        assert_eq!(d2.distance(d1), expected_distance_d2_to_d1);
    }

    #[test]
    fn test_multiplication() {
        let v1 = 271.893635;
        let v2 = 4.0;
        
        let d1 = Ring360(v1);
        let d2 = Ring360(v2);
        
        let expected_result_f64 = (v1 * v2) % Ring360::BASE;

        assert_eq!((d1 * d2).to_f64(), expected_result_f64);

        assert_eq!((d1.multiply(v2)).to_f64(), expected_result_f64);

        let longitude_1 = 297.4.to_360();

        let value_2: f64 = 36.2;
        let longitude_2 = value_2.to_360();

        let result_1 = longitude_1.distance(longitude_2);

        let result_2 = longitude_1.distance_f64(value_2);
        assert_eq!(result_1, result_2);
    }

}


