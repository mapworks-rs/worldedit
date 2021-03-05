use quill::{BlockPosition, Entity, Game, Position};
use quill::entities::Player;

use crate::directional::CoordAxis;
use crate::selection::selection::DeltaFace::*;
use crate::util::blockpos;

pub struct Selection {
    player: Player,
    selection: dyn SelectionType
}

pub trait SelectionType {

    fn contains_positions(&self, positions: Vec<Position>) -> bool;

    fn contains_blocks(&self, positions: Vec<BlockPosition>) -> bool {
        self.contains_positions(positions.iter().map(|p| Position::from(*p)).collect())
    }

    fn contains_block(&self, position: BlockPosition) -> bool {
        self.contains_positions(vec![Position::from(position)])
    }

    fn contains_position(&self, position: Position) -> bool {
        self.contains_positions(vec![position])
    }

    fn expand_face(&mut self, delta_face: DeltaFace, amount: i32);

    fn expand_mirrored(&mut self, axis: CoordAxis, amount: i32) {
        match axis {
            CoordAxis::X => {
                self.expand_face(WEST, -amount);
                self.expand_face(EAST, amount);
            },
            CoordAxis::Y => {
                self.expand_face(TOP, amount);
                self.expand_face(BOTTOM, -amount);
            },
            CoordAxis::Z => {
                self.expand_face(NORTH, -amount);
                self.expand_face(SOUTH, amount);
            }
        }
    }

    fn get_blocks(self) -> Vec<BlockPosition>;
}

pub mod cuboid {
    use quill::{BlockPosition, Position};

    use crate::directional::CoordAxis;
    use crate::math::shape::rec;
    use crate::selection::selection::{DeltaFace, SelectionType};
    use crate::selection::selection::DeltaFace::*;

    pub struct CuboidSelection {
        pub(super) min: BlockPosition,
        pub(super) max: BlockPosition
    }

    impl CuboidSelection {

        /// min and max MUST be conserved. See the expand_face function in CuboidSelection for examples
        /// on conserving the min/mas relation
        pub fn new(pos1: BlockPosition, pos2: BlockPosition) -> CuboidSelection {
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

        fn expand_face(&mut self, expand_face: DeltaFace, amount: i32) {
            match expand_face {
                TOP => {
                    self.max.y += amount;
                    if self.max.y < self.min.y {
                        let temp = self.min.y;
                        self.min.y = self.max.y;
                        self.max.y = temp;
                    }
                }
                BOTTOM => {
                    self.min.y -= amount;
                    if self.max.y < self.min.y {
                        let temp = self.min.y;
                        self.min.y = self.max.y;
                        self.max.y = temp;
                    }
                }
                NORTH => {
                    self.min.z -= amount;
                    if self.max.z < self.min.z {
                        let temp = self.min.z;
                        self.min.z = self.max.z;
                        self.max.z = temp;
                    }
                }
                SOUTH => {
                    self.max.z += amount;
                    if self.max.z < self.min.z {
                        let temp = self.min.z;
                        self.min.z = self.max.z;
                        self.max.z = temp;
                    }
                }
                EAST => {
                    self.max.x += amount;
                    if self.max.x < self.min.x {
                        let temp = self.min.x;
                        self.min.x = self.max.x;
                        self.max.x = temp;
                    }

                }
                WEST => {
                    self.min.x -= amount;
                    if self.max.x < self.min.x {
                        let temp = self.min.x;
                        self.min.x = self.max.x;
                        self.max.x = temp;
                    }
                }
            }
        }

        fn get_blocks(self) -> Vec<BlockPosition> {
            rec(self.max.x - self.min.x, self.max.y - self.min.y, self.max.z - self.min.z, true, &self.min)
        }
    }
}

pub mod elliptical {
    use quill::{BlockPosition, Position};

    use crate::directional::CoordAxis;
    use crate::math::shape::ellipse;
    use crate::selection::selection::{DeltaFace, SelectionType};
    use crate::selection::selection::cuboid::CuboidSelection;
    use crate::util::blockpos;

    pub struct EllipticalSelection {
        encapsulating: CuboidSelection,
        height: i32,
        radius_x: u32,
        radius_z: u32
    }

    impl EllipticalSelection {
        pub fn new(pos1: BlockPosition, pos2: BlockPosition) -> EllipticalSelection {
            let cuboid = CuboidSelection::new(pos1, pos2);
            let min = cuboid.min;
            let max = cuboid.max;
            EllipticalSelection {
                encapsulating: cuboid,
                height: max.y - min.y,
                radius_x: ((max.x - min.x) as f64 / 2.0).ceil() as u32,
                radius_z: ((max.z - min.z) as f64 / 2.0).ceil() as u32
            }
        }
    }

    impl SelectionType for EllipticalSelection {
        fn contains_positions(&self, positions: Vec<Position>) -> bool {

            // first check if its even within the cuboid
            if !self.encapsulating.contains_positions(positions.clone()) {
                return false;
            }

            let mut contains = true;
            for pos in positions {
                // x^2 * r_z^2 + z^2 * r_x^2 = r_x^2 * r_z^2
                let term = pos.x * pos.x * (self.radius_z * self.radius_z) as f64 +
                    pos.z * pos.z * (self.radius_x * self.radius_x) as f64;
                if term > (self.radius_z * self.radius_z * self.radius_x * self.radius_x) as f64 {
                    contains = false;
                    break;
                }
            }

            contains
        }

        fn expand_face(&mut self, expand_type: DeltaFace, amount: i32) {
            self.encapsulating.expand_face(expand_type, amount);
            self.height = self.encapsulating.max.y - self.encapsulating.min.y;
            self.radius_x = ((self.encapsulating.max.x - self.encapsulating.min.x) as f64 / 2.0).ceil() as u32;
            self.radius_z = ((self.encapsulating.max.z - self.encapsulating.min.z) as f64 / 2.0).ceil() as u32;
        }

        fn get_blocks(self) -> Vec<BlockPosition> {
            let centre = BlockPosition {
                x: self.encapsulating.min.x + self.radius_x as i32,
                y: self.encapsulating.min.y as i32,
                z: self.encapsulating.min.z + self.radius_z as i32
            };
            ellipse(self.radius_x as f32, self.radius_z as f32, self.height, true, &centre)
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