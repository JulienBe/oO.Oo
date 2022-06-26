use std::collections::HashMap;
use bevy::DefaultPlugins;
use bevy::prelude::*;

const SCREEN_WIDTH: f32 = 40.0;
const SCREEN_HEIGHT: f32 = 20.0;
const HALF_SCREEN_WIDTH: f32 = SCREEN_WIDTH / 2.0;
const HALF_SCREEN_HEIGHT: f32 = SCREEN_HEIGHT / 2.0;
const SPEED: f32 = 15.0;
const PLAYER_SIZE: f32 = 1.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct LogicalSize {
    w: f32,
    h: f32,
    hw: f32,
    hh: f32,
}

impl LogicalSize {
    fn new(dim: f32) -> Self {
        Self { w: dim, h: dim, hw: dim / 2.0, hh: dim /2.0 }
    }
}

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
        .add_system(constrain_entities)
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
                    Action::LEFT => mvt.x -= SPEED,
                    Action::RIGHT => mvt.x += SPEED,
                    Action::UP => mvt.y += SPEED,
                    Action::DOWN => mvt.y -= SPEED,
                    Action::NONE => {}
                };
                mvt.normalize();
                mvt *= time.delta_seconds();
                transform.translation += mvt;
            }
        });
}

fn constrain_entities(mut query: Query<(&mut Transform, &LogicalSize)>) {
    for (mut transform, size) in query.iter_mut() {
        transform.translation.x = transform.translation.x.max(-HALF_SCREEN_WIDTH + size.w).min(HALF_SCREEN_WIDTH + size.w);
        transform.translation.y = transform.translation.y.max(-HALF_SCREEN_HEIGHT + size.hh).min(HALF_SCREEN_HEIGHT - size.hh);
    }
}

fn add_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: PLAYER_SIZE })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(1.5, 0.5, 1.5),
            ..default()
        })
        .insert(LogicalSize::new(PLAYER_SIZE))
        .insert(Player);
}

fn setup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 10.0;
    camera.transform = Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn_bundle(camera);
}