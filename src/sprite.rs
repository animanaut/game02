use bevy::prelude::*;

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
    let handle = asset_server.load(HOVER_SPRITE);
    let sprite = Sprite::from_image(handle);
    // TODO: how to default_nearest to avoid blurry sprites
    commands.spawn((
        sprite,
        Transform::from_scale(Vec3::splat(SPRITE_SCALE)).with_translation(vec3(400., 400., 0.)),
        DespawnOnExit(Game),
    ));
}

fn sprite_update() {
    debug!("update {}", NAME);
}
