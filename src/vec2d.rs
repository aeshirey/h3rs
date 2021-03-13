#[derive(PartialEq, Default, Debug)]
pub struct Vec2d {
    /// x component,
    pub x: f64,
    /// y component
    pub y: f64,
}

impl Vec2d {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /**
     * Calculates the magnitude of a 2D cartesian vector.
     * @param v The 2D cartesian vector.
     * @return The magnitude of the vector.
     */
    pub fn _v2dMag(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /**
     * Finds the intersection between two lines. Assumes that the lines intersect
     * and that the intersection is not at an endpoint of either line.
     * @param p0 The first endpoint of the first line.
     * @param p1 The second endpoint of the first line.
     * @param p2 The first endpoint of the second line.
     * @param p3 The second endpoint of the second line.
     * @param inter The intersection point.
     */
    pub fn _v2dIntersect(p0: &Self, p1: &Self, p2: &Self, p3: &Self) -> Self {
        let s1 = Vec2d::new(p1.x - p0.x, p1.y - p0.y);
        let s2 = Vec2d::new(p3.x - p2.x, p3.y - p2.y);

        let t = (s2.x * (p0.y - p2.y) - s2.y * (p0.x - p2.x)) / (-s2.x * s1.y + s1.x * s2.y);

        Self::new(p0.x + (t * s1.x), p0.y + (t * s1.y))
    }

    /* Whether two 2D vectors are equal. Does not consider possible false
     * negatives due to floating-point errors.
     * @param v1 First vector to compare
     * @param v2 Second vector to compare
     * @return Whether the vectors are equal
    bool _v2dEquals(const Vec2d* v1, const Vec2d* v2) {
        return v1->x == v2->x && v1->y == v2->y;
    }
    */
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v2dMag() {
        let v = Vec2d::new(3.0, 4.0);
        let expected = 5.0;
        let mag = v._v2dMag();
        assert!((mag - expected).abs() < f64::EPSILON, "magnitude as expected");
    }

    #[test]
    fn test_v2dIntersect() {
        let p0 = Vec2d::new(2.0, 2.0);
        let p1 = Vec2d::new(6.0, 6.0);
        let p2 = Vec2d::new(0.0, 4.0);
        let p3 = Vec2d::new(10.0, 4.0);

        let intersection = Vec2d::_v2dIntersect(&p0, &p1, &p2, &p3);

        let expectedX = 4.0;
        let expectedY = 4.0;

        assert!((intersection.x - expectedX).abs() < f64::EPSILON, "X coord as expected");
        assert!((intersection.y - expectedY).abs() < f64::EPSILON, "Y coord as expected");
    }

    #[test]
    fn test_v2dEquals() {
        let v1 = Vec2d::new(3.0, 4.0);
        let v2 = Vec2d::new(3.0, 4.0);
        let v3 = Vec2d::new(3.5, 4.0);
        let v4 = Vec2d::new(3.0, 4.5);

        assert_eq!(v1, v2, "true for equal vectors");
        assert_ne!(v1, v3, "false for different x");
        assert_ne!(v1, v4, "false for different y");
    }
}
