use bevy::prelude::*;

use super::GameState::Game;

// Constants
const NAME: &str = "gravity";

// Events / Messages

// Resources
#[derive(Resource)]
pub struct GravityResource {
    /// direction of gravity in unit vector direction
    pub direction: Vec3,
    /// acceleration force in m/s^2
    pub force: f32,
}

impl Default for GravityResource {
    fn default() -> Self {
        Self {
            direction: Vec3 {
                x: 0.,
                y: -1.,
                z: 0.,
            },
            force: 9.81,
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
            .add_systems(Update, (update_gravity,).run_if(in_state(Game)));
    }
}

// Systems

fn update_gravity(gravity: Res<GravityResource>) {
    debug!(
        "updating {}, dir:{:?}, f:{:?}",
        NAME, gravity.direction, gravity.force
    );
}
