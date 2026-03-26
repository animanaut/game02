use bevy::prelude::*;

use crate::thruster::{RightThruster, ThrustRight, ThrustUp, UpThruster};

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
            .add_systems(OnEnter(Game), setup_controller)
            .add_systems(Update, (update_controller_events,).run_if(in_state(Game)));
    }
}

#[derive(Component)]
struct HoverController;

#[derive(Component)]
struct Hover;

// Systems
fn setup_controller(mut commands: Commands) {
    debug!("starting {}", NAME);
    commands
        .spawn(Hover)
        .insert(HoverController)
        .insert(UpThruster)
        .insert(RightThruster)
        .insert(DespawnOnExit(Game));
}

fn update_controller_events(
    keyboard_inputs: Res<ButtonInput<KeyCode>>,
    mut thrust_up_events: MessageWriter<ThrustUp>,
    mut thrust_right_events: MessageWriter<ThrustRight>,
) {
    debug!("updating {}", NAME);

    if keyboard_inputs.pressed(KeyCode::KeyW) {
        thrust_up_events.write(ThrustUp);
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
