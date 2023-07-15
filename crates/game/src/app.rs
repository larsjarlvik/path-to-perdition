use bevy_ecs::schedule::Schedule;

use crate::{
    components::{self, AssetType},
    state, systems,
};

pub(crate) struct App {
    schedule: Schedule,
}

impl App {
    pub fn new(state: &mut state::State) -> Self {
        state.world.spawn(components::Asset(AssetType::Default));
        state.world.spawn(components::Asset(AssetType::Model));

        let mut schedule = Schedule::default();
        schedule.add_systems(systems::load_asssets);
        schedule.add_systems(systems::render_model);

        Self { schedule }
    }

    pub fn update(&mut self, state: &mut state::State) {
        self.schedule.run(&mut state.world);
    }
}
