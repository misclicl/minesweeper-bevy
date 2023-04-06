use serde::{Deserialize, Serialize};
use bevy::prelude::*;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::InspectorOptions))]
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
// #[reflect(Resource)]
pub struct Uncover;
