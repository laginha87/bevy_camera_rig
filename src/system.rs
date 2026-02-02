use std::collections::HashSet;

use crate::prelude::*;
use bevy::prelude::*;

#[derive(Message)]
pub struct RigChanged(Entity);

#[derive(Component, Reflect)]
pub enum RigDriver {
    Position(Position),
    Rotation(Rotation),
    LookAt(LookAt),
    LockPosition(LockPosition),
    Arm(Arm),
    YawPitch(YawPitch),
    Smooth(Smooth),
}

impl RigDriver {
    pub fn update(&mut self, transform: &Transform, delta_time_in_seconds: f32) -> Transform {
        match self {
            RigDriver::Position(position) => position.update(transform, delta_time_in_seconds),
            RigDriver::Rotation(rotation) => rotation.update(transform, delta_time_in_seconds),
            RigDriver::LookAt(look_at) => look_at.update(transform, delta_time_in_seconds),
            RigDriver::LockPosition(lock_position) => {
                lock_position.update(transform, delta_time_in_seconds)
            }
            RigDriver::Arm(arm) => arm.update(transform, delta_time_in_seconds),
            RigDriver::YawPitch(yaw_pitch) => yaw_pitch.update(transform, delta_time_in_seconds),
            RigDriver::Smooth(smooth) => smooth.update(transform, delta_time_in_seconds),
        }
    }
}

pub fn update_camera_position(
    mut cameras: Query<(&mut Transform, &Children)>,
    mut drivers: Query<&mut RigDriver>,
    time: Res<Time>,
    mut camera_changed_event: MessageReader<RigChanged>,
) {
    if camera_changed_event.is_empty() {
        return;
    }
    let mut visited = HashSet::<Entity>::new();

    let delta_time_seconds = time.delta_secs();
    for RigChanged(camera) in camera_changed_event.read() {
        if visited.contains(camera) {
            continue;
        };
        visited.insert(*camera);

        let (mut transform, camera_drivers) = cameras.get_mut(*camera).unwrap();
        let mut parent_transform = Transform::IDENTITY;
        for driver in camera_drivers {
            if let Ok(mut driver) = drivers.get_mut(*driver) {
                parent_transform = driver.update(&parent_transform, delta_time_seconds);
            }
        }
        *transform = parent_transform;
    }
}

pub fn track_rig_changes(
    drivers: Query<&ChildOf, Changed<RigDriver>>,
    mut camera_changed_event: MessageWriter<RigChanged>,
) {
    for child_of in drivers {
        camera_changed_event.write(RigChanged(child_of.parent()));
    }
}
