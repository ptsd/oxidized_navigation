//! Nav-mesh set up example for both blocking & async pathfinding with a heightfield.
//!
//! Press A to run async path finding.
//!
//! Press B to run blocking path finding.
//!

mod helpers;

use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, NoUserData, RapierPhysicsPlugin};
use oxidized_navigation::{
    debug_draw::{DrawNavMesh, OxidizedNavigationDebugDrawPlugin},
    NavMeshAffector, NavMeshSettings, OxidizedNavigationPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Oxidized Navigation: Rapier 3d Heightfield".to_owned(),
                    ..default()
                }),
                ..default()
            }),
            OxidizedNavigationPlugin::<Collider>::new(NavMeshSettings::from_agent_and_bounds(
                0.5, 1.9, 250.0, -1.0,
            )),
            OxidizedNavigationDebugDrawPlugin,
            // The rapier plugin needs to be added for the scales of colliders to be correct if the scale of the entity is not uniformly 1.
            // An example of this is the "Thin Wall" in [setup_world_system]. If you remove this plugin, it will not appear correctly.
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .insert_resource(DrawNavMesh(true))
        .insert_resource(helpers::DynamicAffectorSettings {
            size: Vec3::splat(2.5),
            transform: Transform::from_xyz(5.0, 1.25, -5.0),
            color: Color::srgb(1.0, 0.1, 0.1),
            collider: Collider::cuboid(1.25, 1.25, 1.25),
        })
        .insert_resource(helpers::path_finding::PathfindingSettings {
            start_pos: Vec3::new(5.0, 1.0, 5.0),
            end_pos: Vec3::new(-15.0, 1.0, -15.0),
            ..default()
        })
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

fn setup(mut commands: Commands) {
    helpers::spawners::spawn_camera(
        &mut commands,
        Some(Transform::from_xyz(60.0, 50.0, 50.0).looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y)),
    );
    helpers::spawners::spawn_directional_light(&mut commands);

    let heightfield_heights = (0..(50 * 50))
        .map(|value| {
            let position = value / 50;

            (position as f32 / 10.0).sin() / 10.0
        })
        .collect();

    // Heightfield.
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Collider::heightfield(heightfield_heights, 50, 50, Vec3::new(50.0, 50.0, 50.0)),
        NavMeshAffector,
    ));
}
