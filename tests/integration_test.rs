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
    let expected_result_f64 = (Ring360::BASE + (v2 - v1)) % Ring360::BASE;
    assert_eq!(d4.to_f64(), expected_result_f64);
}

#[test]
fn test_multiply() {
    let v2 = 134.635893;
    let d2 = Ring360(v2);
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
fn test_absolute_angles() {
    let v1 = 271.5;
    let v2 = 24.5;
    let v1_to_v2_absolute_angle = 113.0;
    let v2_to_v1_absolute_angle = 247.0;
    assert_eq!(v1.angle_360_abs(v2), v1_to_v2_absolute_angle);
    assert_eq!(v2.angle_360_abs(v1), v2_to_v1_absolute_angle);
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

#[test]
fn test_rotations() {
    let v1 = -82.467352;
    let v2 = 432.202828;
    
    let d1 = v1.to_360();
    let d2 = v2.to_360();
    
    let expected_rotations_1 = -1;
    let expected_rotations_2 = 1;
    assert_eq!(d1.rotations(), expected_rotations_1 );
    assert_eq!(d2.rotations(), expected_rotations_2 );
}

#[test]
fn test_progress() {
    let v1 = -270.0;
    let v2 = 540.0;
    
    let d1 = v1.to_360();
    let d2 = v2.to_360();
    
    let expected_progress_1 = -0.75;
    let expected_progress_2 = 1.5;
    assert_eq!(d1.progress(), expected_progress_1 );
    assert_eq!(d2.progress(), expected_progress_2 );
}

#[test]
fn test_gis_180_conversions() {
    let v1 = -75.0;
    let v1_360_system = 285.0;
    
    let d1 = v1.to_360_gis();
    
    
    assert_eq!(d1.degrees(), v1_360_system );

    assert_eq!(d1.to_gis(), v1 );

    let v3 = -179.0;
    let v4 = 179.0;

    let d3 = v3.to_360_gis();
    let d4 = v4.to_360_gis();

    assert_eq!(d3.degrees(), 181.0 );

    assert_eq!(d3.angle(d4), -2.0 );

    assert_eq!((-120.0).to_360_gis().degrees(), 240.0 );

    assert_eq!((-179.0).to_360_gis().degrees(), 181.0 );
    
}

// Ensure both constructors yield the same degree value, but have different initial numbers of rotations
#[test]
fn test_from_gis_constructor() {
    
    let gis_180_value = -90.0;

    let expected_360_value = 270.0;
    let value_with_default_constructor = Ring360(gis_180_value);
    let value_with_gis_constructor = Ring360::from_gis(gis_180_value);

    assert_eq!(value_with_default_constructor.degrees(), expected_360_value);
    assert_eq!(value_with_gis_constructor.degrees(), expected_360_value);

    assert_eq!(value_with_default_constructor.rotations(), -1);
    assert_eq!(value_with_gis_constructor.rotations(), 0);
}

#[test]
fn test_mod_360() {
    
    let negative_lng = -31.5;
    let expected_360_lng = 328.5;

    assert_eq!(negative_lng.mod_360(), expected_360_lng);
}

#[test]
fn test_to_radians() {
    
    let deg_val = 77.2483;
    let lng = deg_val.to_360();
    let radian_val = deg_val / 180.0 * std::f64::consts::PI;

    assert_eq!(lng.to_radians(), radian_val);

    assert_eq!(lng.cos(), radian_val.cos());
    
}

