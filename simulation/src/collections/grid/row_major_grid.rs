use super::Grid;

/// Grid in which cells are stored in row-major order.
#[derive(Debug)]
pub struct RowMajorGrid<T> {
    inner_width: usize,
    inner_height: usize,
    x_offset: usize,
    y_offset: usize,
    cells: Vec<T>,
}

impl<T: Clone> RowMajorGrid<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        RowMajorGrid {
            inner_width: width,
            inner_height: height,
            x_offset: 0,
            y_offset: 0,
            cells: vec![default; width * height],
        }
    }

    pub fn new_sub_grid(
        inner_width: usize,
        inner_height: usize,
        x_offset: usize,
        y_offset: usize,
        default: T,
    ) -> Self {
        RowMajorGrid {
            inner_width,
            inner_height,
            x_offset,
            y_offset,
            cells: vec![default; inner_width * inner_height],
        }
    }
}

impl<T> RowMajorGrid<T> {
    fn index(&self, x: usize, y: usize) -> Option<usize> {
        pos_to_index(
            x,
            y,
            self.inner_width,
            self.inner_height,
            self.x_offset,
            self.y_offset,
        )
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x >= self.x_offset
            && x < self.x_offset + self.inner_width
            && y >= self.y_offset
            && y < self.y_offset + self.inner_height
    }

    pub fn position_iter(&self) -> PositionIter {
        PositionIter::new(
            self.inner_width,
            self.inner_height,
            self.x_offset,
            self.y_offset,
        )
    }

    pub fn row_iter(&self) -> RowIter<T> {
        RowIter {
            grid: &self,
            prev_row: None,
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

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if let Some(i) = self.index(x, y) {
            Some(&mut self.cells[i])
        } else {
            None
        }
    }

    fn set(&mut self, x: usize, y: usize, val: T) {
        if let Some(i) = self.index(x, y) {
            self.cells[i] = val
        }
    }
}

pub struct Iter<'a, T> {
    // Iter must live as long as the grid.
    grid: &'a RowMajorGrid<T>,
    prev_index: Option<usize>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let next_index = if let Some(index) = self.prev_index {
            index + 1
        } else {
            0
        };

        let next_pos = index_to_pos(
            next_index,
            self.grid.inner_width,
            self.grid.inner_height,
            self.grid.x_offset,
            self.grid.y_offset,
        );

        if let Some((x, y)) = next_pos {
            self.prev_index = Some(next_index);
            Some((x, y, &self.grid.cells[next_index]))
        } else {
            None
        }
    }
}

impl<'a, T> IntoIterator for &'a RowMajorGrid<T> {
    type Item = (usize, usize, &'a T);
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            grid: self,
            prev_index: None,
        }
    }
}

pub struct PositionIter {
    inner_width: usize,
    inner_height: usize,
    x_offset: usize,
    y_offset: usize,
    prev: Option<(usize, usize)>,
}

impl PositionIter {
    fn new(inner_width: usize, inner_height: usize, x_offset: usize, y_offset: usize) -> Self {
        PositionIter {
            inner_width,
            inner_height,
            x_offset,
            y_offset,
            prev: None,
        }
    }
}

impl Iterator for PositionIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.prev {
            // Grid has no cells.
            None if self.inner_width == 0 || self.inner_height == 0 => None,
            // Start the iterator at the origin.
            None => Some((self.x_offset, self.y_offset)),
            // Move to the next column.
            Some((x, y)) if x < self.x_offset + self.inner_width - 1 => Some((x + 1, y)),
            // Move to the first column of the next row.
            Some((_, y)) if y < self.y_offset + self.inner_height - 1 => {
                Some((self.x_offset, y + 1))
            }
            // There are no more columns or rows to move to.
            _ => None,
        };
        self.prev = next;
        next
    }
}

pub struct RowIter<'a, T> {
    // Iter must live as long as the grid.
    grid: &'a RowMajorGrid<T>,
    prev_row: Option<usize>,
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        let next_row = if let Some(row) = self.prev_row {
            row + 1
        } else {
            0
        };

        if next_row < self.grid.inner_height {
            self.prev_row = Some(next_row);
            let start = next_row * self.grid.inner_width;
            let end = start + self.grid.inner_width;
            Some(&self.grid.cells[start..end])
        } else {
            None
        }
    }
}

/// Calcuate the x and y position that correspondes to the given index.
fn index_to_pos(
    index: usize,
    inner_width: usize,
    inner_height: usize,
    x_offset: usize,
    y_offset: usize,
) -> Option<(usize, usize)> {
    if index < inner_width * inner_height {
        None
    } else {
        Some((
            index % inner_width + x_offset,
            index / inner_height + y_offset,
        ))
    }
}

/// Calcuate the index that corresponds to the given x and y position.
fn pos_to_index(
    x: usize,
    y: usize,
    inner_width: usize,
    inner_height: usize,
    x_offset: usize,
    y_offset: usize,
) -> Option<usize> {
    if x >= x_offset && x < x_offset + inner_width && y >= y_offset && y < y_offset + inner_height {
        // Calculate the requested x and y, relative the offset of the sub
        // grid's origin.
        let x = x - x_offset;
        let y = y - y_offset;

        Some(y * inner_width + x)
    } else {
        None
    }
}
