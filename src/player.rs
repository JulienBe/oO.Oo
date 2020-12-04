use bevy::prelude::{Commands, LightBundle, Transform, Vec3, Camera3dBundle, Res, Time, Input};
use crate::spawner::Spawner;
use crate::cam::Cam;
use crate::mylight::*;
use bevy::input::prelude::KeyCode;
use bevy::ecs::Query;

pub(crate) fn setup(commands: &mut Commands) {
    commands
        .spawn((Spawner::default(),))
        // light
        .spawn((MyLight::default(), )).with_bundle(LightBundle {
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

pub(crate) fn move_player(time: Res<Time>, keys: Res<Input<KeyCode>>, mut query: Query<(&mut Cam, &mut Transform)>) {
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