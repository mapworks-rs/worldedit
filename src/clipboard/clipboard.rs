use crate::models::block::BlockData;

pub struct Clipboard {
    // a set of block data *relative to copy origin* pertaining to this clipboard
    pub data: Vec<BlockData>
    //todo player (owner) field
}

pub mod operations {

    pub mod rotate {
        use crate::operation::Operation;
        use crate::selection::selection::Selection;
        use crate::clipboard::clipboard::Clipboard;
        use quill_prototype::BlockPosition;
        use std::borrow::BorrowMut;
        use crate::math::quaternion::{quaternion_x, quaternion_y, quaternion_z, Quaternion, quaternion_xyz};
        use crate::math::vector::Vector3;

        pub struct Rotate {
            pub x_deg: f64,
            pub y_deg: f64,
            pub z_deg: f64
        }

        impl Operation for Rotate {
            fn perform(&self, clipboard: Option<&mut Clipboard>, selection: Option<&mut Selection>) {
                if let Some(clip) = clipboard {

                    let quaternion: Quaternion = quaternion_xyz(self.x_deg, self.y_deg, self.z_deg);

                    for block in &mut clip.data {
                        rotate(block.pos.borrow_mut(), &quaternion);
                    }
                }
            }

            fn undo(&self, clipboard: Option<&mut Clipboard>, selection: Option<&mut Selection>) {
                if let Some(clip) = clipboard {

                    let quaternion: Quaternion = quaternion_xyz(-self.x_deg, -self.y_deg, -self.z_deg);

                    for block in &mut clip.data {
                        rotate(block.pos.borrow_mut(), &quaternion);
                    }
                }
            }
        }

        fn rotate(to_rotate: &mut BlockPosition, quaternion: &Quaternion) {
            let rotated = quaternion.rotate(Vector3 {
                x: to_rotate.x as f64,
                y: to_rotate.y as f64,
                z: to_rotate.z as f64
            });

            to_rotate.x = rotated.x.round() as i32;
            to_rotate.y = rotated.y.round() as i32;
            to_rotate.z = rotated.z.round() as i32;
        }
    }

    pub mod flip {
        use crate::models::direction::Plane;
        use crate::clipboard::clipboard::Clipboard;
        use crate::operation::Operation;
        use crate::selection::selection::Selection;

        pub struct Flip {
            pub planes: Vec<Plane>
        }

        impl Operation for Flip {
            fn perform(&self, clipboard: Option<&mut Clipboard>, selection: Option<&mut Selection>) {

                match clipboard {
                    None => { /* todo error handle */ },
                    Some(clip) => {

                        for b in &mut clip.data {
                            for plane in &self.planes {
                                match plane {
                                    //todo block rotation
                                    Plane::X => b.pos.x = -b.pos.x,
                                    Plane::Y => b.pos.y = -b.pos.y,
                                    Plane::Z => b.pos.z = -b.pos.z,
                                }
                            }
                        }
                    },
                }
            }

            fn undo(&self, clipboard: Option<&mut Clipboard>, selection: Option<&mut Selection>) {
                self.perform(clipboard, selection);
            }
        }
    }
}
