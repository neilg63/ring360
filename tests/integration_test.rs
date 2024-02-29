use ring360::*;

#[cfg(test)]

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
    let d5 = d2.multiply(m1);
    let expected_result_f64 = (v2 * m1) % Ring360::BASE;
    assert_eq!(d5.to_f64(), expected_result_f64);

    let d6 = d2.divide(m1);
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
fn test_angles() {
    let v1 = 271.893635;
    let v2 = 24.635893;
    
    let d1 = v1.to_360();
    let d2 = v2.to_360();
    
    let expected_angle_d1_to_d2 = (v2 + Ring360::BASE - v1) % Ring360::BASE;

    assert_eq!(d1.angle(d2), expected_angle_d1_to_d2);
    assert_eq!(d1.angle_f64(v2), expected_angle_d1_to_d2);
    // the reverse direction should be negative
    let expected_angle_d2_to_d1 = 0f64 - expected_angle_d1_to_d2;
    assert_eq!(d2.angle(d1), expected_angle_d2_to_d1);

    let v3 = 324.449474;
    let v4 = 42.356418;
    let d3 = v3.to_360();
    let d4 = v4.to_360();
    let angle = d3.angle( d4 ); // will be the shortest angle
    let target_val = (v4 + Ring360::BASE - v3) % Ring360::BASE;
    assert_eq!(angle, target_val);

    let v5 = 322.393939;
    
    let d5 = v5.to_360();
    let angle = d3.angle( d5 ); // will be the shortest angle
    let target_val = (v5 - v3) % Ring360::BASE;
    assert_eq!(angle, target_val);
    // the same result can be obained from a normal f64 value via the angle_360() mmethod
    assert_eq!(v3.angle_360(v5), target_val);
}

#[test]
fn test_multiplication() {
    let v1 = 271.893635;
    let v2 = 4.0;
    
    let d1 = Ring360(v1);
    
    let expected_result_f64 = (v1 * v2) % Ring360::BASE;

    // You can only multiply or divide by normal f64 values,
    // but the resulting Ring360 value will retain the raw f64 as well its degree
    assert_eq!((d1.multiply(v2)).to_f64(), expected_result_f64);

    let longitude_1 = 297.4.to_360();

    let value_2: f64 = 36.2;
    let longitude_2 = value_2.to_360();

    let result_1 = longitude_1.angle(longitude_2);

    let result_2 = longitude_1.angle_f64(value_2);
    assert_eq!(result_1, result_2);
}

