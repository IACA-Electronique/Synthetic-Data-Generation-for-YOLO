/// Normalizes eight corner coordinates from pixel space to [0, 1] relative to image dimensions.
pub fn normalize_four_points(
    x1: u32, y1: u32,
    x2: u32, y2: u32,
    x3: u32, y3: u32,
    x4: u32, y4: u32,
    image_width: u32,
    image_height: u32,
) -> (f32, f32, f32, f32, f32, f32, f32, f32) {
    let w = image_width as f32;
    let h = image_height as f32;
    (
        x1 as f32 / w, y1 as f32 / h,
        x2 as f32 / w, y2 as f32 / h,
        x3 as f32 / w, y3 as f32 / h,
        x4 as f32 / w, y4 as f32 / h,
    )
}

/// Converts a center point and rotation angle to four corner points of a unit square.
///
/// The square is treated as 1×1 centered at (x, y), rotated by `angle` radians.
/// Returns corners in order: top-left, top-right, bottom-right, bottom-left.
pub fn center_and_angle_to_four_points(
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    angle: f32,
) -> (u32, u32, u32, u32, u32, u32, u32, u32) {
    let cx = x as f32;
    let cy = y as f32;
    let half_w = w as f32 / 2.0;
    let half_h = h as f32 / 2.0;

    let cos_a = angle.cos();
    let sin_a = angle.sin();

    let rotate = |dx: f32, dy: f32| -> (u32, u32) {
        let rx = cx + dx * cos_a - dy * sin_a;
        let ry = cy + dx * sin_a + dy * cos_a;
        (rx.round() as u32, ry.round() as u32)
    };

    let (x1, y1) = rotate(-half_w, -half_h);
    let (x2, y2) = rotate(half_w, -half_h);
    let (x3, y3) = rotate(half_w, half_h);
    let (x4, y4) = rotate(-half_w, half_h);

    (x1, y1, x2, y2, x3, y3, x4, y4)
}