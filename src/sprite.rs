use std::time::Duration;

use bevy::prelude::*;

use crate::{
    game::Player,
    thruster::{RightThrusterFlame, ThrustDirection, Thruster, UpThrusterFlame},
};

use super::GameState::Game;

// Constants
const NAME: &str = "sprite";
const HOVER_SPRITE: &str = "sprites/craft-dev.png";
const RIGHT_FLAME_SPRITE: &str = "sprites/right-flame.png";
const UP_FLAME_SPRITE: &str = "sprites/up-flame.png";
pub const SPRITE_SCALE: f32 = 6.0;
pub const SPRITE_SIZE: f32 = 32.0;

// Plugin
pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app
            // Messages
            // Systems
            .add_systems(OnEnter(Game), sprite_setup)
            .add_systems(Update, execute_animations)
            .add_systems(
                Update,
                (player_added, thruster_added, sprite_update).run_if(in_state(Game)),
            );
    }
}

// Components
#[derive(Component)]
struct SpriteAnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    //fps: u8,
    frame_timer: Timer,
}

impl SpriteAnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            //fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(
            Duration::from_secs_f32(1.0 / (fps as f32)),
            TimerMode::Repeating,
        )
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

fn thruster_added(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    added: Query<(Entity, &Transform, &ThrustDirection), Added<Thruster>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (add, _transform, direction) in added.iter() {
        let mut offset = Vec3::default();
        let animation_config = SpriteAnimationConfig::new(0, 1, 12);
        match direction {
            ThrustDirection::UP => {
                offset.y = -SPRITE_SIZE;
                let handle = asset_server.load(UP_FLAME_SPRITE);
                let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 2, 1, None, None);
                let texture_atlas_layout = texture_atlas_layouts.add(layout);

                let sprite = Sprite {
                    image: handle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: 0,
                    }),
                    ..default()
                };

                commands.entity(add).insert((
                    UpThrusterFlame,
                    Visibility::Hidden,
                    sprite,
                    animation_config,
                    Transform::from_translation(offset),
                    DespawnOnExit(Game),
                ));
            }
            ThrustDirection::RIGHT => {
                offset.x = -SPRITE_SIZE;
                let handle = asset_server.load(RIGHT_FLAME_SPRITE);
                let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 2, 1, None, None);
                let texture_atlas_layout = texture_atlas_layouts.add(layout);

                let sprite = Sprite {
                    image: handle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: 0,
                    }),
                    ..default()
                };
                commands.entity(add).insert((
                    RightThrusterFlame,
                    Visibility::Hidden,
                    sprite,
                    animation_config,
                    Transform::from_translation(offset),
                    DespawnOnExit(Game),
                ));
            }
        }
        debug!("added flame sprite {}", NAME);
    }
}

fn sprite_update() {
    debug!("update {}", NAME);
}

fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut SpriteAnimationConfig, &mut Sprite)>,
) {
    for (mut config, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());

        if config.frame_timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            if atlas.index == config.last_sprite_index {
                atlas.index = config.first_sprite_index;
            } else {
                atlas.index += 1;
            }
        }
    }
}
