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
