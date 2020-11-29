use bevy::ecs::{Res, Query};
use bevy::prelude::{Time, Commands, ResMut, Assets, Mesh, StandardMaterial, PbrBundle, shape, Color, Transform, Vec3};
use rand::Rng;

pub struct Spawner {
    next_spawn: f32
}
impl Default for Spawner {
    fn default() -> Self {
        Spawner {
            next_spawn: 0.0,
        }
    }
}

pub fn cube_spawner(time: Res<Time>, cmds: &mut Commands, mut meshes: ResMut<Assets<Mesh>>, mut mat: ResMut<Assets<StandardMaterial>>, mut query: Query<(&mut Spawner)>) {
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

