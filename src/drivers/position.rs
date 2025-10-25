use crate::driver::RigDriverTrait;
use bevy::prelude::*;

/// Directly sets the position of the camera
#[derive(Debug, Default, Reflect)]
pub struct Position {
    pub position: Vec3,
}

impl Position {
    pub fn new<P>(position: P) -> Self
    where
        P: Into<Vec3>,
    {
        let position = position.into();

        Self { position }
    }

    /// Add the specified vector to the position of this component
    pub fn translate<V>(&mut self, move_vec: V)
    where
        V: Into<Vec3>,
    {
        let position: Vec3 = self.position;
        let move_vec: Vec3 = move_vec.into();
        self.position = position + move_vec;
    }
}

impl RigDriverTrait for Position {
    fn update(&mut self, transform: &Transform, _time_in_seconds: f32) -> Transform {
        Transform {
            translation: self.position,
            rotation: transform.rotation,
            scale: Vec3::ONE,
        }
    }
}
