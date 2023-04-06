use bevy::{prelude::Component, reflect::Reflect};

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::InspectorOptions))]
#[derive(Debug, Default, Clone, Component, Reflect)]
pub struct BombNeighbor {
    pub count: u8,
}
