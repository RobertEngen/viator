use crate::path::jps::jps_i;
use std::collections::HashSet;

use crate::{
    grid::grid::PathfindingGrid,
    primitive::{
        alias::{TransformFloat, TransformInt},
        cuboid::CuboidI,
        vector::{Vec3F, Vec3I},
    },
};

// Example statically sized grid implementation that supports any size up to the max value of
// TransformInt. Implemented with a macro to allow for easily defining different capacities.

// ToDo: Add support for negative coordinates, currently grid will crash if operations are
//       performed on negative coordinates

macro_rules! create_voxel_grid_type {
    ($type_name:ident, $max_capacity:expr) => {
        #[derive(Debug)]
        pub struct $type_name<T: Copy + Clone + Default + Send + Sync> {
            width: usize,
            height: usize,
            depth: usize,
            grid: [T; $max_capacity],
            occupied: HashSet<usize>,
            position: Vec3I,
        }

        impl<T: Copy + Clone + Default + Send + Sync> $type_name<T> {
            pub fn new(position: &Vec3I, extents: &Vec3I) -> Self {
                let width = extents.x as usize;
                let height = extents.y as usize;
                let depth = extents.z as usize;

                Self {
                    width,
                    height,
                    depth,
                    grid: [T::default(); $max_capacity],
                    occupied: HashSet::new(),
                    position: position.clone(),
                }
            }

            pub fn bounds_as_cuboid(&self) -> CuboidI {
                CuboidI {
                    position: self.position.clone(),
                    extents: Vec3I {
                        x: self.width as TransformInt,
                        y: self.height as TransformInt,
                        z: self.depth as TransformInt,
                    },
                }
            }

            pub fn create_cell(
                &mut self,
                x: TransformInt,
                y: TransformInt,
                z: TransformInt,
                value: &T,
            ) -> bool {
                let index = self.index_1d(x, y, z);
                if index >= $max_capacity {
                    return false;
                }
                if !self.occupied.contains(&index) {
                    self.grid[index] = value.clone();
                    self.occupied.insert(index);
                    true
                } else {
                    false
                }
            }

            pub fn read_cell(
                &self,
                x: TransformInt,
                y: TransformInt,
                z: TransformInt,
            ) -> Option<&T> {
                let index = self.index_1d(x, y, z);
                if index >= $max_capacity {
                    return None;
                }
                if self.occupied.contains(&index) {
                    Some(&self.grid[index])
                } else {
                    None
                }
            }

            pub fn read_cell_cloned(
                &self,
                x: TransformInt,
                y: TransformInt,
                z: TransformInt,
            ) -> Option<T> {
                self.read_cell(x, y, z).cloned()
            }

            pub fn update_cell(
                &mut self,
                x: TransformInt,
                y: TransformInt,
                z: TransformInt,
                value: &T,
            ) -> bool {
                let index = self.index_1d(x, y, z);
                if index >= $max_capacity {
                    return false;
                }
                if self.occupied.contains(&index) {
                    self.grid[index] = value.clone();
                    true
                } else {
                    false
                }
            }

            pub fn upsert_cell(
                &mut self,
                x: TransformInt,
                y: TransformInt,
                z: TransformInt,
                value: &T,
            ) {
                let index = self.index_1d(x, y, z);
                if index >= $max_capacity {
                    return;
                }
                self.grid[index] = value.clone();
                self.occupied.insert(index);
            }

            pub fn delete_cell(&mut self, x: TransformInt, y: TransformInt, z: TransformInt) {
                let index = self.index_1d(x, y, z);
                if index >= $max_capacity {
                    return;
                }
                if self.occupied.remove(&index) {
                    self.grid[index] = T::default();
                }
            }

            pub fn delete_cell_check(
                &mut self,
                x: TransformInt,
                y: TransformInt,
                z: TransformInt,
            ) -> bool {
                let index = self.index_1d(x, y, z);
                if index >= $max_capacity {
                    return false;
                }
                if self.occupied.remove(&index) {
                    self.grid[index] = T::default();
                    true
                } else {
                    false
                }
            }

            pub fn delete_cell_pop(
                &mut self,
                x: TransformInt,
                y: TransformInt,
                z: TransformInt,
            ) -> Option<T> {
                let index = self.index_1d(x, y, z);
                if index >= $max_capacity {
                    return None;
                }
                if self.occupied.remove(&index) {
                    Some(std::mem::replace(&mut self.grid[index], T::default()))
                } else {
                    None
                }
            }

            fn index_1d(&self, x: TransformInt, y: TransformInt, z: TransformInt) -> usize {
                let self_pos = self.position;

                #[cfg(feature = "y_up")]
                {
                    // X, Z, Y
                    ((x + self_pos.x)
                        + ((z + self_pos.z) * self.width as TransformInt)
                        + ((y + self_pos.y)
                            * self.width as TransformInt
                            * self.depth as TransformInt)) as usize
                }

                #[cfg(feature = "z_up")]
                {
                    // X, Y, Z
                    ((x + self_pos.x)
                        + ((y + self_pos.y) * self.width as TransformInt)
                        + ((z + self_pos.z)
                            * self.width as TransformInt
                            * self.depth as TransformInt)) as usize
                }
            }

            fn index_3d(&self, index: usize) -> Vec3I {
                let index = index as TransformInt;
                let width = self.width as TransformInt;
                let depth = self.depth as TransformInt;

                let x: TransformInt;
                let y: TransformInt;
                let z: TransformInt;

                #[cfg(feature = "y_up")]
                {
                    x = index % width;
                    z = (index / width) % depth;
                    y = index / (width * depth);
                }

                #[cfg(feature = "z_up")]
                {
                    x = index % width;
                    y = (index / width) % depth;
                    z = index / (width * depth);
                }

                Vec3I { x, y, z }
            }

            fn as_cuboid_cells(&self) -> Vec<CuboidI> {
                let mut cells = Vec::with_capacity(self.grid.len());

                for (idx, _) in self.grid.iter().enumerate() {
                    if !self.occupied.contains(&idx) {
                        continue;
                    }

                    let cell = CuboidI {
                        position: self.index_3d(idx),
                        extents: Vec3I { x: 1, y: 1, z: 1 },
                    };
                    cells.push(cell);
                }

                cells
            }

            fn is_valid_path_node_i(&self, position: &Vec3I) -> bool {
                let index = self.index_1d(position.x, position.y, position.z);
                self.occupied.contains(&index)
            }
        }

        impl<T: Copy + Clone + Default + Send + Sync> PathfindingGrid for $type_name<T> {
            fn is_valid_path_node_i(&self, position: &Vec3I) -> bool {
                self.is_valid_path_node_i(position)
            }

            fn position_i(&self) -> Vec3I {
                self.position
            }

            fn position_f(&self) -> crate::primitive::vector::Vec3F {
                let pos_i = self.position.clone();

                Vec3F {
                    x: pos_i.x as TransformFloat,
                    y: pos_i.y as TransformFloat,
                    z: pos_i.z as TransformFloat,
                }
            }

            fn as_cuboid_cells(&self) -> Vec<CuboidI> {
                self.as_cuboid_cells()
            }

            fn compute_path_i(
                &self,
                start: &Vec3I,
                goal: &Vec3I,
                pathfinding_engine: &crate::path::path::PathfindingType,
            ) -> Option<Vec<Vec3I>> {
                match pathfinding_engine {
                    crate::path::path::PathfindingType::Jps => jps_i(start, goal, self),
                }
            }
        }
    };
}

create_voxel_grid_type!(StaticSizedVoxelGridSmall, 10_000);
create_voxel_grid_type!(StaticSizedVoxelGridMedium, 50_000);
create_voxel_grid_type!(StaticSizedVoxelGridLarge, 100_000);
create_voxel_grid_type!(StaticSizedVoxelGridXL, 1_000_000);
create_voxel_grid_type!(StaticSizedVoxelGridXXL, 100_000_000);
