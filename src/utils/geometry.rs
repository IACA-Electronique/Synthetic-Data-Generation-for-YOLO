/// Converts a center point and rotation angle to four corner points of a unit square.
///
/// The square is treated as 1×1 centered at (x, y), rotated by `angle` radians.
/// Returns corners in order: top-left, top-right, bottom-right, bottom-left.
pub fn center_and_angle_to_four_points(
    x: u32,
    y: u32,
    angle: f32,
) -> (u32, u32, u32, u32, u32, u32, u32, u32) {
    let cx = x as f32;
    let cy = y as f32;
    let half = 0.5_f32;

    let cos_a = angle.cos();
    let sin_a = angle.sin();

    let rotate = |dx: f32, dy: f32| -> (u32, u32) {
        let rx = cx + dx * cos_a - dy * sin_a;
        let ry = cy + dx * sin_a + dy * cos_a;
        (rx.round() as u32, ry.round() as u32)
    };

    let (x1, y1) = rotate(-half, -half);
    let (x2, y2) = rotate(half, -half);
    let (x3, y3) = rotate(half, half);
    let (x4, y4) = rotate(-half, half);

    (x1, y1, x2, y2, x3, y3, x4, y4)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

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
        let (x1, y1, x2, y2, x3, y3, x4, y4) =
            center_and_angle_to_four_points(100, 100, PI);
        // After 180° rotation corners swap diagonally
        assert_eq!((x1, y1), (101, 101));
        assert_eq!((x2, y2), (100, 101));
        assert_eq!((x3, y3), (100, 100));
        assert_eq!((x4, y4), (101, 100));
    }
}
