use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use std::f32::consts::PI;
use bevy::window::CursorGrabMode;


fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "3D Viewer".to_string(),
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(cursor_grab_system)
        .add_system(rotate)
        .add_system(rotate_negative)
        .add_system(mouse_click_system)
        .run();
}

#[derive(Component)]
struct Rotator;
#[derive(Component)]
struct  RotatorN;
#[derive(Component)]
struct  IsCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plain
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {size: 5.0})),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // capsule
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {..default()})),
        material: materials.add(Color::rgb(0.9, 0.3, 0.3).into()),
        transform: Transform::from_xyz(0.0, 1.0, 0.0).with_rotation(Quat::from_rotation_x(-PI / 4.)),
        ..default()
    }, RotatorN));

    // light
    commands.spawn((PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    }, Rotator));

    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-1.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }, IsCamera));
}

fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_grab_mode(CursorGrabMode::Confined);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_grab_mode(CursorGrabMode::None);
        window.set_cursor_visibility(true);
    }
}

fn mouse_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<IsCamera>>
) {
    if mouse_button_input.pressed(MouseButton::Right) {
        for ev in motion_evr.iter() {
            for mut transfrom in &mut query {
                // println!("{:?}", transfrom)
                let scale: f32 = 10.0;
                let trasform_delta = Transform::from_xyz(ev.delta.x / scale, ev.delta.y / scale, 0.0);
                transfrom.translation += trasform_delta.translation;
                println!("{:?}", trasform_delta.translation)
            }
        }
    }

    if mouse_button_input.pressed(MouseButton::Left) {
        for ev in motion_evr.iter() {
            // println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
            for mut transfrom in &mut query {
                let scale: f32 = 10.0;
                let roation_amount = (ev.delta.x / scale).clamp(-0.1, 0.1);
                transfrom.rotate_around(Vec3::ZERO, Quat::from_rotation_y(roation_amount))
            }
        }
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        info!("left mouse just pressed");
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        info!("left mouse just released");
    }
}

fn rotate(mut query: Query<&mut Transform, With<Rotator>>, time: Res<Time>) {
    for mut transform in &mut query {
        // transform.rotate_x(time.delta_seconds() / 1.5);
        transform.rotate_y(time.delta_seconds() / 1.);
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_seconds()))
    }
}

fn rotate_negative(mut query: Query<&mut Transform, With<RotatorN>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_x(time.delta_seconds() / 1.5);
        // transform.rotate_y(time.delta_seconds() / 1.);
        // transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(-time.delta_seconds()))
    }
}