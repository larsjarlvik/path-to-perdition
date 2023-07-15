use crate::{components, resources};
use bevy_ecs::prelude::*;

/** Loads assets and spawns necessary components and resources based on request */
pub(crate) fn load_asssets(
    mut commands: Commands,
    ctx: Res<resources::RenderContext>,
    surface: Res<resources::Surface>,
    query: Query<(Entity, &components::Asset)>,
) {
    for (entity, asset) in query.iter() {
        match &asset.0 {
            components::AssetType::Required => {
                load_default_assets(&mut commands, &ctx, &surface);
            }
            components::AssetType::Model => {
                let model = components::Model::new(&ctx);
                commands.spawn(model);
            }
        }

        commands.entity(entity).remove::<components::Asset>();
    }
}

fn load_default_assets(commands: &mut Commands, ctx: &resources::RenderContext, surface: &resources::Surface) {
    let sample_count = resources::Msaa::get_max_sample_count(surface, ctx.target_format);

    let size = surface.window.inner_size();
    let msaa = resources::Msaa::new(ctx, sample_count, size.width, size.height);
    let pbr = resources::Pbr::new(ctx, sample_count);

    commands.insert_resource(msaa);
    commands.insert_resource(pbr);
}
