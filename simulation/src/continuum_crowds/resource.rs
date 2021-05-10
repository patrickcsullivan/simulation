use super::{GroupCell, SharedCell};
use crate::collections::grid::RowMajorGrid;

#[derive(Debug)]
pub struct SharedGrid(pub RowMajorGrid<SharedCell>);

#[derive(Debug)]
pub struct GroupGrids(
    pub RowMajorGrid<GroupCell>,
    pub RowMajorGrid<GroupCell>,
    pub RowMajorGrid<GroupCell>,
    pub RowMajorGrid<GroupCell>,
);
