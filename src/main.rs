use std::collections::HashMap;
use bevy::DefaultPlugins;
use bevy::prelude::*;

const SPEED: f32 = 5.0;

#[derive(Component)]
struct Player;

enum Action {
    LEFT,
    RIGHT,
    UP,
    DOWN,
    NONE,
}

struct Bindings {
    key_to_action: HashMap<KeyCode, Action>,
}

fn main() {
    let mut key_to_action = HashMap::new();
    key_to_action.insert(KeyCode::W, Action::UP);
    key_to_action.insert(KeyCode::Z, Action::UP);
    key_to_action.insert(KeyCode::Q, Action::LEFT);
    key_to_action.insert(KeyCode::A, Action::LEFT);
    key_to_action.insert(KeyCode::D, Action::RIGHT);
    key_to_action.insert(KeyCode::S, Action::DOWN);
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(Bindings {
            key_to_action
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(add_player)
        .add_system(move_player)
        .run();
}

fn move_player(time: Res<Time>, bindings: Res<Bindings>, keys: Res<Input<KeyCode>>, mut query: Query<(&mut Transform, With<Player>)>) {
    bindings.key_to_action.iter()
        .filter(|(k, _)| { keys.pressed(**k) })
        .for_each(|(_, action)| {
            // Excepting only one player for the moment, but hey, we'll see. Maybe controlling multiple blocs could be fun
            for (mut transform, _) in query.iter_mut() {
                let mut mvt = Vec3::new(0.0, 0.0, 0.0);
                match action {
                    Action::LEFT => mvt.x -= 1.0,
                    Action::RIGHT => mvt.x += 1.0,
                    Action::UP => mvt.y += 1.0,
                    Action::DOWN => mvt.y -= 1.0,
                    Action::NONE => {}
                };
                mvt.normalize();
                mvt *= SPEED * time.delta_seconds();
                transform.translation += mvt;
            }
        });
}

fn add_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(1.5, 0.5, 1.5),
            ..default()
        })
        .insert(Player);
}

fn setup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 3.0;
    camera.transform = Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn_bundle(camera);
}