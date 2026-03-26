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
                    update_controlled_thrusters,
                    update_constant_thrusters,
                )
                    .run_if(in_state(Game)),
            );
    }
}

// Marker Components (for parent entities)

#[derive(Component)]
pub struct ConstantUpThruster;

#[derive(Component)]
pub struct ConstantRightThruster;

#[derive(Component)]
pub struct UpThruster;

#[derive(Component)]
pub struct RightThruster;

#[derive(Component)]
pub struct UpThrusterFlame;

#[derive(Component)]
pub struct RightThrusterFlame;

// Thruster Components
#[derive(Component)]
pub enum ThrustDirection {
    UP,
    RIGHT,
}

#[derive(Component)]
pub enum ThrusterType {
    CONSTANT,
    CONTROLLED,
}

#[derive(Component, Default)]
pub struct Thruster {
    pub direction: Vec3,
    pub force: f32,
}

#[derive(Bundle)]
struct ThrusterBundle {
    name: Name,
    thruster: Thruster,
    offset: Transform,
    direction: ThrustDirection,
    thruster_type: ThrusterType,
}

// Messages

#[derive(Message)]
pub struct ThrustUp;

#[derive(Message)]
pub struct ThrustRight;

// Systems

fn up_thruster_added(
    mut commands: Commands,
    controlled_added: Query<Entity, Added<UpThruster>>,
    constant_added: Query<Entity, Added<ConstantUpThruster>>,
) {
    for add in controlled_added.iter() {
        commands.entity(add).with_child(ThrusterBundle {
            name: Name::new("UpThruster"),
            thruster: Thruster {
                direction: Vec3::new(0.0, 1.0, 0.0),
                force: 1.0,
            },
            offset: Transform::default(),
            direction: ThrustDirection::UP,
            thruster_type: ThrusterType::CONTROLLED,
        });
        debug!("up thruster added {}", NAME);
    }

    for add in constant_added.iter() {
        commands.entity(add).with_child(ThrusterBundle {
            name: Name::new("ConstantUpThruster"),
            thruster: Thruster {
                direction: Vec3::new(0.0, 1.0, 0.0),
                force: 1.0,
            },
            offset: Transform::default(),
            direction: ThrustDirection::UP,
            thruster_type: ThrusterType::CONSTANT,
        });
        debug!("constant up thruster added {}", NAME);
    }
}

fn right_thruster_added(
    mut commands: Commands,
    controlled_added: Query<Entity, Added<RightThruster>>,
    constant_added: Query<Entity, Added<ConstantRightThruster>>,
) {
    for add in controlled_added.iter() {
        commands.entity(add).with_child(ThrusterBundle {
            name: Name::new("RightThruster"),
            thruster: Thruster {
                direction: Vec3::new(1.0, 0.0, 0.0),
                force: 1.0,
            },
            offset: Transform::default(),
            direction: ThrustDirection::RIGHT,
            thruster_type: ThrusterType::CONTROLLED,
        });
        debug!("right thruster added {}", NAME);
    }

    for add in constant_added.iter() {
        commands.entity(add).with_child(ThrusterBundle {
            name: Name::new("ConstantRightThruster"),
            thruster: Thruster {
                direction: Vec3::new(1.0, 0.0, 0.0),
                force: 1.0,
            },
            offset: Transform::default(),
            direction: ThrustDirection::RIGHT,
            thruster_type: ThrusterType::CONSTANT,
        });
        debug!("constant right thruster added {}", NAME);
    }
}

fn update_controlled_thrusters(
    mut up_thruster_reader: MessageReader<ThrustUp>,
    mut right_thruster_reader: MessageReader<ThrustRight>,
    mut parent_transform: Query<
        (&Children, &mut Transform),
        Or<(With<RightThruster>, With<UpThruster>)>,
    >,
    thrusters: Query<(&Thruster, &ThrustDirection, &ThrusterType)>,
) {
    for _ in up_thruster_reader.read() {
        for (children, mut transform) in parent_transform.iter_mut() {
            for child in children {
                if let Ok((thruster, ThrustDirection::UP, ThrusterType::CONTROLLED)) =
                    thrusters.get(*child)
                {
                    transform.translation += thruster.direction;
                }
            }
        }
    }

    for _ in right_thruster_reader.read() {
        for (children, mut transform) in parent_transform.iter_mut() {
            for child in children {
                if let Ok((thruster, ThrustDirection::RIGHT, ThrusterType::CONTROLLED)) =
                    thrusters.get(*child)
                {
                    transform.translation += thruster.direction;
                }
            }
        }
    }
}

fn update_constant_thrusters(
    mut parent_transform: Query<
        (&Children, &mut Transform),
        Or<(With<ConstantUpThruster>, With<ConstantRightThruster>)>,
    >,
    thrusters: Query<(&Thruster, &ThrusterType)>,
) {
    for (children, mut transform) in parent_transform.iter_mut() {
        for child in children {
            if let Ok((thruster, ThrusterType::CONSTANT)) = thrusters.get(*child) {
                transform.translation += thruster.direction;
            }
        }
    }
}
