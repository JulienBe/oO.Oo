use bevy::prelude::Vec3;

pub struct MyLight {
    pub transform: Vec3,
}

impl Default for MyLight {
    fn default() -> MyLight {
        MyLight {
            transform: Vec3::new(0.0, 0.0, 5.0),
        }
    }
}