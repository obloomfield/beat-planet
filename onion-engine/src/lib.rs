pub mod context;

use std::f32::consts::PI;

use bevy::{
    core_pipeline::bloom::BloomSettings, prelude::*, render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    }
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use context::{get_cookies, Cookies};
use wasm_bindgen::prelude::*;

use bevy_web_asset::WebAssetPlugin;
use bevy_kira_audio::prelude::*;

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

#[derive(Default)]
struct Lane {
  entity: Option<Entity>,
  material: Handle<StandardMaterial>,
}

#[derive(Resource, Default)]
struct Game {
  lanes: Vec<Lane>,
  score: i32,

}

#[derive(Default, Clone)]
struct FrontendContext {
    title: String,
    events: String,
    song_url: String,
    csrf: Option<String>,
    uid: Option<String>,
}

pub fn main() {
  run_onion_engine("EPIC CHILL SONG".to_string(),"test events".to_string(), "www.google.com".to_string(), "csrftoken=1234;uid=5678".to_string());
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
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
      .add_systems(Startup, start_bg_audio)
      .add_systems(Startup, move 
        |commands: Commands, meshes: ResMut<Assets<Mesh>>, materials: ResMut<Assets<StandardMaterial>>, game: ResMut<Game>| {
          setup(commands, meshes, materials, game, &config);
        })
      .add_systems(Update, rotate)
      .add_systems(Update, move |contexts: EguiContexts| {
        ui_example_system(contexts, &config_clone);
      })
      .add_systems(Update, (key_input, update_materials))
      .run();
}

fn start_bg_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
  audio.play(asset_server.load("audio.ogg")).looped();
}

fn ui_example_system(mut contexts: EguiContexts, config: &FrontendContext) {
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

const X_EXTENT: f32 = 12.0;
const NUM_LANES: usize = 4;
const LANE_EXTENT: f32 = 6.0;

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
        meshes.add(Torus::default()),
        meshes.add(Cylinder::default()),
        meshes.add(Sphere::default().mesh().ico(5).unwrap()),
        meshes.add(Sphere::default().mesh().uv(32, 18)),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            PbrBundle {
                mesh: shape,
                material: debug_material.clone(),
                transform: Transform::from_xyz(
                    -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                    2.0,
                    0.0,
                )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                ..default()
            },
            Shape,
        ));
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
    }, BloomSettings::OLD_SCHOOL));
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
  for mut transform in &mut query {
      transform.rotate_y(time.delta_seconds() / 2.);
  }
}

fn key_input(keyboard_input: Res<ButtonInput<KeyCode>>, 
  mut materials: ResMut<Assets<StandardMaterial>>,
  game: ResMut<Game>) {
  // println!("PRESS: {:?}", keyboard_input);
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

const DECAY_SPEED: f32 = 20.0;
fn update_materials(mut materials: ResMut<Assets<StandardMaterial>>, game: Res<Game>, time: Res<Time>) {
  for lane in &game.lanes {
    let material = materials.get_mut(&lane.material).unwrap();
    let r_val = material.base_color.r();
    let new_r_val = (r_val - DECAY_SPEED * time.delta_seconds() as f32).max(0.5);
    material.base_color = Color::rgb(new_r_val, 0.5, 0.5);
  }
}

#[test]
fn add_test() {
    assert_eq!(1 + 1, add(1, 1));
}