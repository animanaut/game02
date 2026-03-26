//! This example will display a simple menu using Bevy UI where you can start a new game,
//! change some settings or quit. There is no actual game, it will just display the current
//! settings for 5 seconds before going back to the menu.

use app_states::AppStatesPlugin;
use game::GamePlugin;
//use game_camera::GameCameraPlugin;
use gravity::GravityPlugin;
use hover_controller::HoverControllerPlugin;
use menu::MenuPlugin;
use splash::SplashPlugin;
use sprite::SpritePlugin;
use thruster::ThrusterPlugin;

use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::app_states::GameState;

mod app_states;
mod game;
mod game_camera;
mod gravity;
mod hover_controller;
mod menu;
mod splash;
mod sprite;
mod thruster;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        // Insert as resource the initial value for the settings resources
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        // Declare the game state, whose starting value is determined by the `Default` trait
        .add_systems(Startup, setup)
        // Adds the plugins for each state
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins((
            AppStatesPlugin,
            SplashPlugin,
            MenuPlugin,
            GamePlugin,
            //GameCameraPlugin,
            HoverControllerPlugin,
            ThrusterPlugin,
            SpritePlugin,
            GravityPlugin,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
