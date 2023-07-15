use bevy_ecs::schedule::Schedule;
use cgmath::*;

use crate::{
    components::{self, AssetType},
    state, systems,
};

pub(crate) struct App {
    schedule: Schedule,
}

impl App {
    pub fn new(state: &mut state::State) -> Self {
        state.world.spawn(components::Asset(AssetType::Required));
        state.world.spawn(components::Asset(AssetType::Model));
        state.world.spawn(components::Camera::new(point3(5.0, 5.0, 10.0)));

        let mut schedule = Schedule::default();
        schedule.add_systems(systems::load_asssets);
        schedule.add_systems(systems::render_model);

        Self { schedule }
    }

    pub fn update(&mut self, state: &mut state::State) {
        self.schedule.run(&mut state.world);
    }
}
