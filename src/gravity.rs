use bevy::prelude::*;

use super::GameState::Game;

// Constants
const NAME: &str = "gravity";

// Events / Messages

// Resources

// Components

// Bundles

// Plugin
pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app
            // Types
            // Messages
            // Systems
            .add_systems(Update, (update_gravity,).run_if(in_state(Game)));
    }
}

// Systems

fn update_gravity() {
    // TODO
    debug!("updating {}", NAME);
}
