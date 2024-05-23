use bevy::prelude::*;

use crate::{
    component::grid::grid_draw_component::GridDrawComponent, plugin::gizmo::gizmo::ViatorGizmos,
};

// Very poorly optimized for large grids, gizmos recompute and rerender every frame.
// ideally we should generate a mesh from the grid where contigous cells share
// edges/vertices.

pub fn draw_grid_unoptimized_system(
    mut viator_gizmos: Gizmos<ViatorGizmos>,
    query: Query<&GridDrawComponent>,
) {
    for grid_component in query.iter() {
        for transform in &grid_component.cells {
            viator_gizmos.cuboid(transform.clone(), grid_component.color);
        }
    }
}
