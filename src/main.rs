use bevy::app::{App, Plugin, AppBuilder};
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};
use rand::Rng;

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

fn cube_spawner(time: Res<Time>, cmds: &mut Commands, mut meshes: ResMut<Assets<Mesh>>, mut mat: ResMut<Assets<StandardMaterial>>, mut query: Query<(&mut Spawner)>) {
    for mut spawn in query.iter_mut() {
        if time.time_since_startup().as_secs_f32() > spawn.next_spawn {
            spawn.next_spawn = time.time_since_startup().as_secs_f32() + 2.0;
            let mut rng = rand::thread_rng();
            cmds.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: mat.add(Color::rgb(0.5, 0.7, 0.6).into()),
                transform: Transform::from_translation(Vec3::new(rng.gen_range(-5.0, 5.0), 0.0, 0.0)),
                ..Default::default()
            });
        }
    }
}

struct Spawner {
    next_spawn: f32
}
impl Default for Spawner {
    fn default() -> Self {
        Spawner {
            next_spawn: 0.0,
        }
    }
}


struct Cam {
    pub transform: Vec3,
}

impl Default for Cam {
    fn default() -> Cam {
        Cam {
            transform: Vec3::new(0.0, 0.0, 5.0),
        }
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
    for (mut camera, mut transform) in query.iter_mut() {
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