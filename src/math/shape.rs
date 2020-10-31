use quill_prototype::BlockPosition;
use crate::util::blockpos;

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

pub fn ellipse(r_x: i32, r_z: i32, filled: bool, origin: &BlockPosition) -> Vec<BlockPosition> {
    let mut vecs: Vec<BlockPosition> = Vec::new();

    if r_x == 0 || r_z == 0 {
        return vecs;
    }

    let radii_bound = r_z * r_z * r_x * r_x;

    for x in 0..(r_x + 1) as i32 {
        for z in 0..(r_z + 1) as i32 {
            let p = x * x * r_z * r_z + z * z * r_x * r_x;
            if p > radii_bound || (!filled && p < radii_bound) { continue; }

            vecs.push(blockpos(origin.x + x, origin.y, origin.z + z));
            vecs.push(blockpos(origin.x - x, origin.y, origin.z + z));
            vecs.push(blockpos(origin.x + x, origin.y, origin.z - z));
            vecs.push(blockpos(origin.x - x, origin.y, origin.z - z));
        }
    }

    vecs
}

pub fn circle(r: u32, filled: bool, origin: &BlockPosition) -> Vec<BlockPosition> {
    ellipse(r as i32, r as i32, filled, origin)
}

pub fn ellipsoid(r_x: i32, r_y: i32, r_z: i32, filled: bool, origin: &BlockPosition) -> Vec<BlockPosition> {
    let mut vecs: Vec<BlockPosition> = Vec::new();

    if r_x == 0 || r_z == 0 || r_y == 0 {
        return vecs;
    }

    let radii_bound = r_x * r_x * r_y * r_y * r_z * r_z;

    for x in 0..(r_x + 1) {
        for y in 0..(r_y + 1) {
            for z in 0..(r_z + 1) {
                let p = x*x * r_z*r_z * r_y*r_y + y*y * r_x*r_x * r_z*r_z + z*z + r_y*r_y + r_x*r_x;
                if p > radii_bound || (!filled && p < radii_bound) { continue; }

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

pub fn sphere(r: i32, filled: bool, origin: &BlockPosition) -> Vec<BlockPosition> {
    ellipsoid(r, r, r, filled, origin)
}

pub fn cyl_ellip(r_x: i32, r_z: i32, height: i32, filled: bool, origin: &BlockPosition) -> Vec<BlockPosition> {
    let mut vecs: Vec<BlockPosition> = Vec::new();

    for y in 0..(height + 1) {
        vecs.extend(ellipse(r_x, r_z, filled, &blockpos(origin.x, origin.y + y, origin.z)));
    }
    
    vecs
}

pub fn cylinder(r: i32, height: i32, filled: bool, origin: &BlockPosition) -> Vec<BlockPosition> {
    cyl_ellip(r, r, height, filled, origin)
}

pub fn pyramid(height: u32, filled: bool, origin: &BlockPosition) -> Vec<BlockPosition> {
    let mut vecs: Vec<BlockPosition> = Vec::new();

    for i in 0..height {
        vecs.extend(rec((2 * height - i) as i32, 1, (2 * height - i) as i32, filled,
            &blockpos(origin.x, origin.y + i as i32, origin.z)
        ));
    }

    vecs
}

pub fn cone(height: u32, filled: bool, origin: &BlockPosition) -> Vec<BlockPosition> {
    let mut vecs: Vec<BlockPosition> = Vec::new();

    for i in 0..height {
        vecs.extend(circle((height - i), filled,
                           &blockpos(origin.x, origin.y + i as i32, origin.z)));
    }

    vecs
}
