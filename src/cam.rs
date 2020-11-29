use bevy::prelude::Vec3;

pub struct Cam {
    pub transform: Vec3,
}

impl Default for Cam {
    fn default() -> Cam {
        Cam {
            transform: Vec3::new(0.0, 0.0, 5.0),
        }
    }
}