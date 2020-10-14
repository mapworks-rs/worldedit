use crate::models::block::BlockData;
use quill_prototype::BlockPosition;

pub struct Clipboard {
    // a set of block data *relative to copy origin* pertaining to this clipboard
    pub data: Vec<BlockData>,
    pub origin: BlockPosition,
    //todo player (owner) field
}

pub mod operations {

    pub mod rotate {
        use crate::operation::Operation;
        use crate::selection::selection::Selection;
        use crate::clipboard::clipboard::Clipboard;
        use quill_prototype::BlockPosition;
        use std::borrow::BorrowMut;
        use glam::{Quat, Vec3A};

        pub struct Rotate {
            pub x_deg: f64,
            pub y_deg: f64,
            pub z_deg: f64
        }

        impl Operation for Rotate {
            fn perform(&self, clipboard: Option<&mut Clipboard>, selection: Option<&mut Selection>) {
                if let Some(clip) = clipboard {

                    let quat = Quat::from_rotation_y(self.y_deg.to_radians() as f32)
                        .mul_quat(Quat::from_rotation_x(self.x_deg.to_radians() as f32))
                        .mul_quat(Quat::from_rotation_z(self.z_deg.to_radians() as f32));

                    for block in &mut clip.data {
                        rotate(block.pos.borrow_mut(), quat);
                    }
                }
            }

            fn undo(&self, clipboard: Option<&mut Clipboard>, selection: Option<&mut Selection>) {
                if let Some(clip) = clipboard {

                    let quat = Quat::from_rotation_y(-self.y_deg.to_radians() as f32)
                        .mul_quat(Quat::from_rotation_x(-self.x_deg.to_radians() as f32))
                        .mul_quat(Quat::from_rotation_z(-self.z_deg.to_radians() as f32));

                    for block in &mut clip.data {
                        rotate(block.pos.borrow_mut(), quat);
                    }
                }
            }
        }

        fn rotate_deg(block_pos: &mut BlockPosition, x_deg: f64, y_deg: f64, z_deg: f64) {
            let vec = Quat::from_rotation_y(y_deg.to_radians() as f32)
                .mul_quat(Quat::from_rotation_x(x_deg.to_radians() as f32))
                .mul_quat(Quat::from_rotation_z(z_deg.to_radians() as f32))
                .mul_vec3a(Vec3A::new(block_pos.x as f32, block_pos.y as f32, block_pos.z as f32));
            block_pos.x = vec.x() as i32;
            block_pos.y = vec.y() as i32;
            block_pos.z = vec.z() as i32;
        }

        fn rotate(block_pos: &mut BlockPosition, quaternion: Quat) {
            let vec = quaternion.mul_vec3a(Vec3A::new(block_pos.x as f32, block_pos.y as f32, block_pos.z as f32));
            block_pos.x = vec.x() as i32;
            block_pos.y = vec.y() as i32;
            block_pos.z = vec.z() as i32;
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
