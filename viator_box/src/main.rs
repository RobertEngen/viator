use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, log, prelude::*};
use bevy_console::ConsolePlugin;
use bevy_mod_raycast::deferred::{DeferredRaycastingPlugin, RaycastPluginState, RaycastSource};
use bevy_obj::ObjPlugin;
use component::{
    camera::fly_camera_component::FlyCameraComponent, grid::grid_draw_component::GridDrawComponent,
    ui::fps_text_component::FpsTextComponent,
};
use plugin::{gizmo::gizmo::ViatorGizmos, material::line_material::LineMaterial};
use system::{
    camera::fly_camera_system::fly_camera_system,
    grid::draw_grid_unoptimized_system::draw_grid_unoptimized_system,
    ui::fps_text_system::fps_text_system,
};
use viator::{
    grid::grid::PathfindingGrid,
    grid::voxel::{
        dynamic_sized_voxel_grid::DynamicSizedVoxelGrid,
        static_sized_voxel_grid::StaticSizedVoxelGridLarge,
    },
    path::path::PathfindingType,
    primitive::{cell::Cell, vector::Vec3I},
};

pub mod component;
pub mod plugin;
pub mod system;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<LineMaterial>::default(),
            FrameTimeDiagnosticsPlugin,
            ConsolePlugin,
            ObjPlugin,
        ))
        .add_plugins(DeferredRaycastingPlugin::<ViatorGizmos>::default())
        .insert_resource(RaycastPluginState::<ViatorGizmos>::default())
        .init_gizmo_group::<ViatorGizmos>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                fly_camera_system,
                fps_text_system,
                draw_grid_unoptimized_system,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn Editor Camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.5, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(FlyCameraComponent {
            speed: 30.0,
            sensitivity: 0.5,
            pitch: 0.0,
            yaw: 0.0,
        })
        .insert(RaycastSource::<ViatorGizmos>::new_cursor());
    // Spawn Editor Camera

    // Spawn FPS Counter
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    ..default()
                },
            ),
            TextSection::from_style(if cfg!(feature = "default_font") {
                TextStyle {
                    font_size: 60.0,
                    color: Color::GOLD,
                    ..default()
                }
            } else {
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 60.0,
                    color: Color::GOLD,
                }
            }),
        ]),
        FpsTextComponent,
    ));
    // Spawn FPS Counter

    // Spawn Example Dynamic Sized Voxel Grid
    let grid_pos = Vec3I { x: 0, y: 0, z: 0 };
    let grid_size = Vec3I {
        x: 10,
        y: 10,
        z: 10,
    };
    let mut grid_boxed = Box::new(DynamicSizedVoxelGrid::<Cell>::new(&grid_pos, &grid_size));
    grid_boxed.create_cell(0, 0, 0, &Cell::default());
    grid_boxed.create_cell(1, 0, 0, &Cell::default());
    grid_boxed.create_cell(0, 1, 0, &Cell::default());
    grid_boxed.create_cell(0, 2, 0, &Cell::default());
    grid_boxed.create_cell(0, 3, 0, &Cell::default());
    grid_boxed.create_cell(0, 0, 1, &Cell::default());

    let draw_grid_component = GridDrawComponent::new(grid_boxed, Color::DARK_GRAY);
    commands.spawn_empty().insert(draw_grid_component);
    // Spawn Example Dynamic Sized Grid

    // Spawn Example Static Sized Voxel Grid
    let grid_pos = Vec3I { x: 2, y: 0, z: 2 };
    let grid_size = Vec3I {
        x: 10,
        y: 10,
        z: 10,
    };
    let mut grid_boxed = Box::new(StaticSizedVoxelGridLarge::<Cell>::new(
        &grid_pos, &grid_size,
    ));
    grid_boxed.create_cell(0, 0, 0, &Cell::default());
    grid_boxed.create_cell(1, 0, 0, &Cell::default());
    grid_boxed.create_cell(0, 1, 0, &Cell::default());
    grid_boxed.create_cell(0, 2, 0, &Cell::default());
    grid_boxed.create_cell(0, 3, 0, &Cell::default());
    grid_boxed.create_cell(0, 0, 1, &Cell::default());

    let draw_grid_component = GridDrawComponent::new(grid_boxed, Color::GOLD);
    commands.spawn_empty().insert(draw_grid_component);
    // Spawn Example Static Sized Voxel Grid
}
