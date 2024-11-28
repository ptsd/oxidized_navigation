use bevy::prelude::*;
use bevy::tasks;
use bevy::log::{error, info};
use bevy::color::palettes;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy::math::Vec3;
use bevy::input::ButtonInput;
use oxidized_navigation::debug_draw::DrawPath;
use oxidized_navigation::{NavMesh, NavMeshSettings};
use oxidized_navigation::query::{find_path, find_polygon_path, perform_string_pulling_on_path};
use std::sync::{Arc, RwLock};
use oxidized_navigation::tiles::NavMeshTiles;

//  Running pathfinding in a task without blocking the frame.
//  Also check out Bevy's async compute example.
//  https://github.com/bevyengine/bevy/blob/main/examples/async_tasks/async_compute.rs

#[derive(Resource)]
pub struct PathfindingSettings {
    pub start_pos: Vec3,
    pub end_pos: Vec3,
    pub position_search_radius: Option<f32>,
    pub area_cost_multipliers: Option<&'static[f32]>,
    pub path_display_time_secs: f32,
}

impl Default for PathfindingSettings {
    fn default() -> Self {
        Self {
            start_pos: Vec3::new(5.0, 1.0, 5.0),
            end_pos: Vec3::new(-15.0, 1.0, -15.0),
            position_search_radius: None,
            area_cost_multipliers: None, //Some(&[1.0,0.5]),
            path_display_time_secs: 4.0,
        }
    }
}

#[allow(unused)]
pub fn run_blocking_pathfinding(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    nav_mesh_settings: Res<NavMeshSettings>,
    nav_mesh: Res<NavMesh>,
    settings: Res<PathfindingSettings>,
) {
    if !keys.just_pressed(KeyCode::KeyB) {
        return;
    }

    // Get the underlying nav_mesh.
    if let Ok(nav_mesh) = nav_mesh.get().read() {
        let start_pos = settings.start_pos;
        let end_pos = settings.end_pos;

        // Run pathfinding to get a polygon path.
        match find_polygon_path(
            &nav_mesh,
            &nav_mesh_settings,
            start_pos,
            end_pos,
            settings.position_search_radius,
            settings.area_cost_multipliers,//Some(&[1.0, 0.5]),
        ) {
            Ok(path) => {
                info!("Path found (BLOCKING): {:?}", path);

                // Convert polygon path to a path of Vec3s.
                match perform_string_pulling_on_path(&nav_mesh, start_pos, end_pos, &path) {
                    Ok(string_path) => {
                        info!("String path (BLOCKING): {:?}", string_path);
                        commands.spawn(DrawPath {
                            timer: Some(Timer::from_seconds(settings.path_display_time_secs, TimerMode::Once)),
                            pulled_path: string_path,
                            color: palettes::css::RED.into(),
                        });
                    }
                    Err(error) => error!("Error with string path: {:?}", error),
                };
            }
            Err(error) => error!("Error with pathfinding: {:?}", error),
        }
    }
}

// Holder resource for tasks.
#[derive(Default, Resource)]
pub struct AsyncPathfindingTasks {
    tasks: Vec<Task<Option<Vec<Vec3>>>>,
}

// Queue up pathfinding tasks.
#[allow(unused)]
pub fn run_async_pathfinding(
    keys: Res<ButtonInput<KeyCode>>,
    nav_mesh_settings: Res<NavMeshSettings>,
    nav_mesh: Res<NavMesh>,
    mut pathfinding_task: ResMut<AsyncPathfindingTasks>,
    settings: Res<PathfindingSettings>,
) {
    if !keys.just_pressed(KeyCode::KeyA) {
        return;
    }

    let thread_pool = AsyncComputeTaskPool::get();

    let nav_mesh_lock = nav_mesh.get();

    let task = thread_pool.spawn(async_path_find(
        nav_mesh_lock,
        nav_mesh_settings.clone(),
        settings.start_pos,
        settings.end_pos,
        None,
        settings.area_cost_multipliers
    ));

    pathfinding_task.tasks.push(task);
}

// Poll existing tasks.
#[allow(unused)]
pub fn poll_pathfinding_tasks_system(
    mut commands: Commands,
    mut pathfinding_task: ResMut<AsyncPathfindingTasks>,
    settings: Res<PathfindingSettings>,
) {
    // Go through and remove completed tasks.
    pathfinding_task.tasks.retain_mut(|task| {
        if let Some(string_path) = tasks::block_on(tasks::poll_once(task)).unwrap_or(None) {
            info!("Async path task finished with result: {:?}", string_path);
            commands.spawn(DrawPath {
                timer: Some(Timer::from_seconds(settings.path_display_time_secs, TimerMode::Once)),
                pulled_path: string_path,
                color: palettes::basic::BLUE.into(),
            });

            false
        } else {
            true
        }
    });
}

/// Async wrapper function for path finding.
#[allow(unused)]
async fn async_path_find(
    nav_mesh_lock: Arc<RwLock<NavMeshTiles>>,
    nav_mesh_settings: NavMeshSettings,
    start_pos: Vec3,
    end_pos: Vec3,
    position_search_radius: Option<f32>,
    area_cost_multipliers: Option<&'static[f32]>,
) -> Option<Vec<Vec3>> {
    // Get the underlying nav_mesh.
    let Ok(nav_mesh) = nav_mesh_lock.read() else {
        return None;
    };

    // Run pathfinding to get a path.
    match find_path(
        &nav_mesh,
        &nav_mesh_settings,
        start_pos,
        end_pos,
        position_search_radius,
        area_cost_multipliers,
    ) {
        Ok(path) => {
            info!("Found path (ASYNC): {:?}", path);
            return Some(path);
        }
        Err(error) => error!("Error with pathfinding: {:?}", error),
    }

    None
}