use std::f32::consts::PI;
use synthetic_data_generator_for_yolo::utils::geometry::center_and_angle_to_four_points;

#[test]
fn no_rotation_returns_corners_around_center() {
    let (x1, y1, x2, y2, x3, y3, x4, y4) = center_and_angle_to_four_points(10, 10, 0.0);
    assert_eq!((x1, y1), (10, 10)); // top-left rounds to center for unit square
    assert_eq!((x2, y2), (11, 10));
    assert_eq!((x3, y3), (11, 11));
    assert_eq!((x4, y4), (10, 11));
}

#[test]
fn half_turn_is_symmetric() {
    let (x1, y1, x2, y2, x3, y3, x4, y4) = center_and_angle_to_four_points(100, 100, PI);
    // After 180° rotation corners swap diagonally
    assert_eq!((x1, y1), (101, 101));
    assert_eq!((x2, y2), (100, 101));
    assert_eq!((x3, y3), (100, 100));
    assert_eq!((x4, y4), (101, 100));
}
