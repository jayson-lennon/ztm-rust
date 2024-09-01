fn main() {}

#[derive(Debug, Default, PartialEq, PartialOrd)]
struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Circle {
    pub center: Position,
    pub radius: f64,
}

impl Circle {
    pub fn area(&self) -> f64 {
        use std::f64::consts::PI;
        PI * (self.radius * self.radius)
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            center: Position::default(),
            radius: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_circle_is_at_origin() {
        let circle = Circle::default();
        assert_eq!(circle.center, Position::default());
    }

    #[test]
    fn default_circle_has_radius_gt_zero() {
        let circle = Circle::default();
        assert!(circle.radius > 0.0);
    }

    #[test]
    fn default_position_is_at_origin() {
        let position = Position::default();
        assert_eq!(position.x, 0.0);
        assert_eq!(position.y, 0.0);
    }

    #[test]
    fn calculates_area_of_a_circle() {
        use std::f64::consts::PI;

        // Given a default circle
        let circle = Circle::default();

        // When we calculate the area
        let area = circle.area();

        // Then we get the correct area of the circle
        //
        // This equation produces a known good result. So if the implementation changes later (like
        // if it's updated as an optimization or something), then we can confirm that our result is
        // the same.
        let expected = PI * (circle.radius * circle.radius);
        assert_eq!(expected, area);
    }
}
