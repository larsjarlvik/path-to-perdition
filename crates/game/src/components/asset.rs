use bevy_ecs::prelude::Component;

#[derive(Debug)]
pub(crate) enum AssetType {
    /** Assets needed to run, mainly shaders. */
    Required,
    /** Load a model */
    Model,
}

/** Request an asset of a specific type to be loaded */
#[derive(Component, Debug)]
pub(crate) struct Asset(pub AssetType);
