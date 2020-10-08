// hella deprecated
pub mod vector {
    use quill_prototype::BlockPosition;

    pub struct Vector3 {
        pub x: f64,
        pub y: f64,
        pub z: f64
    }
    
    impl Vector3 {
        pub fn rotate(&mut self, x_rad: f64, y_rad: f64, z_rad: f64) {

            // != 0.0 checks to save some computation time

            if y_rad != 0.0 {
                let new_x = self.x*y_rad.cos() - self.z*y_rad.sin();
                self.z = self.x*y_rad.sin() + self.z*y_rad.cos();
                self.x = new_x;
            }

            if x_rad != 0.0 {
                let new_z = self.z*x_rad.cos() - self.y*x_rad.sin();
                self.y = self.z*x_rad.sin() + self.y*x_rad.cos();
                self.z = new_z;
            }

            if z_rad != 0.0 {
                let new_x = self.x*z_rad.cos() + self.y*z_rad.sin();
                self.y = -self.x*z_rad.sin() + self.y*z_rad.cos();
                self.x = new_x;
            }
        }

        pub fn rotate_deg(&mut self, x_deg: f64, y_deg: f64, z_deg: f64) {
            self.rotate(x_deg.to_radians(), y_deg.to_radians(), z_deg.to_radians());
        }
    }

    pub fn rotate(block_pos: &mut BlockPosition, x_rad: f64, y_rad: f64, z_rad: f64) {
        let mut vec =  Vector3 {
            x: block_pos.x as f64,
            y: block_pos.y as f64,
            z: block_pos.z as f64
        };
        vec.rotate(x_rad, y_rad, z_rad);

        block_pos.x = vec.x.round() as i32;
        block_pos.y = vec.y.round() as i32;
        block_pos.z = vec.z.round() as i32;
    }
}
