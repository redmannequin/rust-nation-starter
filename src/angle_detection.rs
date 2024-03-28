use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Vector {
        Vector { x, y }
    }

    fn cross_product(&self, other: &Vector) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

/// Not sure if this is correct. Let's test it 🤠
fn detect_angle(car_vector: Vector, point: Vector) -> f64 {
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

fn get_orth_vector(v: Vector, p: Vector) -> Vector {
    // Compute a vector perpendicular to V
    let v_perpendicular = Vector { x: -v.y, y: v.x }; // A vector perpendicular to V is (-y, x)

    // Compute the vector from P to a point that lies on the line passing through P in the direction of V
    let point_on_line = Vector {
        x: p.x + v.x,
        y: p.y + v.y,
    };

    // Compute the vector that goes from P to the computed point
    let p_to_point_on_line = Vector {
        x: point_on_line.x - p.x,
        y: point_on_line.y - p.y,
    };

    // Compute the dot product of the vectors v_perpendicular and p_to_point_on_line
    // If the dot product is zero, then the vectors are orthogonal
    let dot_product =
        v_perpendicular.x * p_to_point_on_line.x + v_perpendicular.y * p_to_point_on_line.y;

    if dot_product == 0.0 {
        v_perpendicular
    } else {
        panic!("Error: Something went wrong!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let car = Vector::new(0.0, 1.0);
        let res = detect_angle(car, Vector::new(1.0, 0.0));
        assert_eq!(res, 90.0);
    }

    #[test]
    fn test2() {
        let car = Vector::new(1.0, 0.0);
        let res = detect_angle(car, Vector::new(0.0, 0.0));
        assert_eq!(res, 180.0);
    }

    #[test]
    fn test3() {
        let car = Vector::new(-1.0, 0.0);
        let res = detect_angle(car, Vector::new(0.0, 0.0));
        assert_eq!(res, 0.0);
    }

    #[test]
    fn test4() {
        let car = Vector::new(-1.0, -1.0);
        let res = detect_angle(car, Vector::new(0.0, 0.0));
        assert_eq!(res, 45.0);
    }

    #[test]
    fn test5() {
        let car = Vector::new(1.0, 1.0);
        let res = detect_angle(car, Vector::new(0.0, 0.0));
        assert_eq!(res, 180.0 + 45.0);
    }
}
