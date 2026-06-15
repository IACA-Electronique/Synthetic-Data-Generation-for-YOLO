use synthetic_data_generator_for_yolo::utils::geometry::normalize_four_points;

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
