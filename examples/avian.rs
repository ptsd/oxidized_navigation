//! A simple example showing how to use oxidized_navigation with Avian3d.
//! Press M to draw nav-mesh.
//! Press X to spawn or despawn red cube.

mod helpers;

use avian3d::prelude::{Collider, PhysicsPlugins};
use bevy::prelude::*;
use oxidized_navigation::{
    debug_draw::{
        DrawNavMesh,
        OxidizedNavigationDebugDrawPlugin
    },
    NavMeshAffector,
    NavMeshSettings,
    OxidizedNavigationPlugin
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Oxidized Navigation: Avian3d".to_owned(),
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
            transform: Transform::from_xyz(5.0, 1.25, -5.0),
            color: Color::srgb(1.0, 0.1, 0.1),
            collider: Collider::cuboid(2.5, 2.5, 2.5),
        })
        .insert_resource(helpers::path_finding::PathfindingSettings::default())
        .insert_resource(helpers::path_finding::AsyncPathfindingTasks::default())
        .add_systems(Startup, setup)
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    helpers::spawners::spawn_camera(&mut commands, None);
    helpers::spawners::spawn_directional_light(&mut commands);

    // Ground plane
    helpers::spawners::spawn_standard_plane(
        &mut commands,
        &mut meshes,
        &mut materials,
        None,
        None,
        Some(Color::srgb(0.3, 0.5, 0.3)),
        (Collider::cuboid(20.0, 0.05, 20.0), NavMeshAffector),
    );

    // Cube
    helpers::spawners::spawn_standard_cuboid(
        &mut commands,
        &mut meshes,
        &mut materials,
        Some(Vec3::splat(2.5)),
        Some(Transform::from_xyz(-5.0, 1.25, -5.0)),
        Some(Color::srgb(0.4, 0.5, 0.9)),
        (Collider::cuboid(2.5, 2.5, 2.5), NavMeshAffector),
    );

    //Thin wall
    helpers::spawners::spawn_standard_cuboid(
        &mut commands,
        &mut meshes,
        &mut materials,
        Some(Vec3::new(5.0, 1.5, 0.1)),
        Some(Transform::from_xyz(-3.0, 0.75, 8.0)),
        Some(Color::srgb(0.4, 0.8, 0.9)),
        (Collider::cuboid(5.0, 1.5, 0.1), NavMeshAffector),
    );
}