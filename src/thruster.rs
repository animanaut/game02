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
                (
                    up_thruster_added,
                    right_thruster_added,
                    update_up_thrusters,
                    update_right_thrusters,
                )
                    .run_if(in_state(Game)),
            );
    }
}

#[derive(Component)]
pub enum ThrustDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
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
    pub direction: Vec3,
}

#[derive(Bundle)]
struct ThrusterBundle {
    name: Name,
    thruster: Thruster,
    transform: Transform,
    direction: ThrustDirection,
}

// Systems

fn up_thruster_added(mut commands: Commands, added: Query<Entity, Added<UpThruster>>) {
    for add in added.iter() {
        commands.entity(add).with_child(ThrusterBundle {
            name: Name::new("UpThruster"),
            thruster: Thruster {
                direction: Vec3::new(0.0, 1.0, 0.0),
            },
            transform: Transform::default(),
            direction: ThrustDirection::UP,
        });
        debug!("up thruster added {}", NAME);
    }
}

fn right_thruster_added(mut commands: Commands, added: Query<Entity, Added<RightThruster>>) {
    for add in added.iter() {
        commands.entity(add).with_child(ThrusterBundle {
            name: Name::new("RightThruster"),
            thruster: Thruster {
                direction: Vec3::new(1.0, 0.0, 0.0),
            },
            transform: Transform::default(),
            direction: ThrustDirection::RIGHT,
        });
        debug!("right thruster added {}", NAME);
    }
}

// TODO : dont iterate over all thrusters, just the relevant ones

fn update_up_thrusters(
    mut up_thruster_reader: MessageReader<ThrustUp>,
    mut parent_transform: Query<(&Children, &mut Transform), With<UpThruster>>,
    thrusters: Query<(&Thruster, &ThrustDirection)>,
) {
    for _ in up_thruster_reader.read() {
        for (children, mut transform) in parent_transform.iter_mut() {
            for child in children {
                if let Ok((thruster, ThrustDirection::UP)) = thrusters.get(*child) {
                    transform.translation += thruster.direction;
                }
            }
        }
    }
}

fn update_right_thrusters(
    mut right_thruster_reader: MessageReader<ThrustRight>,
    mut parent_transform: Query<(&Children, &mut Transform), With<RightThruster>>,
    thrusters: Query<(&Thruster, &ThrustDirection)>,
) {
    for _ in right_thruster_reader.read() {
        for (children, mut transform) in parent_transform.iter_mut() {
            for child in children {
                if let Ok((thruster, ThrustDirection::RIGHT)) = thrusters.get(*child) {
                    transform.translation += thruster.direction;
                }
            }
        }
    }
}
