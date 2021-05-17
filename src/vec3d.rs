pub struct Vec3d {
    /// x component
    pub x: f64,
    /// y component
    pub y: f64,
    /// z component
    pub z: f64,
}

/// Square of a number
fn _square(x: f64) -> f64 {
    x * x
}


impl Vec3d {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3d { x, y, z }
    }

    /// Calculate the square of the distance between two 3D coordinates.
    pub fn _pointSquareDist(&self, other: &Self) -> f64 {
        _square(self.x - other.x) + _square(self.y - other.y) + _square(self.z - other.z)
    }
}

/*
/// Calculate the 3D coordinate on unit sphere from the latitude and longitude.
///
///@param geo The latitude and longitude of the point.
///@param v The 3D coordinate of the point.
///
void _geoToVec3d(const GeoCoord* geo, Vec3d* v) {
    double r = cos(geo->lat);

    v->z = sin(geo->lat);
    v->x = cos(geo->lon) * r;
    v->y = sin(geo->lon) * r;
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pointSquareDist() {
        let v1 = Vec3d::new(0., 0., 0.);
        let v2 = Vec3d::new(1., 0., 0.);
        let v3 = Vec3d::new(0., 1., 1.);
        let v4 = Vec3d::new(1., 1., 1.);
        let v5 = Vec3d::new(1., 1., 2.);

        assert!(
            (v1._pointSquareDist(&v1)).abs() < f64::EPSILON,
            "distance to self is 0"
        );
        assert!(
            (v1._pointSquareDist(&v2) - 1.).abs() < f64::EPSILON,
            "distance to <1,0,0> is 1"
        );
        assert!(
            (v1._pointSquareDist(&v3) - 2.).abs() < f64::EPSILON,
            "distance to <0,1,1> is 2"
        );
        assert!(
            (v1._pointSquareDist(&v4) - 3.).abs() < f64::EPSILON,
            "distance to <1,1,1> is 3"
        );
        assert!(
            (v1._pointSquareDist(&v5) - 6.).abs() < f64::EPSILON,
            "distance to <1,1,2> is 6"
        );
    }

    /*
    #[test]
    fn test_geoToVec3d() {
        Vec3d origin = {0};

        GeoCoord c1 = {0, 0};
        Vec3d p1;
        _geoToVec3d(&c1, &p1);
        assert!(fabs(_pointSquareDist(&origin, &p1) - 1) < EPSILON_RAD, "Geo point is on the unit sphere");

        GeoCoord c2 = {M_PI_2, 0};
        Vec3d p2;
        _geoToVec3d(&c2, &p2);
        assert!(fabs(_pointSquareDist(&p1, &p2) - 2) < EPSILON_RAD, "Geo point is on another axis");

        GeoCoord c3 = {M_PI, 0};
        Vec3d p3;
        _geoToVec3d(&c3, &p3);
        assert!(fabs(_pointSquareDist(&p1, &p3) - 4) < EPSILON_RAD, "Geo point is the other side of the sphere");
    }
    */
}
