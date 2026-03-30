use bevy::{input::keyboard::KeyboardInput, prelude::*};

use crate::{
    gravity::Mass,
    thruster::{ConstantRightThruster, UpThruster},
};

use super::GameState;

// Plugin
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // This plugin will contain the game. In this case, it's just be a screen that will
        // display the current settings for 5 seconds before returning to the menu
        app.add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(
                Update,
                (handle_player, handle_keyboard).run_if(in_state(GameState::Game)),
            );
    }
}

// TODO: maybe extract into its own file
// marker entity for any player entity
#[derive(Component)]
pub struct Player;

fn game_setup(mut commands: Commands) {
    commands
        .spawn(Player)
        .insert(Name::new("Player"))
        .insert(UpThruster)
        .insert(ConstantRightThruster)
        .insert(Mass { amount_kg: 1000.0 });
}

fn handle_player(player_query: Query<&Player>) {
    for _player in player_query.iter() {
        // TODO
    }
}

fn handle_keyboard(
    mut keyboard_inputs: MessageReader<KeyboardInput>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for keyboard_input in keyboard_inputs.read() {
        if keyboard_input.key_code.eq(&KeyCode::Escape) {
            game_state.set(GameState::Menu);
        }
    }
}
