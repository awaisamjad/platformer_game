use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Platform Game".into(),
                        resolution: (960.0, 720.0).into(),
                        resizable: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_systems(
            Startup,
            (setup, setup_ground, setup_level_1_a, setup_level_1_a),
        )
        .add_systems(Startup, bevy::window::close_on_esc)
        .add_systems(Update, (move_character, spawn_dynamite))
        .insert_resource(Money(100))
        .run();
}

#[derive(Component)]
pub struct MainCharacter {
    pub speed: f32,
}

#[derive(Resource)]
pub struct Money(pub u8);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    //~ Spawn Camera
    commands.spawn(Camera2dBundle::default());

    //~ Spawn Background
    // let background = asset_server.load("background.png");
    // commands.spawn(
    //     (SpriteBundle {
    //         sprite: Sprite {
    //             custom_size: Some(Vec2::new(960.0,720.0)),
    //             ..Default::default()
    //         },
    //         texture: background,
    //         ..Default::default()
    //     }),
    // );

    //~ Character
    let character: Handle<Image> = asset_server.load("character.png");
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(50.0, 50.0),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..Default::default()
            },
            texture: character,
            transform: Transform::from_translation(Vec3::new(-210.0, 40.0, 0.0)),
            ..Default::default()
        },
        MainCharacter { speed: 300.0 },
    ));
}

fn setup_ground(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ground: Handle<Image> = asset_server.load("ground.png");
    let ground_x = 800.0;
    let ground_y = 200.0;
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(ground_x * 0.5, (ground_y * 0.5) - 10.0),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(ground_x, ground_y)),
                ..Default::default()
            },
            texture: ground,
            transform: Transform::from_translation(Vec3::new(0.0, -300.0, 0.0)),
            ..Default::default()
        },
    ));
}

fn setup_level_1_a(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ground: Handle<Image> = asset_server.load("ground.png");
    let ground_x = 500.0;
    let ground_y = 200.0;
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(ground_x * 0.5, (ground_y * 0.5) - 10.0),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(ground_x, ground_y)),
                ..Default::default()
            },
            texture: ground,
            transform: Transform::from_translation(Vec3::new(-300.0, 0.0, 0.0)),
            ..Default::default()
        },
    ));
}

fn setup_level_1_b(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ground: Handle<Image> = asset_server.load("ground.png");
    let ground_x = 500.0;
    let ground_y = 200.0;
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(ground_x * 0.5, (ground_y * 0.5) - 10.0),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(ground_x, ground_y)),
                ..Default::default()
            },
            texture: ground,
            transform: Transform::from_translation(Vec3::new(300.0, 0.0, 0.0)),
            ..Default::default()
        },
    ));
}

fn move_character(
    mut character_position: Query<(&mut Transform, &MainCharacter)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, character) in &mut character_position {
        let character_speed = character.speed * time.delta_seconds();
        if keyboard_input.pressed(KeyCode::A) {
            //~ Left
            transform.translation.x -= character_speed;
        }
        if keyboard_input.pressed(KeyCode::D) {
            //~ Right
            transform.translation.x += character_speed;
        }
        if keyboard_input.pressed(KeyCode::S) {
            //~ Up
            transform.translation.y -= character_speed;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            //~ Jump
            transform.translation.y += character_speed;
        }
    }
}

fn spawn_dynamite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut money_amount: ResMut<Money>,
    character: Query<&Transform, With<MainCharacter>>,
) {
    if !keyboard_input.just_pressed(KeyCode::E) {
        return;
    }
    let character_transform = character.single();
    let dynamite_price: u8 = 20;
    if money_amount.0 >= dynamite_price {
        money_amount.0 -= dynamite_price;
        info!(
            "Placed Dynamite!!! Cost: £20!!! Total Amount: {}",
            money_amount.0
        );
        let dynamite: Handle<Image> = asset_server.load("dynamite.png");

        commands.spawn((
            RigidBody::Dynamic,
            GravityScale(0.9),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(60.0, 60.0)),
                    ..Default::default()
                },
                texture: dynamite,
                transform: *character_transform,
                ..Default::default()
            },
        ));
    }
}

fn spawn_bronze_coin(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut money_amount: ResMut<Money>,
    character: Query<&Transform, With<MainCharacter>>,
) {
    let bronze_coin: Handle<Image> = asset_server.load("bronze coin.png");
    commands.spawn(
        (SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(60.0, 60.0)),
                ..Default::default()
            },
            texture: bronze_coin,
            transform: Transform::from_translation(Vec3::new(10.0, 0.0, 0.0)),
            ..Default::default()
        }),
    );
}

fn collect_bronze_coin(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut money_amount: ResMut<Money>,
    character: Query<&Transform, With<MainCharacter>>,
    bronze_coin: Query<&Transform>,
) {
    let money_amount: u8 = 5;
    let character_transform = character.single();
}
