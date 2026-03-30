use bevy::prelude::*;

use super::GameState::Game;

// Constants
const NAME: &str = "gravity";

// Events / Messages

// Components

#[derive(Component)]
pub struct Mass {
    pub amount_kg: f32,
}

#[derive(Component)]
pub struct Velocity {
    pub m_per_second: Vec3,
}

// Resources
#[derive(Resource)]
pub struct GravityResource {
    /// acceleration force vector in m/s^2
    pub acceleration: Vec3,
}

impl Default for GravityResource {
    fn default() -> Self {
        Self {
            acceleration: Vec3 {
                x: 0.,
                y: -9.81,
                z: 0.,
            },
        }
    }
}

// Components

// Bundles

// Plugin
pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app
            // Types
            .init_resource::<GravityResource>()
            // Messages
            // Systems
            .add_systems(Update, (update_gravity, mass_added).run_if(in_state(Game)));
    }
}

// Systems

fn update_gravity(
    time: Res<Time>,
    mut mass_objects: Query<(&Mass, &mut Transform, &mut Velocity)>,
    gravity: Res<GravityResource>,
) {
    debug!("updating {}, gravity:{:?}", NAME, gravity.acceleration);
    for (mass, mut transform, mut velocity) in mass_objects.iter_mut() {
        debug!(
            "mass:{} at position:{} with velocity:{}",
            mass.amount_kg, transform.translation, velocity.m_per_second
        );
        velocity.m_per_second.y -= 0.3 * time.delta_secs();
        transform.translation += velocity.m_per_second;
    }
}

fn mass_added(mut commands: Commands, added: Query<Entity, Added<Mass>>) {
    for add in added.iter() {
        commands.entity(add).insert(Velocity {
            m_per_second: Vec3::default(),
        });
    }
}
