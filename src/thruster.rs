use bevy::prelude::*;

use super::GameState::Game;

// Constants
const NAME: &str = "thruster";

// Plugin
pub struct ThrusterPlugin;

impl Plugin for ThrusterPlugin {
    fn build(&self, app: &mut App) {
        app
            // Messages
            .add_message::<ThrustUp>()
            .add_message::<ThrustRight>()
            // Systems
            .add_systems(
                Update,
                (up_thruster_added, right_thruster_added).run_if(in_state(Game)),
            );
    }
}

#[derive(Component)]
pub struct UpThruster;

#[derive(Component)]
pub struct RightThruster;

#[derive(Message)]
pub struct ThrustUp;

#[derive(Message)]
pub struct ThrustRight;

#[derive(Component)]
pub struct Thruster {
    direction: Vec3,
}

#[derive(Bundle)]
struct ThrusterBundle {
    name: Name,
    thruster: Thruster,
}

// Systems

fn up_thruster_added(mut commands: Commands, added: Query<Entity, Added<UpThruster>>) {
    for add in added.iter() {
        commands.entity(add).with_child(ThrusterBundle {
            name: Name::new("UpThruster"),
            thruster: Thruster {
                direction: Vec3::new(0.0, 1.0, 0.0),
            },
        });
    }
}

fn right_thruster_added(mut commands: Commands, added: Query<Entity, Added<RightThruster>>) {
    for add in added.iter() {
        commands.entity(add).with_child(ThrusterBundle {
            name: Name::new("RightThruster"),
            thruster: Thruster {
                direction: Vec3::new(1.0, 0.0, 0.0),
            },
        });
    }
}
