use crate::util::blockpos;
use quill::{BlockPosition, Position};

pub struct Selection {
    pos1: BlockPosition,
    pos2: BlockPosition
}

impl Selection {

    pub fn contains_blocks(&self, positions: Vec<BlockPosition>) -> bool {
        self.contains_positions(positions.iter().map(|p| Position::from(*p)).collect())
    }

    pub fn contains_block(&self, position: BlockPosition) -> bool {
        self.contains_positions(vec![Position::from(position)])
    }

    pub fn contains_position(&self, position: Position) -> bool {
        self.contains_positions(vec![position])
    }

    pub fn contains_positions(&self, positions: Vec<Position>) -> bool {
        let minx = self.pos1.x.min(self.pos2.x) as f64;
        let maxx = self.pos1.x.max(self.pos2.x) as f64;
        let miny = self.pos1.y.min(self.pos2.y) as f64;
        let maxy = self.pos1.y.max(self.pos2.y) as f64;
        let minz = self.pos1.z.min(self.pos2.z) as f64;
        let maxz = self.pos1.z.max(self.pos2.z) as f64;

        let mut contains = true;
        for pos in positions {
            if pos.x < minx || pos.x > maxx ||
                pos.y < miny || pos.y > maxy ||
                pos.z < minz || pos.z > maxz {
                contains = false;
                break;
            }
        }

        contains
    }
}

pub enum ExpandType {
    UP, DOWN, LEFT, RIGHT, FRONT, BACK
}