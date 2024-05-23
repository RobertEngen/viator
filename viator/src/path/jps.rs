use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use crate::{grid::grid::PathfindingGrid, primitive::vector::Vec3I};

const JPS_EPSILON: f32 = 1e-6;

#[derive(Copy, Clone)]
struct JpsINode {
    pub position: Vec3I,
    pub cost: f32,
}

impl PartialEq for JpsINode {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && (self.cost - other.cost).abs() < JPS_EPSILON
    }
}

impl Eq for JpsINode {}

impl Ord for JpsINode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap()
    }
}

impl PartialOrd for JpsINode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn jps_i(start: &Vec3I, goal: &Vec3I, grid: &dyn PathfindingGrid) -> Option<Vec<Vec3I>> {
    let mut open_list = BinaryHeap::new();
    let mut came_from = HashMap::<Vec3I, Option<Vec3I>>::new();
    let mut cost_so_far = HashMap::<Vec3I, f32>::new();

    open_list.push(JpsINode {
        position: *start,
        cost: 0.0,
    });

    came_from.insert(*start, None);
    cost_so_far.insert(*start, 0.0);

    while let Some(current) = open_list.pop() {
        if current.position == *goal {
            return Some(jps_i_reconstruct_path(came_from, *goal));
        }

        for direction in jps_i_directions() {
            let new_position = current.position;
            let mut jump_cost = 0.0;

            loop {
                if !jps_i_is_valid_position(grid, &new_position) {
                    break;
                }

                jump_cost += 1.0; // ToDo: Non uniform jump cost

                if jps_i_is_jump_point(grid, new_position, direction) {
                    let new_cost = cost_so_far[&current.position] + jump_cost;

                    if !cost_so_far.contains_key(&new_position)
                        || new_cost < cost_so_far[&new_position]
                    {
                        cost_so_far.insert(new_position, new_cost);
                        let priority = new_cost + jps_i_heuristic(&new_position, &goal);

                        open_list.push(JpsINode {
                            position: new_position,
                            cost: priority,
                        });

                        came_from.insert(new_position, Some(current.position));
                    }
                    break;
                }
            }
        }
    }

    None
}

fn jps_i_heuristic(a: &Vec3I, b: &Vec3I) -> f32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()) as f32
}

fn jps_i_is_valid_position(grid: &dyn PathfindingGrid, position: &Vec3I) -> bool {
    grid.is_valid_path_node_i(position)
}

fn jps_i_is_obstacle(grid: &dyn PathfindingGrid, position: &Vec3I) -> bool {
    !grid.is_valid_path_node_i(position)
}

fn jps_i_directions() -> Vec<Vec3I> {
    vec![
        Vec3I { x: 1, y: 0, z: 0 },
        Vec3I { x: -1, y: 0, z: 0 },
        Vec3I { x: 0, y: 1, z: 0 },
        Vec3I { x: 0, y: -1, z: 0 },
        Vec3I { x: 0, y: 0, z: 1 },
        Vec3I { x: 0, y: 0, z: -1 },
    ]
}

fn jps_i_is_jump_point(grid: &dyn PathfindingGrid, position: Vec3I, direction: Vec3I) -> bool {
    let left = Vec3I {
        x: -direction.z,
        y: direction.y,
        z: direction.x,
    };
    let right = Vec3I {
        x: direction.z,
        y: direction.y,
        z: -direction.x,
    };

    let current = position;

    jps_i_has_forced_neighors(grid, current, direction)
        || jps_i_has_forced_neighors(grid, current, left)
        || jps_i_has_forced_neighors(grid, current, right)
}

fn jps_i_move_in_direction(position: Vec3I, direction: Vec3I) -> Vec3I {
    Vec3I {
        x: position.x + direction.x,
        y: position.y + direction.y,
        z: position.z + direction.z,
    }
}

fn jps_i_has_forced_neighors(
    grid: &dyn PathfindingGrid,
    position: Vec3I,
    direction: Vec3I,
) -> bool {
    let orthogonal_dirs = [
        Vec3I {
            x: -direction.z,
            y: 0,
            z: direction.x,
        },
        Vec3I {
            x: direction.z,
            y: 0,
            z: -direction.x,
        },
    ];

    for &ortho in orthogonal_dirs.iter() {
        let next_pos = jps_i_move_in_direction(position, direction);
        let check_pos = jps_i_move_in_direction(next_pos, ortho);

        if !jps_i_is_valid_position(grid, &check_pos) {
            continue;
        }

        if jps_i_is_obstacle(grid, &check_pos)
            && !jps_i_is_obstacle(grid, &jps_i_move_in_direction(position, ortho))
        {
            return true;
        }
    }

    false
}

fn jps_i_reconstruct_path(came_from: HashMap<Vec3I, Option<Vec3I>>, goal: Vec3I) -> Vec<Vec3I> {
    let mut current = goal;
    let mut path = vec![current];

    while let Some(&Some(prev)) = came_from.get(&current) {
        current = prev;
        path.push(current);
    }

    path.reverse();
    path
}
