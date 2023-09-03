use bevy::prelude::*;
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
        .add_systems(Startup, (setup, setup_ground))
        .add_systems(Startup, bevy::window::close_on_esc)
        .add_systems(Update, (move_character, jump))
        .run();
}

#[derive(Component)]
pub struct MainCharacter {
    pub speed: f32,
    
}
#[derive(Component)]
pub struct Jumper{
    pub jump_impulse: f32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //~ Spawn Camera
    commands.spawn(Camera2dBundle::default());

    //~ Character
    let character: Handle<Image> = asset_server.load("character.png");
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..Default::default()
            },
            texture: character,
            ..Default::default()
        },
        MainCharacter { speed: 300.0 },
        Jumper{ jump_impulse: 10.0},
    ));
}

fn setup_ground(mut commands: Commands) {
    commands
        .spawn(Collider::cuboid(500.0, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -200.0, 0.0)));
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
        
    }
}


fn jump(
    mut players: Query<(&Jumper, &mut RigidBodyVelocity), With<MainCharacter>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
){
    for (mut transform, character) in &mut character_position {
        let character_speed = character.speed * time.delta_seconds();
        if keyboard_input.pressed(KeyCode::Space) {
            //~ Jump
            transform.translation.y += character_speed;
        }
        
    }
}