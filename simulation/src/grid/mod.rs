pub trait Grid<T> {
    fn get(&self, x: usize, y: usize) -> Option<&T>;
    fn set(&mut self, x: usize, y: usize, val: T);
    fn cells(&self) -> &Vec<T>;
}

/// Grid in which cells are stored in row-major order.
pub struct RowMajorGrid<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T: Clone> RowMajorGrid<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        RowMajorGrid {
            width,
            height,
            cells: vec![default; width * height],
        }
    }

    fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }
}

impl<T: Clone> Grid<T> for RowMajorGrid<T> {
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        if let Some(i) = self.index(x, y) {
            Some(&self.cells[i])
        } else {
            None
        }
    }

    fn set(&mut self, x: usize, y: usize, val: T) {
        if let Some(i) = self.index(x, y) {
            self.cells[i] = val
        }
    }

    fn cells(&self) -> &Vec<T> {
        &self.cells
    }
}
