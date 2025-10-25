use bevy::prelude::*;

use crate::{
    drivers::{LookAt, Position, YawPitch},
    system::{RigChanged, RigDriver, track_rig_changes, update_camera_position},
};

pub struct CameraRigPlugin;

impl Plugin for CameraRigPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RigChanged>()
            .add_systems(
                Update,
                (
                    update_camera_position,
                    track_rig_changes.before(update_camera_position),
                ),
            )
            .register_type::<Position>()
            .register_type::<LookAt>()
            .register_type::<YawPitch>()
            .register_type::<RigDriver>();
    }
}
