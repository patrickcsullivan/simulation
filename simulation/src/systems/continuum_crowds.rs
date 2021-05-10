use crate::{
    collections::grid::Grid,
    component::{Position, Velocity},
    resources::continuum_crowds::SharedGrid,
};
use specs::{Join, ReadStorage, System, WriteExpect};

pub struct ResetShared;

impl<'a> System<'a> for ResetShared {
    type SystemData = WriteExpect<'a, SharedGrid>;

    fn run(&mut self, data: Self::SystemData) {
        let mut shared_grid = data;
        for (x, y) in shared_grid.0.position_iter() {
            if let Some(mut cell) = shared_grid.0.get_mut(x, y) {
                cell.density = 0.0;
                cell.avg_velocity = (0.0, 0.0);
            }
        }
    }
}

pub struct AssignDensitiesAndVelocities;

/// The density exponent is a designer specified constant that determines the
/// speed of density falloff. For larger values, an agent will contribute less
/// to the density of neigboring cells. For smaller values, an agent will
/// contribute more to the density of neighboring cells.
const DENSITY_EXPONENT: f32 = 1.0;

impl<'a> System<'a> for AssignDensitiesAndVelocities {
    type SystemData = (
        WriteExpect<'a, SharedGrid>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut shared_grid, positions, velocities) = data;

        for (pos, vel) in (&positions, &velocities).join() {
            // Calculate the center of cell "A", the closest cell whose center x
            // and y are both less than pos's x and y. Cell "A" might not exist
            // if it's out of bounds, but we will still calculate all other
            // neighbor cell positions off of it.
            let a_center_x = (pos.x + 0.5).floor() - 0.5;
            let a_center_y = (pos.y + 0.5).floor() - 0.5;

            // Calculate the distance of pos from the center of the cell at
            // (x-1, y-1).
            let delta_x = pos.x - a_center_x;
            let delta_y = pos.y - a_center_y;

            // Cell "A".
            if a_center_x >= 0.0 && a_center_y >= 0.0 {
                let cell_x = a_center_x as usize;
                let cell_y = a_center_y as usize;
                if let Some(mut cell) = shared_grid.0.get_mut(cell_x, cell_y) {
                    let density = (1.0 - delta_x).min(1.0 - delta_y).powf(DENSITY_EXPONENT);
                    cell.density += density;
                    cell.avg_velocity = (
                        cell.avg_velocity.0 + density * vel.x,
                        cell.avg_velocity.1 + density * vel.y,
                    );
                }
            }

            // Cell "B".
            if a_center_y >= 0.0 {
                let cell_x = (a_center_x + 1.0) as usize;
                let cell_y = a_center_y as usize;
                if let Some(mut cell) = shared_grid.0.get_mut(cell_x, cell_y) {
                    let density = (delta_x).min(1.0 - delta_y).powf(DENSITY_EXPONENT);
                    cell.density += density;
                    cell.avg_velocity = (
                        cell.avg_velocity.0 + density * vel.x,
                        cell.avg_velocity.1 + density * vel.y,
                    );
                }
            }

            // Cell "C".
            {
                let cell_x = (a_center_x + 1.0) as usize;
                let cell_y = (a_center_y + 1.0) as usize;
                if let Some(mut cell) = shared_grid.0.get_mut(cell_x, cell_y) {
                    let density = (delta_x).min(delta_y).powf(DENSITY_EXPONENT);
                    cell.density += density;
                    cell.avg_velocity = (
                        cell.avg_velocity.0 + density * vel.x,
                        cell.avg_velocity.1 + density * vel.y,
                    );
                }
            }

            // Cell "D".
            if pos.x >= 0.0 {
                let cell_x = a_center_x as usize;
                let cell_y = (a_center_y + 1.0) as usize;
                if let Some(mut cell) = shared_grid.0.get_mut(cell_x, cell_y) {
                    let density = (1.0 - delta_x).min(delta_y).powf(DENSITY_EXPONENT);
                    cell.density += density;
                    cell.avg_velocity = (
                        cell.avg_velocity.0 + density * vel.x,
                        cell.avg_velocity.1 + density * vel.y,
                    );
                }
            }
        }

        for (x, y) in shared_grid.0.position_iter() {
            if let Some(mut cell) = shared_grid.0.get_mut(x, y) {
                cell.avg_velocity = (
                    cell.avg_velocity.0 / cell.density,
                    cell.avg_velocity.1 / cell.density,
                );
            }
        }
    }
}
