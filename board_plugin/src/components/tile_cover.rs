use bevy::{prelude::Component, reflect::Reflect};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::InspectorOptions))]
#[derive(Debug, Default, Copy, Clone, Component, Reflect, Serialize, Deserialize)]
pub struct TileCover;
