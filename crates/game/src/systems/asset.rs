use crate::{components, resources};
use bevy_ecs::prelude::*;

pub(crate) fn load_asssets(mut commands: Commands, state: Res<resources::State>, query: Query<(Entity, &components::Asset)>) {
    if let Some(render_state) = &state.render_state {
        for (entity, asset) in query.iter() {
            match &asset.0 {
                components::AssetType::Default => {
                    load_default_assets(&mut commands, render_state);
                }
                components::AssetType::Model => {
                    let model = components::Model::new(render_state);
                    commands.spawn(model);
                }
            }

            commands.entity(entity).remove::<components::Asset>();
        }
    }
}

fn load_default_assets(commands: &mut Commands, render_state: &resources::RenderState) {
    let pbr = resources::Pbr::new(render_state);
    commands.insert_resource(pbr);
}
