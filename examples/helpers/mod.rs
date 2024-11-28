use bevy::prelude::*;
use oxidized_navigation::{colliders::OxidizedCollider, debug_draw::DrawNavMesh, NavMeshAffector};

pub mod path_finding;
pub mod spawners;

#[derive(Resource)]
pub struct DynamicAffectorSettings<T: Component + OxidizedCollider> {
  pub size: Vec3,
  pub transform: Transform,
  pub color: Color,
  pub collider: T,
}

// fn print_controls() {
//   info!("=========================================");
//   info!("| Press A to run ASYNC path finding.    |");
//   info!("| Press B to run BLOCKING path finding. |");
//   info!("| Press M to toggle drawing nav-mesh.   |");
//   info!("| Press X to spawn or despawn red cube. |");
//   info!("=========================================");
// }

pub fn toggle_dynamic_affector<T: Component + OxidizedCollider + Clone>(
  keys: Res<ButtonInput<KeyCode>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut spawned_entity: Local<Option<Entity>>,
  settings: Res<DynamicAffectorSettings<T>>
) {
  if !keys.just_pressed(KeyCode::KeyX) {
    return;
  }

  if let Some(entity) = *spawned_entity {
    commands.entity(entity).despawn_recursive();
    *spawned_entity = None;
  } else {
    let entity = spawners::spawn_standard_cuboid(
      &mut commands,
      &mut meshes,
      &mut materials,
      Some(settings.size),
      Some(settings.transform),
      Some(settings.color),
      (
        settings.collider.clone(),
        NavMeshAffector, // Only entities with a NavMeshAffector component will contribute to the nav-mesh.
      ),
    );

    *spawned_entity = Some(entity);
  }
}

pub fn toggle_nav_mesh_debug_draw(
  keys: Res<ButtonInput<KeyCode>>,
  mut show_navmesh: ResMut<DrawNavMesh>,
) {
  if keys.just_pressed(KeyCode::KeyM) {
    show_navmesh.0 = !show_navmesh.0;
  }
}