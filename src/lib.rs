pub fn plot_line(x1_: i32, y1: i32, x2_: i32, y2: i32) -> Vec<(u32, u32)> {
    let x1: i32;
    let x2: i32;
    // let y1: i32;
    // let y2: i32;
    if x2_ > x1_ {
        x1 = x1_;
        x2 = x2_
    } else {
        x1 = x2_;
        x2 = x1_;
    }
    // Bresenham line
    // let dy = y2 - y1;
    // let dx = x2 - x1;
    let m = 2 * (y2 - y1);
    let n = 2 * (x2 - x1);
    let mut slope_err = m - (x2 - x1);
    let mut y = y1;
    let mut points: Vec<(u32, u32)> = Vec::with_capacity((x2 - x1 + 1) as usize);
    for x in x1..=x2 {
        println!("({x}, {y})", x=x, y=y);
        points.push((x as u32, y as u32));
        slope_err += m;
        if slope_err >= 0 {
            y += 1;
            slope_err -= n;
        }
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_plot_line_horiz() {
        let horizontal = vec![(1, 42), (2, 42), (3, 42), (4, 42), (5, 42)];
        assert_eq!(plot_line(1, 42, 5, 42), horizontal);
    }
    #[test]
    fn test_plot_line_vert() {
        let vertical = vec![(42, 1), (42, 2), (42, 3), (42, 4), (42, 5)];
        assert_eq!(plot_line(42, 1, 42, 5), vertical);
    }
    #[test]
    fn test_plot_line_one() {
        let slope_one = vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)];
        assert_eq!(plot_line(1, 1, 5, 5), slope_one);
    }
    #[test]
    fn test_plot_line_neg_one() {
        let slope_neg_one = vec![(1, 5), (2, 4), (3, 3), (4, 2), (5, 1)];
        assert_eq!(plot_line(1, 5, 5, 1), slope_neg_one);
    }
    #[test]
    fn test_plot_line_3_7() {
        let slope_3_7 = vec![(1, 1), (2, 2), (3, 2), (4, 3), (5, 3), (6, 4), (7, 4), (8, 4)];
        assert_eq!(plot_line(1, 1, 8, 4), slope_3_7);
    }
    #[test]
    fn test_plot_line_7_3() {
        let slope_7_3 = vec![(1, 4), (2, 3), (3, 3), (4, 4), (5, 4), (6, 1), (7, 1), (8, 1)];
        assert_eq!(plot_line(1, 4, 8, 1), slope_7_3);
    }
}
