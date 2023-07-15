use bevy_ecs::{schedule::Schedule, world};

use crate::{
    components::{self, AssetType},
    resources, systems,
};

pub(crate) struct App {
    pub world: world::World,
    schedule: Schedule,
}

impl App {
    pub fn new(instance: wgpu::Instance) -> Self {
        let mut world: world::World = world::World::new();

        let state = resources::State {
            instance,
            adapter: None,
            surface_state: None,
            render_state: None,
        };

        world.insert_resource(state);
        world.spawn(components::Asset(AssetType::Default));
        world.spawn(components::Asset(AssetType::Model));

        let mut schedule = Schedule::default();
        schedule.add_systems(systems::load_asssets);
        schedule.add_systems(systems::render_model);

        Self { world, schedule }
    }

    pub fn update(&mut self) {
        self.schedule.run(&mut self.world);
    }
}
