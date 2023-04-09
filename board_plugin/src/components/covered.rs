use bevy::prelude::*;
#[cfg(feature = "debug")]
use serde::{Deserialize, Serialize};

#[cfg_attr(
    feature = "debug",
    derive(
        Debug,
        bevy_inspector_egui::InspectorOptions,
        Reflect,
        Serialize,
        Deserialize
    )
)]
#[derive(Component, Clone)]
pub struct Covered {
    pub is_covered: bool
}
