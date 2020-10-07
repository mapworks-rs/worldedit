pub mod quaternion {
    use crate::math::vector::Vector3;

    pub fn quaternion_x(angle_deg: f64) -> Quaternion {
        Quaternion {
            x: (angle_deg.to_radians() / 2.0).sin(),
            y: 0.0,
            z: 0.0,
            w: (angle_deg.to_radians() / 2.0).cos()
        }
    }

    pub fn quaternion_y(angle_deg: f64) -> Quaternion {
        Quaternion {
            x: 0.0,
            y: (angle_deg.to_radians() / 2.0).sin(),
            z: 0.0,
            w: (angle_deg.to_radians() / 2.0).cos()
        }
    }

    pub fn quaternion_z(angle_deg: f64) -> Quaternion {
        Quaternion {
            x: 0.0,
            y: 0.0,
            z: (angle_deg.to_radians() / 2.0).sin(),
            w: (angle_deg.to_radians() / 2.0).cos()
        }
    }

    pub fn quaternion_xyz(x_deg: f64, y_deg: f64, z_deg: f64) -> Quaternion {
        quaternion_x(x_deg).multiply(quaternion_y(y_deg).multiply(quaternion_z(z_deg)))
    }

    pub struct Quaternion {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub w: f64,
    }

    impl Quaternion {
        pub fn multiply(&self, other: Quaternion) -> Quaternion {
            Quaternion {
                x: self.x * other.w + self.y * other.z - self.z * other.y + self.w * other.x,
                y: -self.x * other.z + self.y * other.w + self.z * other.x + self.w * other.y,
                z: self.x * other.y - self.y * other.x + self.z * other.w + self.w * other.z,
                w: -self.x * other.x - self.y * other.y - self.z * other.z + self.w * other.w,
            }
        }

        fn inverse(&self) -> Quaternion {
            let mag_sqrd = (self.w*self.w + self.x*self.x + self.y*self.y + self.z*self.z);

            Quaternion {
                x: -self.x / mag_sqrd,
                y: -self.y / mag_sqrd,
                z: -self.z / mag_sqrd,
                w: self.w / mag_sqrd
            }
        }

        pub fn rotate(&self, vec: Vector3) -> Vector3 {
            Vector3 {
                x: self.w*self.w*vec.x + 2.0*self.y*self.w*vec.z - 2.0*self.z*self.w*vec.y + self.x*self.x*vec.x +
                    2.0*self.y*self.x*vec.y + 2.0*self.z*self.x*vec.z - self.z*self.z*vec.x - self.y*self.y*vec.x,

                y: 2.0*self.x*self.y*vec.x + self.y*self.y*vec.y + 2.0*self.z*self.y*vec.z + 2.0*self.w*self.z*vec.x -
                    self.z*self.z*vec.y + self.w*self.w*vec.y - 2.0*self.x*self.w*vec.z - self.x*self.x*vec.y,

                z: 2.0*self.x*self.z*vec.x + 2.0*self.y*self.z*vec.y + self.z*self.z*vec.z - 2.0*self.w*self.y*vec.x -
                    self.y*self.y*vec.z + 2.0*self.w*self.x*vec.y - self.x*self.x*vec.z + self.w*self.w*vec.z
            }
        }
    }
}

pub mod vector {

    pub struct Vector3 {
        pub x: f64,
        pub y: f64,
        pub z: f64
    }
}
