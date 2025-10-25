use bevy::prelude::*;

use crate::driver::RigDriverTrait;

/// Offsets the camera along a vector, in the coordinate space of the parent.
#[derive(Debug, Reflect)]
pub struct Arm {
    pub offset: Vec3,
}

impl Arm {
    pub fn new<V>(offset: V) -> Self
    where
        V: Into<Vec3>,
    {
        let offset = offset.into();

        Self { offset }
    }
}

impl RigDriverTrait for Arm {
    fn update(&mut self, transform: &Transform, _time_in_seconds: f32) -> Transform {
        Transform {
            translation: transform.translation + transform.rotation * self.offset,
            rotation: transform.rotation,
            scale: Vec3::ONE,
        }
    }
}
