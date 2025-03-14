use bevy::prelude::*;
use bevy_balls::{balls::BallsPlugin, world_plane::WorldPlanePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldPlanePlugin)
        .add_plugins(BallsPlugin)
        .run();
}
