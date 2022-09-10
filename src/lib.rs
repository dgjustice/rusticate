// CubicW is a 4-tuple of (x, y) weights for use with a Bezier curve
type CubicW = ((u32, u32), (u32, u32), (u32, u32), (u32, u32));

pub fn plot_line(x1_: i32, y1_: i32, x2_: i32, y2_: i32) -> Vec<(u32, u32)> {
    // Bresenham lines
    // Start off by flipping things around to determine whether to increment x or y
    let dy = (y2_ - y1_).abs();
    let dx = (x2_ - x1_).abs();
    let x1: i32;
    let x2: i32;
    let y1: i32;
    let y2: i32;
    let flipper: fn(u32, u32) -> (u32, u32);
    if dy <= dx {
        flipper = |a, b| (a, b);
        if x2_ > x1_ {
            x1 = x1_;
            x2 = x2_;
            y1 = y1_;
            y2 = y2_;
        } else {
            x1 = x2_;
            x2 = x1_;
            y1 = y2_;
            y2 = y1_;
        }
    } else {
        flipper = |a, b| (b, a);
        if y2_ > y1_ {
            x1 = y1_;
            x2 = y2_;
            y1 = x1_;
            y2 = x2_;
        } else {
            x1 = y2_;
            x2 = y1_;
            y1 = x2_;
            y2 = x1_;
        }
    }
    // Whether to increment positive or negative direction
    let incr_y = if y2 > y1 { 1 } else { -1 };
    // Error steps
    let m = 2 * (y2 - y1).abs();
    let n = 2 * (x2 - x1);
    let mut slope_err = m - (x2 - x1);
    let mut y = y1;
    let mut points: Vec<(u32, u32)> = Vec::with_capacity((x2 - x1 + 1) as usize);
    for x in x1..=x2 {
        // println!("{:#?}", flipper(x as u32, y as u32));
        points.push(flipper(x as u32, y as u32));
        slope_err += m;
        if slope_err >= 0 {
            y += incr_y;
            slope_err -= n;
        }
    }
    points
}

fn calc_bezier_cubic_point(t: f32, w: CubicW) -> (u32, u32) {
    // Calculate a point on a Bezier curve given a parametric `t` and weight 4-tuple
    let t2 = t * t;
    let t3 = t2 * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let mt3 = mt2 * mt;
    (
        (w.0.0 as f32 * mt3 + w.1.0 as f32 * mt2 * t * 3.0 + w.2.0 as f32 * mt * t2 * 3.0 + w.3.0 as f32 * t3) as u32,
        (w.0.1 as f32 * mt3 + w.1.1 as f32 * mt2 * t * 3.0 + w.2.1 as f32 * mt * t2 * 3.0 + w.3.1 as f32 * t3) as u32
    )
}

pub fn plot_bezier(w: CubicW) -> Vec<(u32, u32)> {
    // Plot a Bezier curve by iterating lines between control points
    let out = vec![(0, 0)];
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calc_bezier_cubic_point_do_it_run() {
        let w = ((0, 0), (0, 0), (0, 0), (0, 0));
        assert_ne!(plot_bezier(w).len(), 0)
    }
    #[test]
    fn test_plot_line_horiz() {
        let points = vec![(1, 42), (2, 42), (3, 42), (4, 42), (5, 42)];
        assert_eq!(plot_line(1, 42, 5, 42), points);
    }
    #[test]
    fn test_plot_line_vert() {
        let points = vec![(42, 1), (42, 2), (42, 3), (42, 4), (42, 5)];
        assert_eq!(plot_line(42, 1, 42, 5), points);
    }
    #[test]
    fn test_plot_line_neg_vert() {
        let points = vec![(42, 1), (42, 2), (42, 3), (42, 4), (42, 5)];
        assert_eq!(plot_line(42, 5, 42, 1), points);
    }
    #[test]
    fn test_plot_line_one() {
        let points = vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)];
        assert_eq!(plot_line(1, 1, 5, 5), points);
    }
    #[test]
    fn test_plot_line_neg_one() {
        let points = vec![(1, 5), (2, 4), (3, 3), (4, 2), (5, 1)];
        assert_eq!(plot_line(1, 5, 5, 1), points);
    }
    #[test]
    fn test_plot_line_3_7() {
        let points = vec![
            (1, 1),
            (2, 2),
            (3, 2),
            (4, 3),
            (5, 3),
            (6, 4),
            (7, 4),
            (8, 4),
        ];
        assert_eq!(plot_line(1, 1, 8, 4), points);
    }
    #[test]
    fn test_plot_line_neg_3_7() {
        let points = vec![
            (1, 4),
            (2, 3),
            (3, 3),
            (4, 2),
            (5, 2),
            (6, 1),
            (7, 1),
            (8, 1),
        ];
        assert_eq!(plot_line(1, 4, 8, 1), points);
    }
    #[test]
    fn test_plot_line_7_3() {
        let points = vec![
            (1, 1),
            (2, 2),
            (2, 3),
            (3, 4),
            (3, 5),
            (4, 6),
            (4, 7),
            (4, 8),
        ];
        assert_eq!(plot_line(1, 1, 4, 8), points);
    }
    #[test]
    fn test_plot_line_neg_7_3() {
        let points = vec![
            (4, 1),
            (3, 2),
            (3, 3),
            (2, 4),
            (2, 5),
            (1, 6),
            (1, 7),
            (1, 8),
        ];
        assert_eq!(plot_line(4, 1, 1, 8), points);
    }
}
