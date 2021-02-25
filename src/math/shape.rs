use quill_prototype::BlockPosition;
use crate::util::{blockpos, len_sq2, len_sq3};

pub fn rec(x: i32, y: i32, z: i32, filled: bool, origin: &BlockPosition) -> Vec<BlockPosition> {
    let mut vecs: Vec<BlockPosition> = Vec::new();

    if x == 0 || z == 0 || y == 0 {
        return vecs;
    }

    if !filled {
        for y_diff in 0..y {
            for x_diff in 0..x {
                vecs.push(blockpos(origin.x + x_diff, origin.y + y_diff, origin.z));
                vecs.push(blockpos(origin.x + x_diff, origin.y + y_diff, origin.z + z - 1));
            }

            for z_diff in 0..z {
                vecs.push(blockpos(origin.x, origin.y + y_diff, origin.z + z_diff));
                vecs.push(blockpos(origin.x + x - 1, origin.y + y_diff, origin.z + z_diff));
            }
        }
    } else {
        for y_diff in 0..y {
            for x_diff in 0..x {
                for z_diff in 0..z {
                    vecs.push(blockpos(origin.x + x_diff, origin.y + y_diff, origin.z + z_diff));
                }
            }
        }
    }

    vecs
}

pub fn ellipse(mut r_x: f32, mut r_z: f32, height: i32, filled: bool, origin: &BlockPosition) -> Vec<BlockPosition> {
    let mut vecs: Vec<BlockPosition> = Vec::new();

    r_x += 0.5;
    r_z += 0.5;

    let inv_x: f32 = 1.0 / r_x;
    let inv_z: f32 = 1.0 / r_z;

    for x in 0..r_x.ceil() as i32 {
        let next_x_val = (x as f32 + 1.0) * inv_x;
        for z in 0..r_z.ceil() as i32 {
            let next_z_val = (z as f32 + 1.0) * inv_z;

            let len_sq_val: f32 = len_sq2(x as f32 * inv_x, z as f32 * inv_z);

            if len_sq_val > 1.0 || (
                !filled &&
                len_sq2(next_x_val, z as f32 * inv_z) <= 1.0 &&
                len_sq2(x as f32 * inv_x, next_z_val) <= 1.0) {
                continue;
            }

            for y in 0..height {
                vecs.push(blockpos(origin.x + x, origin.y + y, origin.z + z));
                vecs.push(blockpos(origin.x - x, origin.y + y, origin.z + z));
                vecs.push(blockpos(origin.x + x, origin.y + y, origin.z - z));
                vecs.push(blockpos(origin.x - x, origin.y + y, origin.z - z));
            }
        }
    }

    vecs
}

pub fn ellipsoid(mut r_x: f32, mut r_y: f32, mut r_z: f32, filled: bool, origin: &BlockPosition) -> Vec<BlockPosition> {
    let mut vecs: Vec<BlockPosition> = Vec::new();

    r_x += 0.5;
    r_z += 0.5;
    r_y += 0.5;

    let inv_x: f32 = 1.0 / r_x;
    let inv_z: f32 = 1.0 / r_z;
    let inv_y: f32 = 1.0 / r_y;

    for x in 0..r_x.ceil() as i32 {
        let next_x_val = (x as f32 + 1.0) * inv_x;
        for z in 0..r_z.ceil() as i32 {
            let next_z_val = (z as f32 + 1.0) * inv_z;
            for y in 0..r_y.ceil() as i32 {
                let next_y_val = (y as f32 + 1.0) * inv_y;

                let len_sq_val: f32 = len_sq3(x as f32 * inv_x, z as f32 * inv_z, y as f32 * inv_y);

                if len_sq_val > 1.0 || (
                    !filled &&
                        len_sq3(next_x_val, z as f32 * inv_z, y as f32 * inv_y) <= 1.0 &&
                        len_sq3(x as f32 * inv_x, next_z_val, y as f32 * inv_y) <= 1.0 &&
                        len_sq3(x as f32 * inv_x, z as f32 * inv_z, next_y_val) <= 1.0) {
                    continue;
                }

                vecs.push(blockpos(origin.x + x, origin.y + y, origin.z + z));
                vecs.push(blockpos(origin.x - x, origin.y + y, origin.z + z));
                vecs.push(blockpos(origin.x + x, origin.y + y, origin.z - z));
                vecs.push(blockpos(origin.x - x, origin.y + y, origin.z - z));
                vecs.push(blockpos(origin.x + x, origin.y - y, origin.z + z));
                vecs.push(blockpos(origin.x - x, origin.y - y, origin.z + z));
                vecs.push(blockpos(origin.x + x, origin.y - y, origin.z - z));
                vecs.push(blockpos(origin.x - x, origin.y - y, origin.z - z));
            }
        }
    }

    vecs
}
