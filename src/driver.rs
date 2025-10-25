use bevy::prelude::Transform;

pub trait RigDriverTrait: std::any::Any + std::fmt::Debug + Sync + Send {
    /// Calculates the transform of this driver component based on the parent
    /// provided in `params`.
    fn update(&mut self, transform: &Transform, time_in_seconds: f32) -> Transform;
}
