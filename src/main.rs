mod spawner;
mod cam;

use bevy::app::{App, Plugin, AppBuilder};
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};
use rand::Rng;
use spawner::*;
use cam::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(move_cubes)
        .add_system(cube_spawner)
        .add_system(moving_cam)
        .run();
}

fn move_cubes(time: Res<Time>, mut query: Query<(&mut Transform, &Handle<StandardMaterial>)>,) {
    for (mut transform, material_handle) in query.iter_mut() {
        //let material = materials.get_mut(material_handle).unwrap();
        transform.translation += Vec3::new(0.0, 0.0, 1.0) * time.delta_seconds();
        //material.albedo =
        //    Color::BLUE * Vec3::splat((3.0 * time.seconds_since_startup() as f32).sin());
    }
}

fn moving_cam(time: Res<Time>, keys: Res<Input<KeyCode>>, mut query: Query<(&mut Cam, &mut Transform)>) {
    let mut trans = (0.0, 0.0);
    let dt = time.delta_seconds();

    if keys.pressed(KeyCode::Left) {
        trans.0 -= 1.0;
    }
    if keys.pressed(KeyCode::Right) {
        trans.0 += 1.0;
    }
    if keys.pressed(KeyCode::Down) {
        trans.1 -= 1.0;
    }
    if keys.pressed(KeyCode::Up) {
        trans.1 += 1.0;
    }
    for (camera, mut transform) in query.iter_mut() {
        transform.translation += Vec3::new(trans.0 * dt, trans.1 * dt, 0.0);
    }
}

fn setup(commands: &mut Commands) {
    commands
        .spawn((Spawner::default(),))
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 4.0)),
            ..Default::default()
        })
        // cam
        .spawn((Cam::default(), )).with_bundle(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0))
            .looking_at(Vec3::default(), Vec3::unit_y()),
        ..Default::default()
    });
}