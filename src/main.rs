use quill_prototype::BlockPosition;
use crate::math::shape::line;
use crate::util::blockpos;

pub mod operation;
pub mod models;
pub mod selection;
pub mod clipboard;
pub mod math;
pub mod util;

fn main() {
    let v: Vec<BlockPosition> = line(2, 10, &blockpos(0, 0, 0));

     for bp in v {
         println!("({},{})", bp.x, bp.z);
     }
}
