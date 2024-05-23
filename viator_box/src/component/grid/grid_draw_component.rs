use bevy::prelude::*;
use viator::grid::grid::PathfindingGrid;

#[derive(Component)]
pub struct GridDrawComponent {
    pub grid: Box<dyn PathfindingGrid>,
    pub cells: Vec<Transform>,
    pub color: Color,
}

impl GridDrawComponent {
    pub fn new(grid: Box<dyn PathfindingGrid>, color: Color) -> Self {
        let mut cells = Vec::new();

        for cell in grid.as_cuboid_cells() {
            let cell_position = cell.position;
            let pos_x = cell_position.x as f32;
            let pos_y = cell_position.y as f32;
            let pos_z = cell_position.z as f32;

            let cell_scale = cell.extents;
            let scale_x = cell_scale.x as f32;
            let scale_y = cell_scale.y as f32;
            let scale_z = cell_scale.z as f32;

            let mut cell_transform = Transform::from_translation(Vec3::new(pos_x, pos_y, pos_z));
            cell_transform.scale = Vec3::new(scale_x, scale_y, scale_z);

            cells.push(cell_transform);
        }

        Self { grid, cells, color }
    }
}
