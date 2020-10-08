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
        use crate::math::vector::{Vector3, rotate};

        pub struct Rotate {
            pub x_deg: f64,
            pub y_deg: f64,
            pub z_deg: f64
        }

        impl Operation for Rotate {
            fn perform(&self, clipboard: Option<&mut Clipboard>, selection: Option<&mut Selection>) {
                if let Some(clip) = clipboard {
                    for block in &mut clip.data {
                        rotate(block.pos.borrow_mut(), self.x_deg.to_radians(),
                               self.y_deg.to_radians(), self.z_deg.to_radians())
                    }
                }
            }

            fn undo(&self, clipboard: Option<&mut Clipboard>, selection: Option<&mut Selection>) {
                if let Some(clip) = clipboard {
                    for block in &mut clip.data {
                        rotate(block.pos.borrow_mut(), -self.x_deg.to_radians(),
                               -self.y_deg.to_radians(), -self.z_deg.to_radians())
                    }
                }
            }
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
