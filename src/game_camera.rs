use bevy::prelude::*;

use super::GameState::Game;

// Constants
const NAME: &str = "game camera";

// Plugin
pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Game), game_camera_setup)
            .add_systems(Update, (update_camera).run_if(in_state(Game)));
    }
}

#[derive(Component)]
struct GameCamera;

fn game_camera_setup(mut commands: Commands) {
    debug!("starting {}", NAME);
    commands
        .spawn(GameCamera)
        .insert(DespawnOnExit(Game))
        .insert(Name::new("GameCamera"))
        .insert(Camera3d::default());
}

fn update_camera(camera_query: Query<&GameCamera>) {
    debug!("updating {}", NAME);
    for _camera in camera_query.iter() {
        // TODO
    }
}
