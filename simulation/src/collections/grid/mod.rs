mod row_major_grid;

pub use row_major_grid::RowMajorGrid;
pub trait Grid<T> {
    fn get(&self, x: usize, y: usize) -> Option<&T>;
    fn set(&mut self, x: usize, y: usize, val: T);
}
