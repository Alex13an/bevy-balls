use bevy::prelude::*;

pub struct WorldPlanePlugin;

impl Plugin for WorldPlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_world, spawn_camera));
    }
}

const TRACK_SIZE_X: f32 = 400.0;
const TRACK_SIZE_Y: f32 = 800.0;

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(TRACK_SIZE_X, TRACK_SIZE_Y))),
        MeshMaterial2d(materials.add(Color::srgb(0.12, 0.12, 0.15))),
        Transform::from_translation(Vec3::new(0., 0., 0.)),
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
