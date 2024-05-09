pub mod context;
pub mod utils;
pub mod types;

use std::f32::consts::PI;

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use context::get_cookies;
use types::{Cookies, FrontendContext, Game, GameState, Lane};
use wasm_bindgen::prelude::*;

use bevy_web_asset::WebAssetPlugin;
use bevy_kira_audio::prelude::*;

use rand::Rng;

use crate::types::NoteEntity;

const TEST_BEATMAP_EVENTS: &str = "1000,0,0\n2000,1,0\n3000,2,0\n4000,3,0\n5000,0,0\n6000,1,0\n7000,2,0\n8000,3,0";

pub fn main() {
  run_onion_engine("EPIC CHILL SONG".to_string(),TEST_BEATMAP_EVENTS.to_string(), "www.google.com".to_string(), "csrftoken=1234;uid=5678".to_string());
}

#[wasm_bindgen]
pub fn run_onion_engine(beatmap_title: String, beatmap_events: String, beatmap_song_url: String, cookie_string: String) {
  let cookies: Cookies = get_cookies(cookie_string);

  let config = FrontendContext {
    title: beatmap_title,
    events: beatmap_events,
    song_url: beatmap_song_url,
    csrf: cookies.csrf,
    uid: cookies.uid,
  };
  let config_clone = config.clone();
  // i am so lazy

  App::new()
      .add_plugins((WebAssetPlugin::default(), DefaultPlugins, AudioPlugin))
      .add_plugins(EguiPlugin)
      .init_resource::<Game>()
      .init_state::<GameState>()
      .add_systems(Startup, move 
        |commands: Commands, meshes: ResMut<Assets<Mesh>>, materials: ResMut<Assets<StandardMaterial>>, game: ResMut<Game>| {
          setup(commands, meshes, materials, game, &config);
        })
      .add_systems(Update, (start_game_key).run_if(in_state(GameState::Paused)))
      .add_systems(OnEnter(GameState::Playing), start_bg_audio)
      .add_systems(Update, (key_input, update_materials))
      .add_systems(Update, (keys_game, update_transform).run_if(in_state(GameState::Playing)))
      .add_systems(Update, move |contexts: EguiContexts| {
        context_debug_ui(contexts, &config_clone);
      })
      .run();
}

fn start_game_key(keyboard_input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>, mut game: ResMut<Game>, time: Res<Time>) {
  if keyboard_input.just_pressed(KeyCode::Space) {
    game.start_time = time.elapsed_seconds();
    next_state.set(GameState::Playing);
  }
}

fn start_bg_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
  audio.play(asset_server.load("audio.ogg")).looped();
}

fn context_debug_ui(mut contexts: EguiContexts, config: &FrontendContext) {
  egui::Window::new("FRONTEND CONTEXT").show(contexts.ctx_mut(), |ui| {
      ui.label("SONG URL: ".to_string() + &config.song_url.clone());
      if config.csrf.is_some() {
        ui.label("CSRF: ".to_string() + &config.csrf.clone().unwrap());
      }
      if config.uid.is_some() {
        ui.label("UID: ".to_string() + &config.uid.clone().unwrap());
      }
  });
}


const NUM_LANES: usize = 4;
const LANE_EXTENT: f32 = 6.0;
const NOTE_SPEED: f32 = 20.0 / 1000.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game: ResMut<Game>,
    config: &FrontendContext,
) { 

    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Relative,
            top: Val::Percent(-8.0),
            left: Val::Percent(30.0),
            width: Val::Px(600.0),
            height: Val::Px(200.0),
            ..Default::default()
        },
        ..Default::default()
    }).with_children(
        |parent| {
          parent.spawn(TextBundle::from_section(
            config.title.clone(), 
            TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    font: Default::default()
            }));
        }
    );

    let debug_material = materials.add(Color::rgb(10.0, 10.0, 10.0));

    let shapes = [
        meshes.add(Cuboid::default()),
        meshes.add(Capsule3d::default()),
        meshes.add(Cylinder::default()),
    ];

    let num_shapes = shapes.len();

    for line in config.events.lines() {
      let parts: Vec<&str> = line.split(",").collect();
      let time = parts[0].parse::<f32>().unwrap();
      let lane = parts[1].parse::<usize>().unwrap();
      let shape = &shapes[rand::thread_rng().gen_range(0..num_shapes)];
      let id = commands.spawn((
        PbrBundle {
            mesh: shape.clone(),
            material: debug_material.clone(),
            transform: Transform::from_xyz(
                -LANE_EXTENT / 2. + lane as f32 / (NUM_LANES - 1) as f32 * LANE_EXTENT,
                2.0,
                -time*NOTE_SPEED,
            ).with_rotation(Quat::from_rotation_x(-PI / 4.)),
            ..default()
        },
        NoteEntity,
      )).id();
      game.notes.push(types::Note {
        entity: Some(id),
        material: debug_material.clone(),
        lane,
        time,
      });
    }

    game.lanes = (0..NUM_LANES).map(|i| {
      let material = materials.add(Color::rgb(0.5, 0.5, 0.5));
      let id = commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cylinder::default().mesh()),
            material: material.clone(),
            transform: Transform::from_xyz(
                -LANE_EXTENT / 2. + i as f32 / (NUM_LANES - 1) as f32 * LANE_EXTENT,
                -0.2,
                5.0,
            ),
            ..default()
        },
      )).id();
      Lane {
        entity: Some(id),
        material: material.clone(),
      }
    }).collect();

  
    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         shadows_enabled: true,
    //         intensity: 10_000_000.,
    //         range: 100.0,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(8.0, 16.0, 8.0),
    //     ..default()
    // });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(Color::BLACK),
        ..default()
    });

    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        camera: Camera {
          hdr: true,
          ..default()
        },
        ..default()
      }, 
      BloomSettings::OLD_SCHOOL,
      FogSettings {
        color: Color::rgba(0., 0., 0., 1.0),
        falloff: FogFalloff::Linear {
            start: 5.0,
            end: 30.0,
        },
        ..default()
      }
    ));
}

fn update_transform(mut query: Query<&mut Transform, With<NoteEntity>>, game: Res<Game>, time: Res<Time>) {
  for mut transform in &mut query {
      transform.rotate_y(time.delta_seconds() / 2.);
      transform.translation.z += NOTE_SPEED * time.delta_seconds()*1000.;
  }
}

const DECAY_SPEED: f32 = 20.0;
fn update_materials(mut materials: ResMut<Assets<StandardMaterial>>, game: Res<Game>, time: Res<Time>) {
  for lane in &game.lanes {
    let material = materials.get_mut(&lane.material).unwrap();
    let r_val = material.base_color.r();
    let new_r_val = (r_val - DECAY_SPEED * time.delta_seconds() as f32).max(0.5);
    material.base_color = Color::rgb(new_r_val, 0.5, 0.5);
  }
}

fn key_input(keyboard_input: Res<ButtonInput<KeyCode>>, 
  mut materials: ResMut<Assets<StandardMaterial>>,
  game: ResMut<Game>) {
    let mut keys_pressed = Vec::new();
    if keyboard_input.pressed(KeyCode::KeyC) {
      keys_pressed.push(&game.lanes[0]);
    } if keyboard_input.pressed(KeyCode::KeyV) {
      keys_pressed.push(&game.lanes[1]);
    } if keyboard_input.pressed(KeyCode::KeyN) {
      keys_pressed.push(&game.lanes[2]);
    } if keyboard_input.pressed(KeyCode::KeyM) {
      keys_pressed.push(&game.lanes[3]);
    }

    for lane in keys_pressed {
      let material = materials.get_mut(&lane.material).unwrap();
      material.base_color = Color::rgb(10.0, 0.0, 0.0);
    }
}

fn keys_game(keyboard_input: Res<ButtonInput<KeyCode>>, 
  mut game: ResMut<Game>,
  time: Res<Time>) {
    let mut keys_pressed = Vec::new();
    if keyboard_input.just_pressed(KeyCode::KeyC) {
      keys_pressed.push(&game.lanes[0]);
    } if keyboard_input.just_pressed(KeyCode::KeyV) {
      keys_pressed.push(&game.lanes[1]);
    } if keyboard_input.just_pressed(KeyCode::KeyN) {
      keys_pressed.push(&game.lanes[2]);
    } if keyboard_input.just_pressed(KeyCode::KeyM) {
      keys_pressed.push(&game.lanes[3]);
    }
    if keys_pressed.len() != 0 {
      let cur_time = time.elapsed_seconds() - game.start_time;

      let active_notes = utils::active_notes(&game.notes, cur_time*1000.);
      // println!("ACTIVE NOTES: {:?}", active_notes.len());

      for note in active_notes {
        let hit = utils::calculate_hit((cur_time * 1000.) as i32, note.time as i32);
        let score = utils::calculate_score(hit);
        game.score += score;
        if hit != types::HitType::None {
          println!("HIT: {:?}, SCORE: {}", hit, game.score);
          game.combo += 1;
        } else {
          game.combo = 0;
          game.health -= 1.0;
        }
      }
    }
}