use quill_prototype::BlockPosition;
use std::f32::consts::PI;
use crate::util::blockpos;

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

pub fn circle(r: i32, filled: bool, origin: &BlockPosition) -> Vec<BlockPosition> {
    ellipse(r, r, filled, origin)
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

pub fn cyl_elip(r_x: i32, r_z: i32, height: i32, filled: bool, origin: &BlockPosition) -> Vec<BlockPosition> {
    let mut vecs: Vec<BlockPosition> = Vec::new();

    for y in 0..(height + 1) {
        vecs.extend(ellipse(r_x, r_z, filled, &blockpos(origin.x, origin.y + y, origin.z)));
    }
    
    vecs
}

pub fn cyl(r: i32, height: i32, filled: bool, origin: &BlockPosition) -> Vec<BlockPosition> {
    cyl_elip(r, r, height, filled, origin)
}

pub fn rec_prism(x: i32, y: i32, z: i32, origin: &BlockPosition) -> Vec<BlockPosition> {
    let mut vecs: Vec<BlockPosition> = Vec::new();

    for a in 0..(x + 1) {
        for b in 0..(z + 1) {
            for c in 0..y {
                vecs.push(BlockPosition {
                    x: (a + origin.x),
                    y: (c + origin.y),
                    z: (b + origin.z)
                });
            }
        }
    }

    vecs
}

pub fn hrec_prism(x: i32, y: i32, z: i32, floor: bool, ceiling: bool, origin: &BlockPosition) -> Vec<BlockPosition> {
    let mut vecs: Vec<BlockPosition> = Vec::new();

    // this function draws the walls
    for y_inc in 0..y {
        // walls iterating over x with z constant
        for i in 0..(x + 1) {
            vecs.push(BlockPosition { x: (i + origin.x), y: (y_inc + origin.y), z: origin.z });
            vecs.push(BlockPosition { x: (i + origin.x), y: (y_inc + origin.y), z: (z + origin.z) });
        }

        // walls iterating over z with x constant
        for i in 0..(z + 1) {
            vecs.push(BlockPosition { x: origin.x, y: (y_inc + origin.y), z: (i + origin.z) });
            vecs.push(BlockPosition { x: (x + origin.x), y: (y_inc + origin.y), z: (i + origin.z) });
        }
    }

    // don't want to run some loops if both are false
    if floor || ceiling {
        // this draws the top and bottom plates (floor and ceiling)
        for a in 0..(z + 1) {
            for b in 0..(x + 1) {
                if floor {
                    vecs.push(BlockPosition { x: (b + origin.x), y: origin.y, z: (a + origin.z) });
                }

                if ceiling {
                    vecs.push(BlockPosition { x: (b + origin.x), y: (y + origin.y - 1), z: (a + origin.z) });
                }
            }
        }
    }

    vecs
}
