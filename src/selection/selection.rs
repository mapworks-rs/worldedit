use quill::{BlockPosition, Position, Game, Entity};
use quill::entities::Player;

use crate::directional::CoordAxis;
use crate::util::blockpos;
use crate::selection::selection::DeltaFace::*;

pub struct Selection<T: SelectionType> {
    player: Player,
    selection: T
}

pub trait SelectionType {

    fn contains_blocks(&self, positions: Vec<BlockPosition>) -> bool {
        self.contains_positions(positions.iter().map(|p| Position::from(*p)).collect())
    }

    fn contains_block(&self, position: BlockPosition) -> bool {
        self.contains_positions(vec![Position::from(position)])
    }

    fn contains_position(&self, position: Position) -> bool {
        self.contains_positions(vec![position])
    }

    fn contains_positions(&self, positions: Vec<Position>) -> bool;

    fn delta_face(&mut self, expand_type: DeltaFace, amount: i32);

    fn delta_mirror(&mut self, axis: CoordAxis, amount: i32);
}

pub mod cuboid {
    use quill::{BlockPosition, Position};

    use crate::directional::CoordAxis;
    use crate::selection::selection::{DeltaFace, SelectionType};

    struct CuboidSelection {
        min: BlockPosition,
        max: BlockPosition
    }

    impl CuboidSelection {

        fn new(pos1: BlockPosition, pos2: BlockPosition) -> CuboidSelection {
            CuboidSelection {
                min: BlockPosition {
                    x: pos1.x.min(pos2.x),
                    y: pos1.y.min(pos2.y),
                    z: pos1.z.min(pos2.z)
                },
                max: BlockPosition {
                    x: pos1.x.max(pos2.x),
                    y: pos1.y.max(pos2.y),
                    z: pos1.z.max(pos2.z)
                }
            }
        }
    }

    impl SelectionType for CuboidSelection {

        fn contains_positions(&self, positions: Vec<Position>) -> bool {

            let mut contains = true;
            for pos in positions {
                if pos.x < self.min.x as f64 || pos.x > self.max.x as f64 ||
                    pos.y < self.min.y as f64 || pos.y > self.max.y as f64 ||
                    pos.z < self.min.z as f64 || pos.z > self.max.z as f64 {
                    contains = false;
                    break;
                }
            }

            contains
        }

        fn delta_face(&mut self, delta_face: DeltaFace, amount: i32) {

        }

        fn delta_mirror(&mut self, axis: CoordAxis, amount: i32) {
            unimplemented!()
        }
    }
}

pub enum DeltaFace {
    TOP, BOTTOM, NORTH, SOUTH, EAST, WEST
}

pub enum RelativeFace {
    UP, DOWN, LEFT, RIGHT, FRONT, BACK
}

impl RelativeFace {
    /// Returns the cardinal face given a relative face as well as a viewpoint's position (player)
    ///
    /// This does not return anything special for up/down y values. Regardless of the direction
    /// the player/viewpoint is facing, only the x and z component will be compared for finding the
    /// proper direction.
    ///
    /// Example: If I have a player looking South and I they now asked for the direction to the left
    /// of them. This function determines that they are looking South (positive z) and gives the
    /// corresponding direction to their left which would be East in this case.
    fn delta_face(&self, player: Entity) -> DeltaFace {

        if matches!(self, RelativeFace::UP) {
            return TOP;
        }

        if matches!(self, RelativeFace::DOWN) {
            return BOTTOM;
        }

        let vec = player.get::<Position>().unwrap().direction();
        let x_abs = vec.x.abs();
        let z_abs = vec.z.abs();

        return if x_abs > z_abs {
            match self {
                RelativeFace::UP => TOP,
                RelativeFace::DOWN => BOTTOM,
                RelativeFace::LEFT => if vec.x > 0.0 { NORTH } else { SOUTH }
                RelativeFace::RIGHT => if vec.x > 0.0 { SOUTH } else { NORTH }
                RelativeFace::FRONT => if vec.x > 0.0 { EAST } else { WEST }
                RelativeFace::BACK => if vec.x > 0.0 { WEST } else { EAST }
            }
        } else {
            match self {
                RelativeFace::UP => TOP,
                RelativeFace::DOWN => BOTTOM,
                RelativeFace::LEFT => if vec.z > 0.0 { EAST } else { WEST }
                RelativeFace::RIGHT => if vec.z > 0.0 { WEST } else { EAST }
                RelativeFace::FRONT => if vec.z > 0.0 { SOUTH } else { NORTH }
                RelativeFace::BACK => if vec.z > 0.0 { NORTH } else { SOUTH }
            }
        }
    }
}