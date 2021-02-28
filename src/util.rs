pub const BLOCKPOS_ZERO: BlockPosition = BlockPosition { x: 0, y: 0, z: 0 };

use plotters::prelude::*;
use quill::BlockPosition;
use crate::directional::CoordAxis;

pub fn blockpos(x: i32, y: i32, z: i32) -> BlockPosition {
    BlockPosition { x, y, z }
}

pub fn len_sq2(a: f32, b: f32) -> f32 {
    a * a + b * b
}

pub fn len_sq3(a: f32, b: f32, c: f32) -> f32 {
    a * a + b * b + c * c
}

// temp
pub fn graph3d(mut vecs: Vec<BlockPosition>) {
    let mut reduced: Vec<(f64, f64, f64)> = vecs.iter().map(|bp| (bp.x as f64, bp.y as f64, bp.z as f64)).collect();

    let root = BitMapBackend::new("graphs/graph.png", (600, 600)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("3D Surface", ("sans-serif", 40))
        .build_cartesian_3d(-20.0..20.0, -20.0..20.0, -20.0..20.0)
        .unwrap();

    chart.configure_axes().draw().unwrap();

    chart.draw_series(reduced.iter().map(|point| Circle::new(*point, 3, &RED))).unwrap();
}

// temp
pub fn graph(mut vecs: Vec<BlockPosition>, plane: CoordAxis) {

    let mut reduced: Vec<(i32, i32)> = match plane {
        CoordAxis::X => vecs.iter().map(|bp| (bp.y, bp.z)).collect(),
        CoordAxis::Y => vecs.iter().map(|bp| (bp.x, bp.z)).collect(),
        CoordAxis::Z => vecs.iter().map(|bp| (bp.y, bp.x)).collect()
    };

    let root_area = BitMapBackend::new("graphs/graph.png", (600, 600))
        .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Nice", ("sans-serif", 40))
        .build_cartesian_2d(-30..30, -30..30)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(reduced.iter().map(|point| Circle::new(*point, 3, &RED))).unwrap();
}
