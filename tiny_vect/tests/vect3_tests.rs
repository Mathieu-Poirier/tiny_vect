use tiny_vect::Vect3;

const EPS: f32 = 1e-6;

// --- Conversions & Into ---
#[test]
fn from_array_f32() {
    let v = Vect3::from([1.0, 2.0, 3.0]);
    assert_eq!(v, Vect3::new(1.0, 2.0, 3.0));
}

#[test]
fn from_tuple_f32() {
    let v = Vect3::from((4.0, 5.0, 6.0));
    assert_eq!(v, Vect3::new(4.0, 5.0, 6.0));
}

#[test]
fn from_array_i32() {
    let v = Vect3::from([7, 8, 9]);
    assert_eq!(v, Vect3::new(7.0, 8.0, 9.0));
}

#[test]
fn from_tuple_i32() {
    let v = Vect3::from((1, 2, 3));
    assert_eq!(v, Vect3::new(1.0, 2.0, 3.0));
}

#[test]
fn into_array() {
    let v = Vect3::new(1.1, 2.2, 3.3);
    let arr: [f32; 3] = v.into();
    assert_eq!(arr, [1.1, 2.2, 3.3]);
}

// --- TryFrom slices ---
#[test]
fn try_from_f32_slice_ok() {
    let v: Vect3 = (&[1.0, 2.0, 3.0][..]).try_into().unwrap();
    assert_eq!(v, Vect3::new(1.0, 2.0, 3.0));
}

#[test]
fn try_from_i32_slice_ok() {
    let v: Vect3 = (&[4, 5, 6][..]).try_into().unwrap();
    assert_eq!(v, Vect3::new(4.0, 5.0, 6.0));
}

#[test]
fn try_from_slice_err() {
    assert!(Vect3::try_from(&[1.0, 2.0][..]).is_err());
}

// --- Arithmetic traits & checked ops ---
#[test]
fn test_add() {
    let a = Vect3::new(1.0, 2.0, 3.0);
    let b = Vect3::new(4.0, 5.0, 6.0);
    assert_eq!(a + b, Vect3::new(5.0, 7.0, 9.0));
}

#[test]
fn test_sub() {
    let a = Vect3::new(4.0, 5.0, 6.0);
    let b = Vect3::new(1.0, 2.0, 3.0);
    assert_eq!(a - b, Vect3::new(3.0, 3.0, 3.0));
}

#[test]
fn test_mul() {
    let v = Vect3::new(1.0, -2.0, 3.0);
    assert_eq!(v * 2.0, Vect3::new(2.0, -4.0, 6.0));
}

#[test]
fn test_div() {
    let v = Vect3::new(2.0, 4.0, 6.0);
    assert_eq!(v / 2.0, Vect3::new(1.0, 2.0, 3.0));
}

#[test]
fn test_neg() {
    let v = Vect3::new(1.0, -2.0, 3.0);
    assert_eq!(-v, Vect3::new(-1.0, 2.0, -3.0));
}

#[test]
fn test_checked_add_safe() {
    let a = Vect3::new(1.0, 1.0, 1.0);
    let b = Vect3::new(2.0, 2.0, 2.0);
    assert_eq!(a.debug_checked_add(b), Vect3::new(3.0, 3.0, 3.0));
}

#[test]
#[should_panic(expected = "Vect3 overflow in add")]
fn test_checked_add_panic() {
    let m = Vect3::new(f32::MAX, f32::MAX, f32::MAX);
    let _ = m.debug_checked_add(m);
}

// --- Dot, Cross, Length & Normalize ---
#[test]
fn test_dot() {
    assert_eq!(
        Vect3::new(1.0, 0.0, 0.0).dot(&Vect3::new(0.0, 1.0, 0.0)),
        0.0
    );
}

#[test]
fn test_cross() {
    assert_eq!(
        Vect3::new(1.0, 0.0, 0.0).cross(&Vect3::new(0.0, 1.0, 0.0)),
        Vect3::new(0.0, 0.0, 1.0)
    );
}

#[test]
fn test_length() {
    assert!((Vect3::new(1.0, 2.0, 2.0).length() - 3.0).abs() < EPS);
}

#[test]
fn test_normalize() {
    let v = Vect3::new(0.0, 3.0, 4.0).normalize();
    assert!((v.length() - 1.0).abs() < EPS);
}

// --- Distance & Distance Squared ---
#[test]
fn test_distance_sq() {
    assert!(
        (Vect3::new(1.0, 0.0, 0.0).distance_squared(&Vect3::new(0.0, 2.0, 2.0)) - 9.0).abs() < EPS
    );
}

#[test]
fn test_distance() {
    assert!((Vect3::new(1.0, 0.0, 0.0).distance(&Vect3::new(0.0, 2.0, 2.0)) - 3.0).abs() < EPS);
}

// --- Angle Between ---
#[test]
fn test_angle_zero() {
    assert_eq!(
        Vect3::new(1.0, 2.0, 3.0).angle_between(&Vect3::new(1.0, 2.0, 3.0)),
        0.0
    );
}

// --- Lerp, Reflect, Project ---
#[test]
fn test_lerp() {
    let a = Vect3::new(0.0, 0.0, 0.0);
    let b = Vect3::new(2.0, 2.0, 2.0);
    let m = a.lerp(&b, 0.5);
    assert_eq!(m, Vect3::new(1.0, 1.0, 1.0));
}

#[test]
fn test_reflect() {
    let v = Vect3::new(1.0, -1.0, 0.0);
    let n = Vect3::new(0.0, 1.0, 0.0);
    assert_eq!(v.reflect(&n), Vect3::new(1.0, 1.0, 0.0));
}

#[test]
fn test_project() {
    let v = Vect3::new(2.0, 0.0, 0.0);
    let onto = Vect3::new(1.0, 1.0, 0.0);
    let p = v.project(&onto);
    assert!((p.x - 1.0).abs() < EPS && (p.y - 1.0).abs() < EPS);
}

// --- Indexing ---
#[test]
fn test_index() {
    let v = Vect3::new(7.0, 8.0, 9.0);
    assert_eq!(v[0], 7.0);
    assert_eq!(v[1], 8.0);
    assert_eq!(v[2], 9.0);
}

#[test]
#[should_panic]
fn test_index_panic() {
    let v = Vect3::default();
    let _ = v[3];
}

// --- Utility checks ---
#[test]
fn test_is_zero() {
    assert!(Vect3::new(0.0, 0.0, 0.0).is_zero());
}

#[test]
fn test_is_normalized() {
    assert!(Vect3::new(1.0, 0.0, 0.0).is_normalized());
}

#[test]
fn test_is_parallel() {
    let a = Vect3::new(1.0, 1.0, 1.0);
    let b = Vect3::new(2.0, 2.0, 2.0);
    assert!(a.is_parallel(&b));
}
