use std::f32::INFINITY;

use tiny_vect::Vect2;

#[test]
fn test_cross_product() {
    let vector1 = Vect2::new(1.0, 0.0);
    let vector2 = Vect2::new(0.0, 1.0);
    assert_eq!(vector1.cross(&vector2), 1.0);
}

#[test]
#[should_panic(expected = "Vect2::cross produced NaN or infinity")]
fn test_cross_product_max() {
    let vector1 = Vect2::new(f32::MAX, f32::MAX);
    let vector2 = Vect2::new(f32::MAX, f32::MAX);
    let result = vector1.cross(&vector2);
    assert_eq!(result, INFINITY);
}

#[test]
fn test_dot_product() {
    let vector1 = Vect2::new(1.0, 0.0);
    let vector2 = Vect2::new(0.0, 1.0);
    assert_eq!(vector1.dot(&vector2), 0.0);
}

#[test]
#[should_panic(expected = "Vect2::dot produced NaN or infinity")]
fn test_dot_product_max() {
    let vector1 = Vect2::new(f32::MAX, f32::MAX);
    let vector2 = Vect2::new(f32::MAX, f32::MAX);
    let result = vector1.dot(&vector2);
    assert_eq!(result, INFINITY);
}

#[test]
fn test_length() {
    let vector = Vect2::new(1.0, 0.0);
    assert_eq!(vector.length(), 1.0);
}

#[test]
#[should_panic(expected = "Vect2::length produced NaN or infinity")]
fn test_length_max() {
    let vector = Vect2::new(f32::MAX, f32::MAX);
    let result = vector.length();
    assert_eq!(result, f32::INFINITY);
}

#[test]
fn test_normalize() {
    let vector = Vect2::new(1.0, 0.0);
    let normalized = vector.normalize();
    assert_eq!(normalized.length(), 1.0);
}

#[test]
#[should_panic(expected = "Vect2::normalize produced non-finite result")]
fn test_normalize_max() {
    let vector = Vect2::new(f32::MAX, f32::MAX);
    let normalized = vector.normalize();
    assert_eq!(normalized.length(), 1.0);
}

#[test]
fn test_distance() {
    let vector1 = Vect2::new(1.0, 0.0);
    let vector2 = Vect2::new(0.0, 1.0);
    assert_eq!(vector1.distance(&vector2), 2.0_f32.sqrt());
}

#[test]
fn test_max_add() {
    let vector1 = Vect2::new(f32::MAX, f32::MAX);
    let vector2 = Vect2::new(f32::MAX, f32::MAX);
    let result = vector1 + vector2;
    assert_eq!(result.x, INFINITY);
    assert_eq!(result.y, INFINITY);
}

#[test]
#[should_panic(expected = "Vect2 overflow in add")]
fn test_max_checked_add_panics() {
    let vector1 = Vect2::new(f32::MAX, f32::MAX);
    let vector2 = Vect2::new(f32::MAX, f32::MAX);
    let _ = vector1.debug_checked_add(vector2); // Should panic on overflow
}

#[test]
fn test_debug_checked_add_safe() {
    let a = Vect2::new(1.0, 2.0);
    let b = Vect2::new(3.0, 4.0);
    let result = a.debug_checked_add(b);
    assert_eq!(result, Vect2::new(4.0, 6.0));
}

#[test]
fn test_from_array_f32() {
    let v: Vect2 = [1.0, 2.0].into();
    assert_eq!(v, Vect2::new(1.0, 2.0));
}

#[test]
fn test_from_tuple_f32() {
    let v: Vect2 = (3.0, 4.0).into();
    assert_eq!(v, Vect2::new(3.0, 4.0));
}

#[test]
fn test_from_array_i32() {
    let v: Vect2 = [5, 6].into();
    assert_eq!(v, Vect2::new(5.0, 6.0));
}

#[test]
fn test_from_tuple_i32() {
    let v: Vect2 = (7, 8).into();
    assert_eq!(v, Vect2::new(7.0, 8.0));
}

#[test]
fn test_into_array() {
    let v = Vect2::new(1.0, 2.0);
    let arr: [f32; 2] = v.into();
    assert_eq!(arr, [1.0, 2.0]);
}

#[test]
fn test_ops_from_tuple() {
    let v1: Vect2 = (1.0, 2.0).into();
    let v2: Vect2 = (3.0, 4.0).into();
    let sum = v1 + v2;
    let diff = v1 - v2;
    let prod = v1 * 2.0;
    let quot = v2 / 2.0;

    assert_eq!(sum, Vect2::new(4.0, 6.0));
    assert_eq!(diff, Vect2::new(-2.0, -2.0));
    assert_eq!(prod, Vect2::new(2.0, 4.0));
    assert_eq!(quot, Vect2::new(1.5, 2.0));
}

#[test]
fn test_debug_checked_from_tuple() {
    let v1: Vect2 = (1.0, 2.0).into();
    let v2: Vect2 = (3.0, 4.0).into();

    let result = v1.debug_checked_add(v2);
    assert_eq!(result, Vect2::new(4.0, 6.0));
}

#[test]
#[should_panic(expected = "Vect2 overflow in add")]
fn test_debug_checked_add_from_max_tuple() {
    let v1: Vect2 = (f32::MAX, f32::MAX).into();
    let v2: Vect2 = (f32::MAX, f32::MAX).into();
    let _ = v1.debug_checked_add(v2);
}

#[test]
fn test_try_from_slice_f32() {
    let slice: &[f32] = &[1.0, 2.0];
    let v = Vect2::try_from(slice).unwrap();
    assert_eq!(v, Vect2::new(1.0, 2.0));
}

#[test]
fn test_try_from_slice_i32() {
    let slice: &[i32] = &[3, 4];
    let v = Vect2::try_from(slice).unwrap();
    assert_eq!(v, Vect2::new(3.0, 4.0));
}

#[test]
fn test_try_from_bad_slice() {
    let bad: &[f32] = &[1.0];
    assert!(Vect2::try_from(bad).is_err());
}

#[test]
fn test_from_array_f32_addv() {
    let base: Vect2 = [1.0, 2.0].into();
    let _ = base + Vect2::new(1.0, 1.0);
}

#[test]
fn test_from_tuple_f32_dot() {
    let base: Vect2 = (1.0, 2.0).into();
    let _ = base.dot(&Vect2::new(1.0, 1.0));
}

#[test]
fn test_from_slice_i32_neg() {
    let base: Vect2 = (&[1, 2][..]).try_into().unwrap();
    let _ = -base;
}

#[test]
fn test_from_array_i32_mul2_0() {
    let base: Vect2 = [1, 2].into();
    let _ = base * 2.0;
}

#[test]
fn test_from_slice_f32_cross() {
    let base: Vect2 = (&[1.0, 2.0][..]).try_into().unwrap();
    let _ = base.cross(&Vect2::new(1.0, 1.0));
}
