use std::f64::consts::PI;

struct Point2 {
    x: i32,
    y: i32,
}

impl Point2 {
    pub fn new(x: i32, y: i32) -> Point2 {
        Point2 { x, y }
    }
}

/// Not sure if this is correct. Let's test it 🤠
fn detect_angle(car_vector: Point2, point: Point2) -> f64 {
    let dx = point.x - car_vector.x;
    let dy = point.y - car_vector.y;
    let first = libm::atan2(dy.into(), dx.into()) * 180.0;
    let angle = first / PI;
    if angle < 0.0 {
        angle + 360.0
    } else {
        angle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let car = Point2::new(0, 1);
        let res = detect_angle(car, Point2::new(1, 0));
        assert_eq!(res, 90.0);
    }

    #[test]
    fn test2() {
        let car = Point2::new(1, 0);
        let res = detect_angle(car, Point2::new(0, 0));
        assert_eq!(res, 180.0);
    }

    #[test]
    fn test3() {
        let car = Point2::new(-1, 0);
        let res = detect_angle(car, Point2::new(0, 0));
        assert_eq!(res, 0.0);
    }

    #[test]
    fn test4() {
        let car = Point2::new(-1, -1);
        let res = detect_angle(car, Point2::new(0, 0));
        assert_eq!(res, 45.0);
    }

    #[test]
    fn test5() {
        let car = Point2::new(1, 1);
        let res = detect_angle(car, Point2::new(0, 0));
        assert_eq!(res, 180.0 + 45.0);
    }
}
