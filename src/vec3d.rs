pub struct Vec3d {
    /// x component
    x:f64, 
    /// y component
    y:f64, 
    /// z component
    z:f64, 
}


/// Square of a number
fn _square(x:f64) { x * x }

impl Vec3d {
    /// Calculate the square of the distance between two 3D coordinates.
    pub fn _pointSquareDist(&self, other: &Self) -> f64 {
        _square(self.x - other.x) + _square(self.y - other.y) + _square(self.z - other.z)
    }
}
