use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

const PLAYER_SPEED: f32 = 500.0; // player sprite speed
const PLAYER_SIZE: f32 = 64.0; // player sprite size
const NUM_ENEMIES: usize = 4;
const ENEMY_SPEED: f32 = 200.0; // enemy sprite speed
const ENEMY_SIZE: f32 = 64.0; // enemy sprite size
const NUM_STARS: usize = 10; // number of stars to spawn
const STAR_SIZE: f32 = 30.0; // star sprite size
const STAR_SPAWN_TIME: f32 = 1.0; // time between star spawns

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Startup, spawn_stars)
        .add_systems(Update,player_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, update_enemy_direction)
        .add_systems(Update, confine_enemy_movement)
        .add_systems(Update, enemy_hit_player)
        .add_systems(Update,player_hit_star)
        .add_systems(Update, update_score)
        .add_systems(Update, tick_star_spawn_timer)
        .add_systems(Update, spawn_stars_over_time)
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,

}

#[derive(Component)]
pub struct Star {}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
       return Score { value: 0};
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        return StarSpawnTimer { timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating)}
    }
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                texture: asset_server.load("sprites/ball_blue_large.png"),
                ..default()
            },
            Player {},
        )
    );
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUM_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/ball_red_large.png"),
                    ..default()
                },
                Enemy {
                    direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                }
            )
        );
    }
}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUM_STARS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/star.png"),
                    ..default()
                },
                Star {},
            )
        );
    }
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
) {
    if let Ok (mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0,0.0,0.0);
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0,0.0,0.0);
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0,1.0,0.0);
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0,-1.0,0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        // bound player x position
        if translation.x < x_min {
            translation.x = x_min;
        }
        else if translation.x > x_max {
            translation.x = x_max;
        }
        // bound player y position
        if translation.y < y_min {
            translation.y = y_min;
        }
        else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;

        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        // play SFX
        if direction_changed {
            let sfx_1 = asset_server.load("audio/pluck_001.ogg");
            let sfx_2 = asset_server.load("audio/pluck_002.ogg");
            let random_num = random::<f32>();
            let sfx_played = match random_num {
                num if num > 0.5 => sfx_1,
                _ => sfx_2,
            };
            commands.spawn(
                AudioBundle {
                    source: sfx_played,
                    ..default()
                }
            );
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        // bound player x position
        if translation.x < x_min {
            translation.x = x_min;
        }
        else if translation.x > x_max {
            translation.x = x_max;
        }
        // bound player y position
        if translation.y < y_min {
            translation.y = y_min;
        }
        else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform.translation.distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Enemy hit player! Game over!");
                // play sfx
                let sfx = asset_server.load("audio/explosionCrunch_000.ogg");
                commands.spawn(
                    AudioBundle {
                        source: sfx,
                        ..default()
                    }
                );
                // despawn player
                commands.entity(player_entity).despawn();
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single_mut() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform.translation.distance(star_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;
            if distance < player_radius + star_radius {
                println!("Player hit star!");
                // update score
                score.value += 1;
                // play sfx
                let sfx = asset_server.load("audio/laserLarge_000.ogg");
                commands.spawn(
                    AudioBundle {
                        source: sfx,
                        ..default()
                    }
                );
                // despawn player
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/star.png"),
                    ..default()
                },
                Star {},
            )
        );
    }
}