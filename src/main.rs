use std::time::Instant;

use rand::Rng;

use crate::math::shape::ellipsoid;
use crate::util::{BLOCKPOS_ZERO, graph3d};
use crate::clipboard::transform::rotate;
use quill::BlockPosition;

pub mod directional;
pub mod selection;
pub mod clipboard;
pub mod math;
pub mod util;

fn main() {
    let mut rng = rand::thread_rng();

    // let mut vecs: Vec<BlockPosition> = ellipse(5.0, 7.0, 1, true, &blockpos(0 ,0, 0));
    //
    // let mut sizes: Vec<(f32, f32, i32)> = Vec::new();
    // for _ in 0..1000 {
    //     sizes.push((rng.gen_range(0, 20) as f32,
    //                 rng.gen_range(0, 20) as f32,
    //                 rng.gen_range(0, 20)))
    // }
    //
    // let now = Instant::now();
    //
    // for t in sizes {
    //     ellipse(t.0, t.1, t.2, true, &BLOCKPOS_ZERO);
    // }
    //
    // println!("generated 1000 filled ellipses in {}ms", now.elapsed().as_millis())

    let mut vecs: Vec<BlockPosition> = ellipsoid(10.0, 15.0, 7.0, false, &BLOCKPOS_ZERO);

    //rotate(&mut vecs, 0.0, 90.0, 0.0);

    graph3d(vecs);
}
