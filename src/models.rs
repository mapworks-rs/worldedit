pub mod block {
    use quill_prototype::BlockPosition;

    pub struct BlockData {
        pub pos: BlockPosition,
        // block data
    }
}

pub mod direction {
    pub enum Plane {
        X, Y, Z
    }
}