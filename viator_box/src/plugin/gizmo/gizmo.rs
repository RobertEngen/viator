use bevy::prelude::*;
use bevy_mod_raycast::deferred::RaycastMesh;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct ViatorGizmos {}

pub type RaycastableGizmo = RaycastMesh<ViatorGizmos>;
