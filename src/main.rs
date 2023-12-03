//! Update a scene from a glTF file, either by spawning the scene as a child of another entity,
//! or by accessing the entities of the scene.

use bevy::{asset::AssetMetaCheck, math::vec3, prelude::*, window::PresentMode};

fn main() {
    App::new() // App
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Game".into(),
                canvas: Some("#my-game".into()),
                fit_canvas_to_parent: true,
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // cube
    commands.spawn(PbrBundle {
        material: materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("texture.png")),
            ..default()
        }),
        mesh: meshes.add(shape::Cube::new(0.5).into()),
        ..default()
    });

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.).into()),
        material: materials.add(Color::GRAY.into()),
        transform: Transform::from_translation(Vec3::Y * -0.5 / 2.),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(vec3(2., 3., 2.)),
        point_light: PointLight {
            intensity: 1000.,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(vec3(1., 1., -1.)).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
