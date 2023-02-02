#[cfg(feature = "reflect")]
use bevy::{ecs::reflect::ReflectResource, reflect::Reflect};
use bevy::{
    prelude::{
        App, Component, Entity, GlobalTransform, Local, Plugin, Quat, Query, Res, Resource, Time,
        Transform, Vec3, With,
    },
    utils::HashMap,
};

pub struct BoidsPlugin;
impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_boid);

        #[cfg(feature = "reflect")]
        app.register_type::<BoidDescriptor>();
    }
}

#[derive(Resource)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Resource))]
pub struct BoidDescriptor {
    pub speed: f32,
    pub minimum_distance: f32,
    pub maximum_distance: f32,
    pub maximum_vision: f32,
    pub rotational_inertia: f32,
    pub rotational_energy: f32,
}

impl Default for BoidDescriptor {
    fn default() -> Self {
        Self {
            speed: 1.0,
            minimum_distance: 2.0,
            maximum_distance: 3.0,
            maximum_vision: 4.0,
            rotational_inertia: 1.0,
            rotational_energy: 1.0,
        }
    }
}

#[derive(Component)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct Boid;

fn move_boid(
    mut boids_query: Query<(Entity, &mut Transform, &GlobalTransform), With<Boid>>,
    mut headings: Local<HashMap<Entity, Vec3>>,
    descriptor: Res<BoidDescriptor>,
    time: Res<Time>,
) {
    for (boid, _, global_transform) in boids_query.iter() {
        let mut separation = Vec3::ZERO;
        let mut alignment = Vec3::ZERO;
        let mut cohesion = Vec3::ZERO;
        let mut minimum_distance = descriptor.maximum_vision;

        for (other_boid, _, other_global_transform) in boids_query.iter() {
            if other_boid == boid {
                continue;
            }

            let position = global_transform.translation();
            let other_position = other_global_transform.translation();
            let distance = position.distance(other_position);

            if distance > descriptor.maximum_vision {
                continue;
            }

            if distance < minimum_distance {
                minimum_distance = distance;
            }

            separation += (position - other_position).normalize_or_zero()
                * descriptor.minimum_distance
                / distance;

            alignment += other_global_transform.forward();

            cohesion += (other_position - position).normalize_or_zero() * distance
                / descriptor.maximum_distance;
        }

        let heading = {
            if minimum_distance < descriptor.minimum_distance {
                separation
            } else if minimum_distance > descriptor.maximum_distance {
                cohesion
            } else {
                alignment
            }
        };

        headings.insert(boid, heading);
    }

    for (boid, mut transform, _) in boids_query.iter_mut() {
        let heading = (*headings.get(&boid).unwrap_or(&Vec3::ZERO)).normalize_or_zero();
        let rot =
            descriptor.rotational_energy * time.delta_seconds() / descriptor.rotational_inertia;
        let direction = transform
            .rotation
            .lerp(Quat::from_rotation_arc(transform.forward(), heading), rot);
        transform.rotation = direction;

        let displacement = transform.forward() * descriptor.speed * time.delta_seconds();
        transform.translation += displacement;
    }
}
