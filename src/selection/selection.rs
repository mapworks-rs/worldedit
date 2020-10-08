use quill_prototype::BlockPosition;
use crate::models::block::BlockData;

fn selection(pos_one: BlockPosition, pos_two: BlockPosition) -> Selection {
    let upper = BlockPosition {
        x: pos_one.x.max(pos_two.x),
        y: pos_one.y.max(pos_two.y),
        z: pos_one.z.max(pos_two.z)
    };

    let lower = BlockPosition {
        x: pos_one.x.min(pos_two.x),
        y: pos_one.y.min(pos_two.y),
        z: pos_one.z.min(pos_two.z)
    };

    Selection {
        upper,
        lower
    }
}

pub struct Selection {
    upper: BlockPosition,
    lower: BlockPosition,
}

impl Selection {
    fn contains(&self, x: f64, y: f64, z: f64) -> bool {
        x <= self.upper.x as f64 && x >= self.lower.x as f64 &&
            y <= self.upper.y as f64 && y >= self.lower.y as f64 &&
            z <= self.upper.z as f64 && z >= self.lower.z as f64
    }

    fn blocks() -> Vec<BlockData> {
        //todo get all blocks within selection that isn't air
        Vec::new()
    }
}

pub mod operations {

    pub mod set {

    }
}