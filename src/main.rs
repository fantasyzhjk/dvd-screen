use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::system::exit_on_esc_system,
    math::Quat,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use rand::Rng;

pub struct PrintTimer(Timer);
pub struct TickTimer(Timer);
pub struct Position(Transform);

struct MeowNei {
    velocity: Vec3,
}

struct Spaze {
    stat: bool,
}

struct Storage {
    background: bool,
}

enum Collider {
    Solid,
    Scorable,
    Paddle,
}
struct Scoreboard {
    score: usize,
}

const WINDOW_WIDTH: f32 = 1600.;
const WINDOW_HEIGHT: f32 = 900.;

///This example is for performance testing purposes.
///See https://github.com/bevyengine/bevy/pull/1492
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "tiny test".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            vsync: false,
            resizable: false,
            ..Default::default()
        })
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::DEBUG,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(Spaze { stat: false })
        .insert_resource(Storage { background: false })
        // .insert_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
        .insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
        .add_startup_system(setup.system())
        .add_system(print_timer.system())
        .add_system(tick_timer.system().label("Tick"))
        .add_system(toggle_cursor.system().after("Tick"))
        .add_system(move_camera.system().after("Tick"))
        .add_system(meow_nei_move.system().after("Tick"))
        .add_system(meow_nei_bounce.system().after("Tick"))
        .add_system(exit_on_esc_system.system())
        .run()
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tile_size = Vec2::splat(128.0);

    let sprite_handle = materials.add(assets.load("timg-w.png").into());

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(PrintTimer(Timer::from_seconds(1.0, true)))
        .insert(TickTimer(Timer::from_seconds(0.05, true)))
        .insert(Position(Transform::from_translation(Vec3::new(
            0.0, 0.0, 10000.0,
        ))));

    // commands.spawn_bundle(Text2dBundle {
    //     text: Text::with_section(
    //         "Hello World",
    //         TextStyle {
    //             font: assets.load("fonts/FiraSans-Bold.ttf"),
    //             font_size: 60.0,
    //             color: Color::WHITE,
    //         },
    //         TextAlignment {
    //             vertical: VerticalAlign::Center,
    //             horizontal: HorizontalAlign::Center,
    //         },
    //     ),
    //     transform: Transform::from_xyz(0.0,200.0,0.0),
    //     ..Default::default()
    // });

    // // Add walls
    // let wall_material = materials.add(Color::rgba(0.8, 0.8, 0.8, 1.0).into());
    // let wall_thickness = 100.0;
    // let bounds = Vec2::new(WINDOW_WIDTH + wall_thickness,WINDOW_HEIGHT + wall_thickness);

    // // left
    // commands
    //     .spawn_bundle(SpriteBundle {
    //         material: wall_material.clone(),
    //         transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
    //         sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
    //         ..Default::default()
    //     })
    //     .insert(Collider::Solid);
    // // right
    // commands
    //     .spawn_bundle(SpriteBundle {
    //         material: wall_material.clone(),
    //         transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
    //         sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
    //         ..Default::default()
    //     })
    //     .insert(Collider::Solid);
    // // bottom
    // commands
    //     .spawn_bundle(SpriteBundle {
    //         material: wall_material.clone(),
    //         transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
    //         sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
    //         ..Default::default()
    //     })
    //     .insert(Collider::Solid);
    // // top
    // commands
    //     .spawn_bundle(SpriteBundle {
    //         material: wall_material,
    //         transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
    //         sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
    //         ..Default::default()
    //     })
    //     .insert(Collider::Solid);

    let position = Vec2::new(0 as f32, 0 as f32);
    let translation = (position * tile_size).extend(0.0);
    // let rotation = Quat::from_rotation_z(rng.gen::<f32>());
    let rotation = Quat::from_rotation_z(0.);
    let scale = Vec3::splat(1.0);
    commands
        .spawn_bundle(SpriteBundle {
            material: sprite_handle.clone(),
            transform: Transform {
                translation,
                rotation,
                scale,
            },
            sprite: Sprite::new(Vec2::new(115., 51.)),
            ..Default::default()
        })
        .insert(MeowNei {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        });
}

fn move_camera(time: Res<Time>, mut query: Query<(&mut Transform, &mut Position)>) {
    // for (mut transform, mut position) in query.iter_mut() {
    //     position
    //         .0
    //         .rotate(Quat::from_rotation_z(time.delta_seconds() * 0.5));
    //     position.0 =
    //         position.0 * Transform::from_translation(Vec3::X * CAMERA_SPEED * time.delta_seconds());
    //     transform.translation = position.0.translation;
    //     // transform.rotation *= Quat::from_rotation_z(time.delta_seconds() / 2.0);
    // }
}

fn print_timer(time: Res<Time>, sprites: Query<&Sprite>, mut query: Query<&mut PrintTimer>) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            println!("Sprites: {}", sprites.iter().count(),);
        }
    }
}

fn tick_timer(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
    mut time_query: Query<&mut TickTimer>,
    mut query: Query<(&MeowNei, &mut Transform, &Handle<ColorMaterial>)>,
    mut spaze: ResMut<Spaze>,
    mut storage: ResMut<Storage>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for mut timer in time_query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            spaze.stat = input.pressed(KeyCode::Space);
            let window = windows.get_primary_mut().unwrap();
            let (_, mut transform, color_mat) = query.single_mut().unwrap();
            window.set_title(format!("meow_nei position: {}", transform.translation));
            let mut c = materials.get_mut(color_mat).unwrap();
            if spaze.stat == true {
                // commands.insert_resource(ClearColor(Color::rgb_u8(190, 0, 0)));
                // let mut rng = rand::thread_rng();
                // c.color = Color::rgb(
                //     rng.gen_range(0.0..1.0),
                //     rng.gen_range(0.0..1.0),
                //     rng.gen_range(0.0..1.0),
                // );
                // transform.scale =
                //     Vec2::new(rng.gen_range(0.0..5.0), rng.gen_range(0.0..10.0)).extend(0.0);
            } else {
                c.color = Color::rgb(0., 0., 0.);
                commands.insert_resource(ClearColor(Color::rgb(1., 1., 1.)));
                transform.scale = Vec2::new(1., 1.).extend(0.0);
            }
        }
    }
}

fn toggle_cursor(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    // let window = windows.get_primary_mut().unwrap();
    // if input.just_pressed(KeyCode::Space) {
    //     window.set_cursor_lock_mode(!window.cursor_locked());
    //     window.set_cursor_visibility(!window.cursor_visible());
    // }
}

fn meow_nei_move(
    time: Res<Time>,
    mut query: Query<(&mut MeowNei, &mut Transform)>,
    mut spaze: ResMut<Spaze>,
) {
    for (mut meow_nei, mut transform) in query.iter_mut() {
        if spaze.stat {
            transform.rotation *= Quat::from_rotation_z(time.delta_seconds() * 50.);
            let mut rng = rand::thread_rng();
            let rng_x = rng.gen_range(0.9..1.1);
            let rng_y = rng.gen_range(0.9..1.1);
            meow_nei.velocity.x *= rng_x;
            meow_nei.velocity.y *= rng_y;
            if meow_nei.velocity.x.abs() < 10. {
                meow_nei.velocity.x *= 10.;
            }
            if meow_nei.velocity.y.abs() < 10. {
                meow_nei.velocity.y *= 10.;
            }
            transform.translation += meow_nei.velocity * time.delta_seconds() * 10.;
        }
        transform.translation += meow_nei.velocity * time.delta_seconds() * 2.;
        // println!("{}", meow_nei.velocity)
    }
}

fn meow_nei_bounce(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut query: Query<(&mut MeowNei, &Transform, &Sprite)>,
    collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
) {
    for (mut meow_nei, meow_nei_transform, sprite) in query.iter_mut() {
        let meow_nei_size = sprite.size;
        let velocity = &mut meow_nei.velocity;
        // println!("x: {:?} y: {:?}", transform.translation.x, transform.translation.y);
        // println!("size: {:?}", sprite.size)
        let bounds = Vec2::new(WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.);
        if (meow_nei_transform.translation.x + (meow_nei_size.x / 2.)) > bounds.x {
            if velocity.x > 0. {
                velocity.x = -velocity.x
            };
        } else if (meow_nei_transform.translation.x - (meow_nei_size.x / 2.)) < -bounds.x {
            if velocity.x < 0. {
                velocity.x = -velocity.x
            };
        } else if (meow_nei_transform.translation.y + (meow_nei_size.y / 2.)) > bounds.y {
            if velocity.y > 0. {
                velocity.y = -velocity.y
            };
        } else if (meow_nei_transform.translation.y - (meow_nei_size.y / 2.)) < -bounds.y {
            if velocity.y < 0. {
                velocity.y = -velocity.y
            };
        }
        for (collider_entity, collider, transform, sprite) in collider_query.iter() {
            let collision = collide(
                meow_nei_transform.translation,
                meow_nei_size,
                transform.translation,
                sprite.size,
            );
            if let Some(collision) = collision {
                // scorable colliders should be despawned and increment the scoreboard on collision
                if let Collider::Scorable = *collider {
                    scoreboard.score += 1;
                    commands.entity(collider_entity).despawn();
                }

                // reflect the ball when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // only reflect if the ball's velocity is going in the opposite direction of the
                // collision
                match collision {
                    Collision::Left => reflect_x = velocity.x > 0.0,
                    Collision::Right => reflect_x = velocity.x < 0.0,
                    Collision::Top => reflect_y = velocity.y < 0.0,
                    Collision::Bottom => reflect_y = velocity.y > 0.0,
                }

                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    velocity.x = -velocity.x;
                }

                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    velocity.y = -velocity.y;
                }

                // break if this collide is on a solid, otherwise continue check whether a solid is
                // also in collision
                if let Collider::Solid = *collider {
                    break;
                }
            }
        }
    }
}
