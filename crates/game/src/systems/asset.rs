use crate::{components, resources};
use bevy_ecs::prelude::*;

/** Loads assets and spawns necessary components and resources based on request */
pub(crate) fn load_asssets(mut commands: Commands, ctx: Res<resources::RenderContext>, query: Query<(Entity, &components::Asset)>) {
    for (entity, asset) in query.iter() {
        match &asset.0 {
            components::AssetType::Required => {
                load_default_assets(&mut commands, &ctx);
            }
            components::AssetType::Model => {
                let model = components::Model::new(&ctx);
                commands.spawn(model);
            }
        }

        commands.entity(entity).remove::<components::Asset>();
    }
}

fn load_default_assets(commands: &mut Commands, ctx: &resources::RenderContext) {
    let pbr = resources::Pbr::new(ctx);
    commands.insert_resource(pbr);
}
