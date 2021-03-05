use glam::{Quat, Vec3A};
use quill::BlockPosition;

/// Rotates a vector of `BlockPosition`s around (0, 0, 0)
///
/// The order of rotations is about the y, x, z axis in that order
pub fn rotate(block_vecs: &mut Vec<BlockPosition>, x_deg: f64, y_deg: f64, z_deg: f64) {
    let quat = Quat::from_rotation_y(y_deg.to_radians() as f32)
        .mul_quat(Quat::from_rotation_x(x_deg.to_radians() as f32))
        .mul_quat(Quat::from_rotation_z(z_deg.to_radians() as f32));
    for bp in block_vecs {
        let vec = quat.mul_vec3a(Vec3A::new(bp.x as f32, bp.y as f32, bp.z as f32));
        bp.x = vec.x as i32;
        bp.y = vec.y as i32;
        bp.z = vec.z as i32;
    }
}