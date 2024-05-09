use bevy::prelude::*;

#[derive(Default)]
pub struct Lane {
  pub entity: Option<Entity>,
  pub material: Handle<StandardMaterial>,
}

#[derive(Component)]
pub struct NoteEntity;

#[derive(Component)]
pub struct Score;

#[derive(Component)]
pub struct Combo;

#[derive(Default)]
pub struct AccuracyBar {
  pub entity: Option<Entity>,
  pub material: Handle<StandardMaterial>,
}

#[derive(Default, Clone)]
pub struct Note {
  pub entity: Option<Entity>,
  pub material: Handle<StandardMaterial>,

  pub lane: usize,
  pub time: f32,
  pub hit: bool,
}

#[derive(Resource, Default)]
pub struct Game {
  pub lanes: Vec<Lane>,
  pub notes: Vec<Note>,
  pub accuracy_bars: Vec<AccuracyBar>,
  pub accuracy_shape: Handle<Mesh>,
  pub score: i32,
  pub combo: i32,
  pub max_combo: i32,
  pub health: f32,
  pub start_time: f32,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Paused,
    Playing,
    GameOver,
}


#[derive(Default, Clone)]
pub struct FrontendContext {
    pub title: String,
    pub events: String,
    pub song_url: String,
    pub csrf: Option<String>,
    pub uid: Option<String>,
}

pub struct Cookies {
  pub csrf: Option<String>,
  pub uid: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitType {
  Perfect,
  Great,
  Good,
  Miss,
  None,
}