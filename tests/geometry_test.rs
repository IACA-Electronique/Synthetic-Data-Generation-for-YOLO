use std::f32::consts::PI;
use synthetic_data_generator_for_yolo::utils::geometry::{
    center_and_angle_to_four_points, normalize_four_points,
};

#[test]
fn no_rotation_returns_corners_around_center() {
    let (x1, y1, x2, y2, x3, y3, x4, y4) = center_and_angle_to_four_points(10, 10, 0.0);
    assert_eq!((x1, y1), (10, 10)); // top-left rounds to center for unit square
    assert_eq!((x2, y2), (11, 10));
    assert_eq!((x3, y3), (11, 11));
    assert_eq!((x4, y4), (10, 11));
}

#[test]
fn normalize_origin_point_is_zero() {
    let (x1, y1, x2, y2, x3, y3, x4, y4) =
        normalize_four_points(0, 0, 0, 0, 0, 0, 0, 0, 100, 200);
    assert_eq!((x1, y1), (0.0, 0.0));
    assert_eq!((x2, y2), (0.0, 0.0));
    assert_eq!((x3, y3), (0.0, 0.0));
    assert_eq!((x4, y4), (0.0, 0.0));
}

#[test]
fn normalize_full_image_corners_are_one() {
    let (x1, y1, x2, y2, x3, y3, x4, y4) =
        normalize_four_points(100, 200, 100, 200, 100, 200, 100, 200, 100, 200);
    assert_eq!((x1, y1), (1.0, 1.0));
    assert_eq!((x2, y2), (1.0, 1.0));
    assert_eq!((x3, y3), (1.0, 1.0));
    assert_eq!((x4, y4), (1.0, 1.0));
}

#[test]
fn normalize_midpoint_is_half() {
    let (x1, y1, x2, y2, x3, y3, x4, y4) =
        normalize_four_points(50, 100, 50, 100, 50, 100, 50, 100, 100, 200);
    assert_eq!((x1, y1), (0.5, 0.5));
    assert_eq!((x2, y2), (0.5, 0.5));
    assert_eq!((x3, y3), (0.5, 0.5));
    assert_eq!((x4, y4), (0.5, 0.5));
}

#[test]
fn normalize_distinct_corners_are_scaled_independently() {
    let (x1, y1, x2, y2, x3, y3, x4, y4) =
        normalize_four_points(0, 0, 100, 0, 100, 200, 0, 200, 100, 200);
    assert_eq!((x1, y1), (0.0, 0.0));
    assert_eq!((x2, y2), (1.0, 0.0));
    assert_eq!((x3, y3), (1.0, 1.0));
    assert_eq!((x4, y4), (0.0, 1.0));
}

#[test]
fn normalize_non_square_image_scales_axes_independently() {
    let (x1, y1, x2, y2, x3, y3, x4, y4) =
        normalize_four_points(50, 50, 50, 50, 50, 50, 50, 50, 100, 200);
    assert_eq!((x1, y1), (0.5, 0.25));
    assert_eq!((x2, y2), (0.5, 0.25));
    assert_eq!((x3, y3), (0.5, 0.25));
    assert_eq!((x4, y4), (0.5, 0.25));
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
