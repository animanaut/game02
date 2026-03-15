use bevy::{input::keyboard::KeyboardInput, prelude::*};

use super::GameState::Game;

// Constants
const NAME: &str = "hover controller";

// Plugin
pub struct HoverControllerPlugin;

impl Plugin for HoverControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Messages
            .add_message::<HoverUp>()
            // Systems
            .add_systems(OnEnter(Game), setup_controller)
            .add_systems(Update, (update_controller).run_if(in_state(Game)));
    }
}

#[derive(Component)]
struct HoverController;

#[derive(Component)]
struct Hover;

#[derive(Message)]
struct HoverUp;

fn setup_controller(mut commands: Commands) {
    debug!("starting {}", NAME);
    commands
        .spawn(Hover)
        .insert(HoverController)
        .insert(DespawnOnExit(Game));
}

fn update_controller(
    mut keyboard_inputs: MessageReader<KeyboardInput>,
    controller_query: Query<(Entity, &HoverController)>,
    // this line crashes in runtime, no idea why
    mut _hover_up_events: MessageWriter<HoverUp>,
) {
    debug!("updating {}", NAME);
    for keyboard_input in keyboard_inputs.read() {
        debug!("{:?}", keyboard_input);
        for controller in controller_query.iter() {
            debug!("controller {:?}", controller.0)
        }
    }
}
