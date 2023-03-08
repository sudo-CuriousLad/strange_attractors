use std::{
    ops::{Mul},
    vec,
};

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(init_particle)
        .add_system(tick)
        .add_system(trail)
        .add_system(tick_trail)
        .insert_resource(ClearColor(Color::DARK_GRAY.mul(0.2)))
        .run()
}

fn init_particle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.01,
                sectors: 10,
                stacks: 10,
            })),
            material: materials.add(StandardMaterial {
                emissive: Color::CYAN.as_rgba().into(),
                ..default()
            }),
            transform: Transform::from_translation(Vec3 {
                x: 0.7,
                y: 0.0,
                z: 0.0,
            }),
            ..default()
        },
        Particle {
            consts: vec![15.6, 1.0, 28.0, -1.143, -0.714],
        },
    ));
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vec3::Z,
            ),
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        BloomSettings {
            threshold: 1.0,
            intensity: 100.0,
            ..default()
        },
    ));
}

#[derive(Component)]
struct Particle {
    consts: Vec<f32>,
}

#[derive(Component)]
struct TrailParticle {
    lifetime: f32,
}

fn calc_chua(pos: Vec3, c: Vec<f32>) -> Vec3 {
    let px = pos[0];
    let py = pos[1];
    let pz = pos[2];
    let c1 = c[0];
    let c2 = c[1];
    let c3 = c[2];
    let m0 = c[3];
    let m1 = c[4];

    let fx = (m1 * px + (m0 - m1)) / (2.0 * ((px + 1.0).abs() - (px - 1.0).abs()));

    let x = c1 * (py - px - fx);
    let y = c2 * (px - py + pz);
    let z = -1.0 * c3 * py;

    return Vec3 { x: x, y: y, z: z };
}

fn tick(mut particles: Query<(&mut Transform, &mut Particle)>) {
    for (mut transform, particle) in &mut particles {
        let pos = transform.translation;

        transform.translation += calc_chua(pos, particle.consts.to_vec()) * 0.02;
    }
}

fn trail(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut particles: Query<(&mut Transform, &mut Particle)>,
) {
    for (transform, mut _particle) in &mut particles {
        let pos = transform.translation;

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: 0.01,
                    sectors: 10,
                    stacks: 10,
                })),
                material: materials.add(StandardMaterial {
                    emissive: Color::Rgba {
                        red: logis_curve(pos.x.abs()),
                        green: logis_curve(pos.y.abs()),
                        blue: logis_curve(pos.z.abs()),
                        alpha: 1.0,
                    },
                    ..default()
                }),
                transform: Transform::from_translation(pos),
                ..default()
            },
            TrailParticle { lifetime: 0.0 },
        ));
    }
}

fn tick_trail(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut particles: Query<(
        Entity,
        &mut Transform,
        &mut Handle<StandardMaterial>,
        &mut TrailParticle,
    )>,
) {
    for (id, mut transform, mut handle, mut particle) in &mut particles {
        let pos = transform.translation;
        if particle.lifetime >= 1.0 {
            // commands.entity(id).despawn();
        } else {
            // transform.scale *= 1.0 - particle.lifetime;
            *handle = materials.add(StandardMaterial {
                emissive: Color::Rgba {
                    red: logis_curve(pos.z.powf(5.0))*1.5,
                    green: logis_curve(pos.y.powf(5.0))*1.5,
                    blue: logis_curve(pos.x.powf(5.0))*1.5,
                    alpha: 1.0,
                }
                .as_rgba()
                .into(),
                ..default()
            });
            particle.lifetime += 0.005;
        }
    }
}

fn logis_curve(input: f32) -> f32 {
    let e: f32 = 2.718281828459045;

    return 1.0 / (1.0 + (1.0 / (e.powf(input))));
}

fn camera_rotation(mut camera: Query<(
    Entity,
    &mut Transform,
    &mut Camera,
    &mut TrailParticle,
)>) {

}
