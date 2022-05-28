use crate::vec::Vec3;

#[test]
fn cross() {
    let a = Vec3::new(2, 3, 4);
    let b = Vec3::new(5, 6, 7);

    assert_eq!(a.cross(b), Vec3::new(-3, 6, -3));
}

#[test]
fn dot() {
    let a = Vec3::new(9, 2, 7);
    let b = Vec3::new(4, 8, 10);

    assert_eq!(a.dot(b), 122);
}

#[test]
fn sq_length() {
    let a = Vec3::new(9, 2, 7);

    assert_eq!(a.sq_length(), 134);
}

#[test]
fn length() {
    let a = Vec3::new(9.0, 2.0, 7.0);
    let result = a.length();
    let expected = (134.0 as f64).sqrt();

    assert_eq!((result * 1000.) as i32, (expected * 1000.) as i32);
}

#[test]
fn angle() {
    let a = Vec3::new(9., 2., 7.);
    let b = Vec3::new(4., 8., 10.);
    let expected: f64 = 0.6672196;

    assert_eq!((a.angle(b) * 1000.) as i32, (expected * 1000.) as i32);
}
