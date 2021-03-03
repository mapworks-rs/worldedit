use quill::{BlockPosition, Position};

pub enum CoordAxis {
    X, Y, Z
}

impl CoordAxis {

    pub fn get_pos_component(&self, position: Position) -> f64 {
        match self {
            CoordAxis::X => position.x,
            CoordAxis::Y => position.y,
            CoordAxis::Z => position.z
        }
    }

    pub fn get_block_component(&self, position: BlockPosition) -> i32 {
        self.get_pos_component(Position::from(position)) as i32
    }
}
