use crate::driver::RigDriverTrait;
use bevy::prelude::*;

/// Directly sets the rotation of the camera
#[derive(Debug, Default, Reflect)]
pub struct Rotation {
    pub rotation: Quat,
}

impl Rotation {
    pub fn new<Q>(rotation: Q) -> Self
    where
        Q: Into<Quat>,
    {
        let rotation = rotation.into();

        Self { rotation }
    }
}

impl RigDriverTrait for Rotation {
    fn update(&mut self, transform: &Transform, _time_in_seconds: f32) -> Transform {
        Transform {
            translation: transform.translation,
            rotation: self.rotation,
            scale: Vec3::ONE,
        }
    }
}
