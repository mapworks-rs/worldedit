use quill_prototype::BlockPosition;

pub fn blockpos(x: i32, y: i32, z: i32) -> BlockPosition {
    BlockPosition { x, y, z }
}