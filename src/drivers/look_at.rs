use crate::{
    driver::RigDriverTrait,
    util::{ExpSmoothed, ExpSmoothingParams},
};
use bevy::prelude::*;

/// Rotates the camera to point at a world-space position.
///
/// The target tracking can be additionally smoothed, and made to look ahead of it.
#[derive(Debug, Reflect)]
pub struct LookAt {
    /// Exponential smoothing factor
    pub smoothness: f32,

    /// The world-space position to look at
    pub target: Vec3,

    // The scale with which smoothing should be applied to the target position
    output_offset_scale: f32,

    smoothed_target: ExpSmoothed<Vec3>,
}

impl LookAt {
    pub fn new<P>(target: P) -> Self
    where
        P: Into<Vec3>,
    {
        let target = target.into();

        Self {
            smoothness: 0.0,
            output_offset_scale: 1.0,
            target,
            smoothed_target: Default::default(),
        }
    }

    /// Set the exponential smoothing factor for target position tracking.
    pub fn tracking_smoothness(mut self, smoothness: f32) -> Self {
        self.smoothness = smoothness;
        self
    }

    /// Reverse target position smoothing, causing the camera to look ahead of it.
    /// This can then be chained with [`Smooth`], to create
    /// a camera that smoothly follows an object, but doesn't lag far behind it.
    ///
    /// [`Smooth`]: struct.Smooth.html
    pub fn tracking_predictive(mut self, predictive: bool) -> Self {
        self.output_offset_scale = if predictive { -1.0 } else { 1.0 };
        self
    }
}

impl RigDriverTrait for LookAt {
    fn update(&mut self, transform: &Transform, delta_time_seconds: f32) -> Transform {
        let target = self.smoothed_target.exp_smooth_towards(
            &self.target,
            ExpSmoothingParams {
                smoothness: self.smoothness,
                output_offset_scale: self.output_offset_scale,
                delta_time_seconds,
            },
        );

        let rotation = transform.looking_at(target, Vec3::Y).rotation;

        Transform {
            translation: transform.translation,
            rotation,
            ..Default::default()
        }
    }
}
