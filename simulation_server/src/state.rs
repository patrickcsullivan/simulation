use simulation::{
    collections::grid::RowMajorGrid,
    component::{Position, Velocity},
    frame::Frame,
    resources::continuum_crowds::{GroupCell, GroupGrids, SharedCell, SharedGrid},
    systems::{
        continuum_crowds::{AssignDensitiesAndVelocities, PrintDensityGrid, ResetShared},
        UpdatePos,
    },
};
use specs::prelude::*;

pub struct State<'a, 'b> {
    pub world: World,
    pub dispatcher: Dispatcher<'a, 'b>,
    pub frame: Option<Frame>,
}

impl State<'_, '_> {
    pub fn new() -> Self {
        let mut world = World::new();

        Self::register_component(&mut world);
        Self::initialize_entities(&mut world);
        Self::initialize_resources(&mut world);

        let mut dispatcher = DispatcherBuilder::new()
            .with(ResetShared, "reset_shared", &[])
            .with(
                AssignDensitiesAndVelocities,
                "assign_densities_and_velocities",
                &["reset_shared"],
            )
            .with(
                PrintDensityGrid,
                "print_density_grid",
                &["assign_densities_and_velocities"],
            )
            .with(
                UpdatePos,
                "update_pos",
                &["assign_densities_and_velocities"],
            )
            .build();
        dispatcher.setup(&mut world);

        State {
            world,
            dispatcher,
            frame: None,
        }
    }

    fn register_component(world: &mut World) {
        world.register::<Position>();
        world.register::<Velocity>();
    }

    fn initialize_entities(world: &mut World) {
        for x in 1..=10 {
            for y in 1..=10 {
                world
                    .create_entity()
                    .with(Position {
                        x: x as f32 * 0.3,
                        y: y as f32 * 0.3,
                    })
                    .with(Velocity { x: 0.2, y: 0.2 })
                    .build();
            }
        }
    }

    fn initialize_resources(world: &mut World) {
        // 1 cell is 4 m wide
        const GRID_WIDTH: usize = 16;
        const GRID_HEIGHT: usize = 16;
        let shared_grid = SharedGrid(RowMajorGrid::new(
            GRID_WIDTH,
            GRID_HEIGHT,
            SharedCell::default(),
        ));
        let group_grids = GroupGrids(
            RowMajorGrid::new(GRID_WIDTH, GRID_HEIGHT, GroupCell::default()),
            RowMajorGrid::new(GRID_WIDTH, GRID_HEIGHT, GroupCell::default()),
            RowMajorGrid::new(GRID_WIDTH, GRID_HEIGHT, GroupCell::default()),
            RowMajorGrid::new(GRID_WIDTH, GRID_HEIGHT, GroupCell::default()),
        );
        world.insert(shared_grid);
        world.insert(group_grids);
    }
}
