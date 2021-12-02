use bevy::prelude::*;
// Game State resource
pub struct GameState {
    pub game_paused: bool,
    pub game_step: bool,
    pub brush_size: usize,
    pub grid_height: f32,
    pub grid_width: f32,
    pub grid_step: f32,
    pub selected_particle: usize,
}
// Grid Resource
#[derive(Default)]
pub struct Grid(pub Vec<Vec<f32>>);

pub struct ParticleLocations {
    pub old_location: Vec2,
    pub new_location: Vec2,
}
#[derive(Debug, Clone, Copy)]
pub struct ParticleData {
    pub density: f32,
    pub particle_type: ParticleType,
    pub material: Color,
}
#[derive(Default)]
pub struct ParticleList(pub Vec<ParticleData>);
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum ParticleType {
    Solid,
    Liquid,
    Gas,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum MyStage {
    BeforeMove,
    AfterMove,
}