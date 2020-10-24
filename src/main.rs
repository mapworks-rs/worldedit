use crate::math::vector::Vector3;
use rand::prelude::*;
use std::time::{Duration, Instant};
use glam::{Vec3A, Quat};

pub mod operation;
pub mod models;
pub mod selection;
pub mod clipboard;

fn main() {

    // let mut rng = thread_rng();
    // let mut vecs: Vec<Vec3A> = Vec::new();
    //
    // let mut q = Quat::from_rotation_x(102.0_f32.to_radians())
    //     .mul_quat(Quat::from_rotation_y(-193.45_f32.to_radians()))
    //     .mul_quat(Quat::from_rotation_z(32.0_f32.to_radians()));
    //
    // let mut vecs: Vec<Vec3A> = Vec::new();
    //
    // for _ in 0..1000000 {
    //     let mut v = Vec3A::new(rng.gen_range(0.0, 10.0),
    //                            rng.gen_range(0.0, 10.0),
    //                            rng.gen_range(0.0, 10.0));
    //     vecs.push(v);
    // }
    //
    // let p = vecs.len();
    //
    // let dur = Instant::now();
    // for vec in vecs {
    //     q.mul_vec3a(vec);
    // }
    // println!("rotated {} vectors in {:?}", p, dur.elapsed());

    let mut q = Quat::from_rotation_y(90.0_f32.to_radians());

    let v = q.mul_vec3a(Vec3A::new(1.0, 1.0, 0.0));

    println!("x={} y={} z={}", v.x(), v.y(), v.z());
}
