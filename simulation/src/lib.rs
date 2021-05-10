pub mod collections;
pub mod component;
pub mod frame;
pub mod resources;
pub mod systems;

use collections::grid::RowMajorGrid;
use resources::continuum_crowds::{GroupCell, SharedCell};

// 1 cell is 4 m wide

const GRID_WIDTH: usize = 8; // 8 cells wide
const GRID_HEIGHT: usize = 8; // 8 cells high

pub fn init() {
    let mut min_height_grad: f32 = 0.0;
    let mut max_height_grad: f32 = 0.0;
    let mut shared_grid = RowMajorGrid::new(GRID_WIDTH, GRID_HEIGHT, SharedCell::default());

    let mut group_a_grid = RowMajorGrid::new(GRID_WIDTH, GRID_HEIGHT, GroupCell::default());
    let mut group_b_grid = RowMajorGrid::new(GRID_WIDTH, GRID_HEIGHT, GroupCell::default());
    let mut group_c_grid = RowMajorGrid::new(GRID_WIDTH, GRID_HEIGHT, GroupCell::default());
    let mut group_d_grid = RowMajorGrid::new(GRID_WIDTH, GRID_HEIGHT, GroupCell::default());
}
