use crate::{
    path::path::PathfindingType,
    primitive::{
        cuboid::CuboidI,
        vector::{Vec3F, Vec3I},
    },
};

pub trait PathfindingGrid: Send + Sync {
    fn is_valid_path_node_i(&self, position: &Vec3I) -> bool;
    fn position_i(&self) -> Vec3I;
    fn position_f(&self) -> Vec3F;
    fn as_cuboid_cells(&self) -> Vec<CuboidI>;
    fn compute_path_i(
        &self,
        start: &Vec3I,
        goal: &Vec3I,
        pathfinding_engine: &PathfindingType,
    ) -> Option<Vec<Vec3I>>;
}
