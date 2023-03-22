#[cfg(feature = "reflect")]
use bevy::{ecs::reflect::ReflectResource, reflect::Reflect};
use bevy::{prelude::*, utils::HashMap};

pub struct BoidsPlugin;
impl Plugin for BoidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_intent.before(apply_physics))
            .add_system(apply_physics);

        #[cfg(feature = "reflect")]
        app.register_type::<BoidDescriptor>();
    }
}

#[derive(Resource)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Resource))]
pub struct BoidDescriptor {
    pub thrust: f32,
    pub lift: f32,
    pub gravity: f32,
    pub bank: f32,
    pub separation: f32,
    pub alignment: f32,
    pub cohesion: f32,
    pub bank_rate: f32,
    pub rise_rate: f32,
    pub maximum_vision: f32,
}

impl Default for BoidDescriptor {
    fn default() -> Self {
        Self {
            thrust: 2.0,
            lift: 1.0,
            gravity: 1.0,
            bank: 1.0,
            separation: 100.0,
            alignment: 1.0,
            cohesion: 10.0,
            bank_rate: 0.01,
            rise_rate: 0.01,
            maximum_vision: 240.0,
        }
    }
}

#[derive(Component)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct Boid;

fn apply_intent(
    mut boids_query: Query<(Entity, &mut Transform, &GlobalTransform), With<Boid>>,
    mut headings: Local<HashMap<Entity, (f32, f32)>>,
    descriptor: Res<BoidDescriptor>,
    time: Res<Time>,
) {
    for (boid, transform, global_transform) in boids_query.iter() {
        let mut heading = Vec3::ZERO;

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

            let separation =
                (position - other_position).normalize_or_zero() * descriptor.separation / distance;
            let alignment = other_global_transform.forward() * descriptor.alignment;
            let cohesion = (other_position - position).normalize_or_zero() * descriptor.cohesion;

            heading += separation + alignment + cohesion;
        }

        let lateral = descriptor.bank_rate
            * ((heading.dot(transform.left()) + transform.up().dot(Vec3::Y))
                - transform.left().dot(Vec3::Y));
        let vertical =
            descriptor.rise_rate * (heading.dot(transform.up()) + transform.up().dot(Vec3::Y));

        headings.insert(boid, (lateral, vertical));
    }

    for (boid, mut transform, _) in boids_query.iter_mut() {
        let &(lateral, vertical) = headings.get(&boid).unwrap_or(&(0.0, 0.0));

        transform.rotate_local_z(lateral * time.delta_seconds());
        transform.rotate_local_x(vertical * time.delta_seconds());
    }
}

fn apply_physics(
    mut boids_query: Query<&mut Transform, With<Boid>>,
    descriptor: Res<BoidDescriptor>,
    time: Res<Time>,
) {
    for mut transform in boids_query.iter_mut() {
        let thrust = transform.forward() * descriptor.thrust;
        let lift = transform.up() * descriptor.lift;
        let gravity = Vec3::NEG_Y * descriptor.gravity;
        transform.translation += (thrust + lift + gravity) * time.delta_seconds();

        let bank = transform.right().dot(Vec3::Y) * descriptor.bank;
        transform.rotate_y(bank * time.delta_seconds());
    }
}
