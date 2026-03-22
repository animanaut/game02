use bevy::prelude::*;

use super::GameState::Game;

// Constants
const NAME: &str = "sprite";
const SPRITE_FILE: &str = "TODO";

// Plugin
pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app
            // Messages
            // Systems
            .add_systems(OnEnter(Game), sprite_setup)
            .add_systems(Update, (sprite_update).run_if(in_state(Game)));
    }
}

#[derive(Bundle)]
struct SpriteBundle {
    name: Name,
}

// Systems

fn sprite_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    debug!("setup {}", NAME);
    commands.spawn(Sprite::from_image(asset_server.load(SPRITE_FILE)));
}

fn sprite_update() {
    debug!("update {}", NAME);
}
