use bevy::prelude::*;

pub struct BallsPlugin;

impl Plugin for BallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_balls)
            .add_systems(Update, (draw_balls_radius, move_balls));
    }
}

const BALL_SIZE: f32 = 10.0;
const BALLS_CIRCLE_RADIUS: f32 = 80.0;
const BALLS_CENTER_POSITION_X: f32 = 0.0;
const BALLS_CENTER_POSITION_Y: f32 = -300.0;
const BALLS_ANGULAR_SPEED: f32 = 300.0;

#[derive(Component)]
pub struct Ball {
    pub angle_deg: f32,
}

impl Ball {
    pub fn new(angle: f32) -> Self {
        Self {
            angle_deg: angle,
        }
    }
}

fn get_ball_position(angle: f32) -> (f32, f32) {
    let pos_x = BALLS_CENTER_POSITION_X + BALLS_CIRCLE_RADIUS * ops::cos(angle.to_radians());
    let pos_y = BALLS_CENTER_POSITION_Y + BALLS_CIRCLE_RADIUS * ops::sin(angle.to_radians());
    (pos_x, pos_y)
}

fn spawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (left_ball_x, left_ball_y) = get_ball_position(180.0);
    let (right_ball_x, right_ball_y) = get_ball_position(0.0);

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BALL_SIZE))),
        MeshMaterial2d(materials.add(Color::srgb(0.34, 0.72, 0.68))),
        Transform::from_translation(Vec3::new(left_ball_x, left_ball_y, 1.)),
        Ball::new(180.0),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BALL_SIZE))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.63, 0.4))),
        Transform::from_translation(Vec3::new(right_ball_x, right_ball_y, 1.)),
        Ball::new(0.0),
    ));
}

fn draw_balls_radius(mut gizmos: Gizmos) {
    gizmos.circle_2d(
        Isometry2d::new(Vec2::new(BALLS_CENTER_POSITION_X, BALLS_CENTER_POSITION_Y), Rot2::degrees(0.0)),
        BALLS_CIRCLE_RADIUS,
        Color::srgb(0.94, 0.95, 1.0),
    ).resolution(64);
}

fn move_balls(
    mut balls_query: Query<(&mut Transform, &mut Ball)>,
    key_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut ball) in balls_query.iter_mut() {
        if key_input.pressed(KeyCode::ArrowRight) {
            ball.angle_deg += BALLS_ANGULAR_SPEED * time.delta_secs();
        } else if key_input.pressed(KeyCode::ArrowLeft) {
            ball.angle_deg -= BALLS_ANGULAR_SPEED * time.delta_secs();
        } else {
            return
        }

        let (ball_x, ball_y) = get_ball_position(ball.angle_deg);
        let mov_vec = Vec3::new(ball_x - transform.translation.x, ball_y - transform.translation.y, 0.0);
        transform.translation += mov_vec;
    }
}
