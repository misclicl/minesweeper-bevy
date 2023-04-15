use bevy::prelude::Entity;

#[derive(Debug, Copy, Clone)]
pub struct TileDiscoverEvent(pub Entity);

#[derive(Debug, Copy, Clone)]
pub struct BombExplosionEvent;

#[derive(Debug, Copy, Clone)]
pub struct TileMarkEvent(pub Entity);
