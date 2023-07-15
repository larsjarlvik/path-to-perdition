use bevy_ecs::prelude::Component;

#[derive(Debug)]
pub(crate) enum AssetType {
    Default,
    Model,
}

#[derive(Component, Debug)]
pub(crate) struct Asset(pub AssetType);
