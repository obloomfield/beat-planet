use bevy::prelude::*;
// use wasm_bindgen::prelude::wasm_bindgen;

// #[wasm_bindgen]
pub fn main() {
  // #[cfg(debug_assertions)]
	// console_error_panic_hook::set_once();

  App::new()
    .add_plugins(DefaultPlugins)
    // .add_plugins(
    //     DefaultPlugins.build()
    //         .disable::<bevy::window::WindowPlugin>()
    // )
    // .add_plugins(WindowPlugin {
    //   primary_window: Some(Window {
    //     resolution: (500.0, 500.0).into(),
    //     title: "Onion Engine".to_string(),
    //     canvas: Some("#game-canvas".into()),
    //     ..default()
    //   }),
    //   ..default()
    // })
    .add_systems(Startup, setup)
    .run();
}


fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // circular base
  commands.spawn(PbrBundle {
      mesh: meshes.add(Circle::new(4.0)),
      material: materials.add(Color::WHITE),
      transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
      ..default()
  });
  // cube
  commands.spawn(PbrBundle {
      mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
      material: materials.add(Color::rgb_u8(124, 144, 255)),
      transform: Transform::from_xyz(0.0, 0.5, 0.0),
      ..default()
  });
  // light
  commands.spawn(PointLightBundle {
      point_light: PointLight {
          shadows_enabled: true,
          ..default()
      },
      transform: Transform::from_xyz(4.0, 8.0, 4.0),
      ..default()
  });
  // camera
  commands.spawn(Camera3dBundle {
      transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
      ..default()
  });
}