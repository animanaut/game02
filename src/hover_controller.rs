use bevy::prelude::*;

use crate::thruster::{ThrustRight, ThrustUp, ThrustUpStandby};

use super::GameState::Game;

// Constants
const NAME: &str = "hover controller";

// Plugin
pub struct HoverControllerPlugin;

impl Plugin for HoverControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Messages
            // Systems
            .add_systems(Update, (update_controller_events).run_if(in_state(Game)));
    }
}

// Systems

fn update_controller_events(
    keyboard_inputs: Res<ButtonInput<KeyCode>>,
    mut thrust_up_events: MessageWriter<ThrustUp>,
    mut thrust_up_off_events: MessageWriter<ThrustUpStandby>,
    mut thrust_right_events: MessageWriter<ThrustRight>,
) {
    debug!("updating {}", NAME);

    if keyboard_inputs.pressed(KeyCode::KeyW) {
        thrust_up_events.write(ThrustUp);
        debug!("player pressed up");
    }

    if keyboard_inputs.just_released(KeyCode::KeyW) {
        thrust_up_off_events.write(ThrustUpStandby);
        debug!("player pressed up");
    }

    if keyboard_inputs.pressed(KeyCode::KeyD) {
        // for now only constant right thrust
        //    thrust_right_events.write(ThrustRight);
        debug!("player pressed right");
    }

    // hardcoded constant right thrust for the moment
    thrust_right_events.write(ThrustRight);
}
