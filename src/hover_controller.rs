use bevy::{input::keyboard::KeyboardInput, prelude::*};

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
    mut keyboard_inputs: MessageReader<KeyboardInput>,
    controller_query: Query<(Entity, &HoverController)>,
    mut thrust_up_events: MessageWriter<ThrustUp>,
    mut thrust_right_events: MessageWriter<ThrustRight>,
) {
    debug!("updating {}", NAME);
    for keyboard_input in keyboard_inputs.read() {
        debug!("{:?}", keyboard_input);
        for controller in controller_query.iter() {
            debug!("controller {:?}", controller.0);

            // TODO: how to make this configurable for the end user

            if keyboard_input.key_code.eq(&KeyCode::KeyW) {
                thrust_up_events.write(ThrustUp);
                debug!("player pressed up");
            }

            if keyboard_input.key_code.eq(&KeyCode::KeyD) {
                thrust_right_events.write(ThrustRight);
                debug!("player pressed right");
            }
        }
    }

    // hardcoded constant right thrust for the moment
    //thrust_right_events.write(ThrustRight);
}
