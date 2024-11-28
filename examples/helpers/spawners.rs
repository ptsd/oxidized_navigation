use bevy::{
    math::primitives,
    pbr::{DirectionalLight, MeshMaterial3d, StandardMaterial},
    prelude::*,
};

pub fn spawn_directional_light(commands: &mut Commands) {
    // Directional light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1.0, -0.5, 0.0)),
    ));
}

pub fn spawn_camera(commands: &mut Commands, transform: Option<Transform>) {
    commands.spawn((
        Camera3d::default(),
        transform.unwrap_or(
            Transform::from_xyz(10.0, 10.0, 15.0).looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
        ),
    ));
}

/// Helper function for creating a standard plane with an additional component bundle.
///
/// Size defaults to ``20.0 x 20.0``
///
/// Transform defaults to [Transform::IDENTITY]
///
/// Color defaults to ``0.5, 0.5, 0.5``
///
pub fn spawn_standard_plane<T: Bundle>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    size: Option<Vec2>,
    transform: Option<Transform>,
    color: Option<Color>,
    additional_components: T,
) -> Entity {
    let size = size.unwrap_or(Vec2::splat(20.0));
    let transform = transform.unwrap_or(Transform::IDENTITY);
    let color = color.unwrap_or(Color::srgb(0.5, 0.5, 0.5));
    spawn_mesh(
        commands,
        meshes.add(Plane3d::default().mesh().size(size.x, size.y)),
        materials.add(color),
        transform,
        additional_components,
    )
}

/// Helper function for creating a standard cuboid with an additional component bundle.
/// Size defaults to ``2.5 x 2.5 x 2.5``
/// Transform defaults to [Transform::IDENTITY]
/// Color defaults to ``0.5, 0.5, 0.5``
///
pub fn spawn_standard_cuboid<T: Bundle>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    size: Option<Vec3>,
    transform: Option<Transform>,
    color: Option<Color>,
    additional_components: T,
) -> Entity {
    let size = size.unwrap_or(Vec3::splat(2.5));
    let transform = transform.unwrap_or(Transform::IDENTITY);
    let color = color.unwrap_or(Color::srgb(0.5, 0.5, 0.5));
    spawn_mesh(
        commands,
        meshes.add(primitives::Cuboid::new(size.x, size.y, size.z)),
        materials.add(color),
        transform,
        additional_components,
    )
}

/// Helper function for creating a mesh with a standard material and an additional component bundle.
fn spawn_mesh<T: Bundle>(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    transform: Transform,
    additional_components: T,
) -> Entity {
    commands
        .spawn((Mesh3d(mesh), MeshMaterial3d(material), transform))
        .insert(additional_components)
        .id()
}
