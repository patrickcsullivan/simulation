mod resource;
mod system;

use crate::collections::grid::RowMajorGrid;

// 1 cell is 4 m wide

const GRID_WIDTH: usize = 8; // 8 cells wide
const GRID_HEIGHT: usize = 8; // 8 cells high

#[derive(Debug, Default, Clone)]
pub struct SharedCell {
    pub density: f32,
    pub height: f32, // // Only set to zero for now.
    pub discomfort: f32,
    pub avg_velocity: (f32, f32),

    pub east_face: SharedCellFace,
    pub north_face: SharedCellFace,
    pub west_face: SharedCellFace,
    pub south_face: SharedCellFace,
}

#[derive(Debug, Default, Clone)]
pub struct SharedCellFace {
    pub height_gradient: f32, // Only set to zero for now.
}

#[derive(Debug, Default, Clone)]
pub struct GroupCell {
    pub potential: f32,

    pub east_face: GroupCellFace,
    pub north_face: GroupCellFace,
    pub west_face: GroupCellFace,
    pub south_face: GroupCellFace,
}

#[derive(Debug, Default, Clone)]
pub struct GroupCellFace {}

pub fn init() {
    let mut min_height_grad: f32 = 0.0;
    let mut max_height_grad: f32 = 0.0;
    let mut shared_grid = RowMajorGrid::new(GRID_WIDTH, GRID_HEIGHT, SharedCell::default());

    let mut group_a_grid = RowMajorGrid::new(GRID_WIDTH, GRID_HEIGHT, GroupCell::default());
    let mut group_b_grid = RowMajorGrid::new(GRID_WIDTH, GRID_HEIGHT, GroupCell::default());
    let mut group_c_grid = RowMajorGrid::new(GRID_WIDTH, GRID_HEIGHT, GroupCell::default());
    let mut group_d_grid = RowMajorGrid::new(GRID_WIDTH, GRID_HEIGHT, GroupCell::default());
}
