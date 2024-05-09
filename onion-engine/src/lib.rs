pub mod context;
pub mod utils;
pub mod types;

use std::f32::consts::PI;

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use context::get_cookies;
use types::{Combo, Cookies, FrontendContext, Game, GameState, Lane, Score};
use wasm_bindgen::prelude::*;

use bevy_web_asset::WebAssetPlugin;
use bevy_kira_audio::prelude::*;

use rand::Rng;

use crate::{types::NoteEntity, utils::{decay_rgb, TOTAL_MS}};

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
      .add_systems(Update, key_input)
      .add_systems(Update, (update_materials, update_transform, keys_game).run_if(in_state(GameState::Playing)))
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
            top: Val::Percent(0.0),
            left: Val::Percent(88.0),
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
                    font_size: 20.0,
                    color: Color::WHITE,
                    font: Default::default()
            }));
        }
    );

    commands.spawn((
      TextBundle::from_section(
          "Score: 0",
          TextStyle {
              font_size: 20.0,
              ..default()
          },
      ).with_text_justify(JustifyText::Right)
      .with_style(Style {
          position_type: PositionType::Relative,
          top: Val::Percent(3.0),
          left: Val::Percent(90.0),
          ..default()
      }),
      Score)
    );

    commands.spawn((
      TextBundle::from_section(
          "Combo: 0",
          TextStyle {
              font_size: 20.0,
              ..default()
          },
      ).with_text_justify(JustifyText::Right)
      .with_style(Style {
          position_type: PositionType::Relative,
          top: Val::Percent(6.0),
          left: Val::Percent(90.0),
          ..default()
      }),
      Combo)
    );
    let shapes = [
        meshes.add(Cuboid::default()),
        meshes.add(Capsule3d::default()),
        meshes.add(Cylinder::default()),
    ];

    let num_shapes = shapes.len();

    game.accuracy_shape = meshes.add(Rectangle::default());

    for line in config.events.lines() {
      let parts: Vec<&str> = line.split(",").collect();
      let time = parts[0].parse::<f32>().unwrap();
      let lane = parts[1].parse::<usize>().unwrap();
      let shape = &shapes[rand::thread_rng().gen_range(0..num_shapes)];
      let mat = materials.add(Color::rgb(10.0, 10.0, 10.0));
      let id = commands.spawn((
        PbrBundle {
            mesh: shape.clone(),
            material: mat.clone(),
            transform: Transform::from_xyz(
                -LANE_EXTENT / 2. + lane as f32 / (NUM_LANES - 1) as f32 * LANE_EXTENT,
                0.,
                -time*NOTE_SPEED + 6.0,
            ).with_rotation(Quat::from_rotation_x(-PI / 4.)),
            ..default()
        },
        NoteEntity,
      )).id();
      game.notes.push(types::Note {
        entity: Some(id),
        material: mat.clone(),
        lane,
        time,
        hit: false,
      });
    }

    game.lanes = (0..NUM_LANES).map(|i| {
      let material = materials.add(Color::rgb(0.75, 0.75, 0.75));
      let id = commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cylinder::default().mesh()),
            material: material.clone(),
            transform: Transform::from_xyz(
                -LANE_EXTENT / 2. + i as f32 / (NUM_LANES - 1) as f32 * LANE_EXTENT,
                -0.2,
                6.0,
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
    //         intensity: 10_000.,
    //         range: 100.0,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(8.0, 16.0, 8.0),
    //     ..default()
    // });

    commands.spawn(PbrBundle {
      transform: Transform::from_xyz(LANE_EXTENT - 2.0, 0.1, 0.0),
      mesh: meshes.add(Plane3d::default().mesh().size(0.1, 50.0)),
      material: materials.add(Color::rgb(10., 10., 10.)),
      ..default()
    });
    commands.spawn(PbrBundle {
      transform: Transform::from_xyz(-LANE_EXTENT + 2.0, 0.1, 0.0),
      mesh: meshes.add(Plane3d::default().mesh().size(0.1, 50.0)),
      material: materials.add(Color::rgb(10., 10., 10.)),
      ..default()
    });
    commands.spawn(PbrBundle {
      transform: Transform::from_xyz(0.0, 0.05, 0.0),
      mesh: meshes.add(Plane3d::default().mesh().size(8., 50.0)),
      material: materials.add(Color::rgb(0.5, 0.5, 0.5)),
      ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(Color::BLACK),
        ..default()
    });

    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 15., 10.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
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
            start: 15.0,
            end: 35.0,
        },
        ..default()
      }
    ));
}


const NOTE_SCALE_DECAY_RATE: f32 = 30.;
fn update_transform(mut query: Query<&mut Transform, With<NoteEntity>>, game: Res<Game>, time: Res<Time>) {
  for note in &game.notes {
    if let Some(entity) = note.entity {
      let mut transform = query.get_mut(entity).unwrap();
      transform.rotate_y(time.delta_seconds() / 2.);
      transform.translation.z += NOTE_SPEED * time.delta_seconds()*1000.;
      if note.hit {
        transform.scale = Vec3::max(transform.scale - Vec3::splat(NOTE_SCALE_DECAY_RATE * time.delta_seconds()), Vec3::splat(0.0));
      }
    }
  }
}

const PRESS_DECAY_SPEED: f32 = 20.0;
const ACCURACY_BAR_DECAY_SPEED: f32 = 10.0;
const NOTE_DECAY_SPEED: f32 = 100.0;
fn update_materials(mut materials: ResMut<Assets<StandardMaterial>>, game: Res<Game>, time: Res<Time>) {
  for lane in &game.lanes {
    let material = materials.get_mut(&lane.material).unwrap();
    material.base_color = decay_rgb(&material.base_color, PRESS_DECAY_SPEED, 0.75, time.delta_seconds());
  }
  for bar in &game.accuracy_bars {
    let material = materials.get_mut(&bar.material).unwrap();
    material.base_color = decay_rgb(&material.base_color, ACCURACY_BAR_DECAY_SPEED, 0., time.delta_seconds());
  }
  for note in &game.notes {
    if note.hit {
      let material = materials.get_mut(&note.material).unwrap();
      material.base_color = decay_rgb(&material.base_color, NOTE_DECAY_SPEED, 0., time.delta_seconds());
    }
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

fn keys_game(
  mut commands: Commands,
  mut materials: ResMut<Assets<StandardMaterial>>,
  keyboard_input: Res<ButtonInput<KeyCode>>, 
  mut game: ResMut<Game>,
  time: Res<Time>,
  mut score: Query<&mut Text, (With<Score>, Without<Combo>)>,
  mut combo: Query<&mut Text, (With<Combo>, Without<Score>)>,
  camera: Query<&Transform, With<Camera>>) {
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

      for note_i in active_notes {
        let hit = utils::calculate_hit((cur_time * 1000.) as i32, game.notes[note_i].time as i32);
        let score = utils::calculate_score(hit);
        game.score += score;
        if hit != types::HitType::None {
          game.notes[note_i].hit = true;
          
          println!("HIT: {:?}", hit);
          game.combo += 1;
          let cam = camera.single();

          let hit_offset = cur_time * 1000. - game.notes[note_i].time;
          let percent_off = hit_offset / (TOTAL_MS as f32 / 2.);
          // let accuracy_height = 
          let accuracy_render_spot = cam.translation + cam.rotation * Vec3::new(3.5, -2.*percent_off, -5.0);

          let bar_material = materials.add(StandardMaterial {
            base_color: utils::hit_color(hit),
            unlit: true,
            ..Default::default()
          });
          let accuracy_bar = commands.spawn((
            PbrBundle {
              mesh: game.accuracy_shape.clone(),
              material: bar_material.clone(),
              transform: Transform {
                translation: accuracy_render_spot,
                rotation: Quat::from_rotation_x(-PI / 4.),
                scale: Vec3::new(0.5, 0.01, 0.01),
                ..Default::default()
              },
              ..Default::default()
            },
          )).id();
          game.accuracy_bars.push(types::AccuracyBar {
            entity: Some(accuracy_bar),
            material: bar_material,
          });

        } else {
          game.combo = 0;
          game.health -= 1.0;
        }
      }
      score.single_mut().sections[0].value = format!("Score: {}", game.score);
    }
    combo.single_mut().sections[0].value = format!("Combo: {}", game.combo);
}