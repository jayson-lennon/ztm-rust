fn main() {}

#[derive(Debug, Default, PartialEq)]
struct Position {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Circle {
    radius: f64,
    position: Position,
}
impl Circle {
    fn area(&self) -> f64 {
        use std::f64::consts::PI;
        PI * (self.radius * self.radius)
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            radius: 2.0,
            position: Position::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_circle_has_radius_greater_than_zero() {
        let circle = Circle::default();
        assert!(circle.radius > 0.0);
    }

    #[test]
    fn default_circle_uses_default_position() {
        let circle = Circle::default();
        assert_eq!(circle.position, Position::default());
    }

    #[test]
    fn default_position_is_at_origin() {
        // origin is (0,0)
        let position = Position::default();
        assert_eq!(position.x, 0.0);
        assert_eq!(position.y, 0.0);
    }

    #[test]
    fn calculates_area_of_a_circle_when_radius_gt_0() {
        use std::f64::consts::PI;

        // Given a default circle
        let circle = Circle::default();

        // When we calculate the area
        let area = circle.area();

        // Then we get the correct area of a circle
        let expected = PI * (circle.radius * circle.radius);
        assert_eq!(expected, area);
    }
}
