use bevy::prelude::*;

use crate::game::Player;

use super::GameState::Game;

// Constants
const NAME: &str = "sprite";
const HOVER_SPRITE: &str = "sprites/craft-dev.png";
pub const SPRITE_SCALE: f32 = 6.0;

// Plugin
pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app
            // Messages
            // Systems
            .add_systems(OnEnter(Game), sprite_setup)
            .add_systems(Update, (player_added, sprite_update).run_if(in_state(Game)));
    }
}

// Systems
fn sprite_setup() {
    debug!("setup {}", NAME);
}

fn player_added(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    added: Query<Entity, Added<Player>>,
) {
    for add in added.iter() {
        let handle = asset_server.load(HOVER_SPRITE);
        let sprite = Sprite::from_image(handle);
        commands.entity(add).insert((
            sprite,
            Transform::from_scale(Vec3::splat(SPRITE_SCALE))
                .with_translation(vec3(-400., 100., 0.)),
            DespawnOnExit(Game),
        ));
        debug!("added player sprite {}", NAME);
    }
}

fn sprite_update() {
    debug!("update {}", NAME);
}
