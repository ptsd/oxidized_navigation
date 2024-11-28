//! A simple example showing how to use oxidized_navigation with a custom component using parry3d colliders.
//! Press M to draw nav-mesh.
//! Press X to spawn or despawn red cube.

mod helpers;

use bevy::prelude::*;
use oxidized_navigation::debug_draw::DrawNavMesh;
use oxidized_navigation::{
    colliders::OxidizedCollider, debug_draw::OxidizedNavigationDebugDrawPlugin, NavMeshAffector,
    NavMeshSettings, OxidizedNavigationPlugin,
};
use parry3d::{
    bounding_volume::Aabb,
    shape::{SharedShape, TypedShape},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Oxidized Navigation: Parry3d".to_owned(),
                    ..default()
                }),
                ..default()
            }),
            OxidizedNavigationPlugin::<ParryNavmeshCollider>::new(
                NavMeshSettings::from_agent_and_bounds(0.5, 1.9, 250.0, -1.0),
            ),
            OxidizedNavigationDebugDrawPlugin,
        ))
        .insert_resource(DrawNavMesh(true))
        .insert_resource(helpers::DynamicAffectorSettings {
            size: Vec3::splat(2.5),
            transform: Transform::from_xyz(5.0, 0.8, -5.0),
            color: Color::srgb(1.0, 0.1, 0.1),
            collider: ParryNavmeshCollider {
                collider: SharedShape::cuboid(1.25, 1.25, 1.25),
            },
        })
        .insert_resource(helpers::path_finding::PathfindingSettings::default())
        .insert_resource(helpers::path_finding::AsyncPathfindingTasks::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                helpers::toggle_nav_mesh_debug_draw,
                helpers::toggle_dynamic_affector::<ParryNavmeshCollider>,
                helpers::path_finding::run_blocking_pathfinding,
                helpers::path_finding::run_async_pathfinding,
                helpers::path_finding::poll_pathfinding_tasks_system,
            ),
        )
        .run();
}

#[derive(Component, Clone)]
struct ParryNavmeshCollider {
    collider: SharedShape,
}

impl OxidizedCollider for ParryNavmeshCollider {
    fn oxidized_into_typed_shape(&self) -> TypedShape {
        self.collider.as_typed_shape()
    }

    fn oxidized_compute_local_aabb(&self) -> Aabb {
        self.collider.compute_local_aabb()
    }
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
        (
            ParryNavmeshCollider {
                collider: SharedShape::cuboid(10.0, 0.1, 10.0),
            },
            NavMeshAffector,
        ),
    );

    // Cube
    helpers::spawners::spawn_standard_cuboid(
        &mut commands,
        &mut meshes,
        &mut materials,
        Some(Vec3::splat(2.5)),
        Some(Transform::from_xyz(-5.0, 1.25, -5.0)),
        Some(Color::srgb(0.4, 0.5, 0.9)),
        (
            ParryNavmeshCollider {
                collider: SharedShape::cuboid(1.25, 1.25, 1.25),
            },
            NavMeshAffector,
        ),
    );

    // Thin wall
    helpers::spawners::spawn_standard_cuboid(
        &mut commands,
        &mut meshes,
        &mut materials,
        Some(Vec3::new(5.0, 1.5, 0.1)),
        Some(Transform::from_xyz(-3.0, 0.75, 8.0)),
        Some(Color::srgb(0.4, 0.8, 0.9)),
        (
            ParryNavmeshCollider {
                collider: SharedShape::cuboid(2.5, 0.75, 0.05),
            },
            NavMeshAffector,
        ),
    );
}
