use crate::library::geometry::point::{partial_cmp, Orientation, Point, PointType};

pub fn convex_hull<T: PointType>(mut poly: Vec<Point<T>>) -> Vec<Point<T>> {
    if poly.len() <= 1 {
        return poly;
    }

    poly.sort_by(partial_cmp);

    let mut h = vec![Point::default(); poly.len() + 1];
    let mut len = 0;
    let mut ed = 0;
    for _ in 0..2 {
        for p in &poly {
            while len >= ed + 2
                && matches!(
                    h[len - 2].orientation(&h[len - 1], p),
                    Orientation::CounterClockwise | Orientation::Collinear
                )
            {
                len -= 1;
            }
            h[len] = *p;
            len += 1;
        }
        len -= 1;
        ed = len;
        poly.reverse();
    }

    h[..len - (len == 2 && h[0] == h[1]) as usize].to_vec()
}

#[cfg(test)]
mod tests {
    use super::convex_hull;
    use crate::library::geometry::point::Point;

    #[test]
    fn test_convex_hull() {
        let points = vec![
            Point::new(0, 0),
            Point::new(3, 3),
            Point::new(0, 3),
            Point::new(3, 0),
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(1, 2),
            Point::new(2, 2),
        ];
        let hull = convex_hull(points);
        assert_eq!(hull.len(), 4);
        assert_eq!(hull[0], Point::new(0, 0));
        assert_eq!(hull[1], Point::new(3, 0));
        assert_eq!(hull[2], Point::new(3, 3));
        assert_eq!(hull[3], Point::new(0, 3));
    }
}
