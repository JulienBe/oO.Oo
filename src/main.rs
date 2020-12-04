mod spawner;
mod cam;
mod player;
mod mylight;

use bevy::app::{App, Plugin, AppBuilder};
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};
use rand::Rng;
use spawner::*;
use player::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(move_cubes)
        .add_system(cube_spawner)
        .add_system(move_player)
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