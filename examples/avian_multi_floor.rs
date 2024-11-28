//! Nav-mesh set up example for multiple floors.
//!
//! Press A to run async path finding.
//!
//! Press B to run blocking path finding.
//!

mod helpers;

use avian3d::prelude::Collider;
use avian3d::PhysicsPlugins;
use bevy::prelude::*;
use oxidized_navigation::{
    debug_draw::{
        DrawNavMesh,
        OxidizedNavigationDebugDrawPlugin
    },
    NavMeshAffector,
    NavMeshSettings,
    OxidizedNavigationPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Oxidized Navigation: Rapier 3d Multi floor".to_owned(),
                    ..default()
                }),
                ..default()
            }),
            OxidizedNavigationPlugin::<Collider>::new(NavMeshSettings::from_agent_and_bounds(
                0.5, 1.9, 250.0, -1.0,
            )),
            OxidizedNavigationDebugDrawPlugin,
            PhysicsPlugins::default(),
        ))
        .insert_resource(DrawNavMesh(true))
        .insert_resource(helpers::DynamicAffectorSettings {
            size: Vec3::splat(2.5),
            transform: Transform::from_xyz(2.5, 1.25, -2.5),
            color: Color::srgb(1.0, 0.1, 0.1),
            collider: Collider::cuboid(2.5, 2.5, 2.5),
        })
        .insert_resource(helpers::path_finding::PathfindingSettings {
            start_pos: Vec3::new(-4.5, 1.0, -4.5),
            end_pos: Vec3::new(5.5, 2.0, 4.0),
            ..default()
        })
        .insert_resource(helpers::path_finding::AsyncPathfindingTasks::default())
        .add_systems(Startup, setup_world_system)
        .add_systems(
            Update,
            (
                helpers::toggle_nav_mesh_debug_draw,
                helpers::toggle_dynamic_affector::<Collider>,
                helpers::path_finding::run_blocking_pathfinding,
                helpers::path_finding::run_async_pathfinding,
                helpers::path_finding::poll_pathfinding_tasks_system,
            ),
        )
        .run();
}

fn setup_world_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    helpers::spawners::spawn_camera(&mut commands, None);
    helpers::spawners::spawn_directional_light(&mut commands);

    let plane_size = 10.0;

    // Lower floor
    helpers::spawners::spawn_standard_plane(
        &mut commands,
        &mut meshes,
        &mut materials,
        Some(Vec2::splat(plane_size)),
        Some(Transform::IDENTITY),
        Some(Color::srgb(0.3, 0.5, 0.3)),
        (
            Collider::cuboid(plane_size, 0.1, plane_size),
            NavMeshAffector,
        ),
    );
    // Upper floor
    helpers::spawners::spawn_standard_plane(
        &mut commands,
        &mut meshes,
        &mut materials,
        Some(Vec2::splat(plane_size)),
        Some(Transform::from_xyz(0.0, 7.0, 0.0)),
        Some(Color::srgb(0.68, 0.68, 1.0)),
        (
            Collider::cuboid(plane_size, 0.1, plane_size),
            NavMeshAffector,
        ),
    );
}
