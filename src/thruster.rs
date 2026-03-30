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
            .add_message::<ThrustUpOff>()
            .add_message::<ThrustRightOff>()
            // Systems
            .add_systems(
                Update,
                (
                    up_thruster_added,
                    right_thruster_added,
                    update_controlled_thrusters,
                    update_constant_thrusters,
                    thruster_state_change,
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

#[derive(Component)]
pub enum ThrusterState {
    #[allow(dead_code)]
    OFF,
    ON,
    FIRING,
}

#[derive(Component, Default)]
pub struct Thruster {
    pub direction: Vec3,
    // TODO: todo
    #[allow(dead_code)]
    pub force: f32,
}

#[derive(Bundle)]
struct ThrusterBundle {
    name: Name,
    thruster: Thruster,
    offset: Transform,
    direction: ThrustDirection,
    thruster_type: ThrusterType,
    thruster_state: ThrusterState,
}

// Messages

#[derive(Message)]
pub struct ThrustUp;

#[derive(Message)]
pub struct ThrustRight;

#[derive(Message)]
pub struct ThrustUpOff;

#[derive(Message)]
pub struct ThrustRightOff;

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
            thruster_state: ThrusterState::ON,
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
            thruster_state: ThrusterState::ON,
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
            thruster_state: ThrusterState::ON,
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
            thruster_state: ThrusterState::ON,
        });
        debug!("constant right thruster added {}", NAME);
    }
}

fn update_controlled_thrusters(
    mut up_thruster_reader: MessageReader<ThrustUp>,
    mut up_thruster_off_reader: MessageReader<ThrustUpOff>,
    mut right_thruster_reader: MessageReader<ThrustRight>,
    mut right_thruster_off_reader: MessageReader<ThrustRightOff>,
    mut parent_transform: Query<
        (&Children, &mut Transform),
        Or<(With<RightThruster>, With<UpThruster>)>,
    >,
    mut thrusters: Query<(
        &Thruster,
        &ThrustDirection,
        &ThrusterType,
        &mut ThrusterState,
    )>,
) {
    for _ in up_thruster_off_reader.read() {
        for (children, _) in parent_transform.iter_mut() {
            for child in children {
                if let Ok((_, ThrustDirection::UP, ThrusterType::CONTROLLED, mut thruster_state)) =
                    thrusters.get_mut(*child)
                {
                    *thruster_state = ThrusterState::ON;
                }
            }
        }
    }

    for _ in right_thruster_off_reader.read() {
        for (children, _) in parent_transform.iter_mut() {
            for child in children {
                if let Ok((
                    _,
                    ThrustDirection::RIGHT,
                    ThrusterType::CONTROLLED,
                    mut thruster_state,
                )) = thrusters.get_mut(*child)
                {
                    *thruster_state = ThrusterState::ON;
                }
            }
        }
    }

    for _ in up_thruster_reader.read() {
        for (children, mut transform) in parent_transform.iter_mut() {
            for child in children {
                if let Ok((
                    thruster,
                    ThrustDirection::UP,
                    ThrusterType::CONTROLLED,
                    mut thruster_state,
                )) = thrusters.get_mut(*child)
                {
                    transform.translation += thruster.direction;
                    *thruster_state = ThrusterState::FIRING;
                }
            }
        }
    }

    for _ in right_thruster_reader.read() {
        for (children, mut transform) in parent_transform.iter_mut() {
            for child in children {
                if let Ok((
                    thruster,
                    ThrustDirection::RIGHT,
                    ThrusterType::CONTROLLED,
                    mut thruster_state,
                )) = thrusters.get_mut(*child)
                {
                    transform.translation += thruster.direction;
                    *thruster_state = ThrusterState::FIRING;
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
    mut thrusters: Query<(
        &Thruster,
        &ThrustDirection,
        &ThrusterType,
        &mut ThrusterState,
    )>,
) {
    for (children, mut transform) in parent_transform.iter_mut() {
        for child in children {
            if let Ok((thruster, _, ThrusterType::CONSTANT, mut thruster_state)) =
                thrusters.get_mut(*child)
            {
                transform.translation += thruster.direction;
                // TODO state firing
                *thruster_state = ThrusterState::FIRING;
            }
        }
    }
}

fn thruster_state_change(
    mut thrusters: Query<
        (&ThrusterState, &mut Visibility),
        (With<Thruster>, Changed<ThrusterState>),
    >,
) {
    for (state, mut visibility) in thrusters.iter_mut() {
        *visibility = match state {
            ThrusterState::OFF => Visibility::Hidden,
            ThrusterState::ON => Visibility::Hidden,
            ThrusterState::FIRING => Visibility::Visible,
        };
    }
}
