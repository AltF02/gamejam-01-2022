use crate::plugins::player::PlayerComponent;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::physics::RapierConfiguration;
use bevy_rapier2d::prelude::RigidBodyVelocityComponent;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>()
            .add_system(exit_game)
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(player_movement));
    }
}

#[derive(Default)]
pub struct Actions {
    pub player_movement: bool,
}

fn exit_game(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_config: Res<RapierConfiguration>,
    mut player_info: Query<(&PlayerComponent, &mut RigidBodyVelocityComponent)>,
    mut actions: ResMut<Actions>,
) {
    for (player, mut rb) in player_info.iter_mut() {
        let up = GameControl::Up.pressed(&keyboard_input);
        let down = GameControl::Down.pressed(&keyboard_input);
        let right = GameControl::Right.pressed(&keyboard_input);
        let left = GameControl::Left.pressed(&keyboard_input);

        actions.player_movement = up || left || down || right;

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        if x_axis != 0 {
            rb.linvel.x = player.speed * (x_axis as f32) * rapier_config.scale;
        } else {
            rb.linvel.x = 0.0;
        }

        if y_axis != 0 {
            rb.linvel.y = player.speed * (y_axis as f32) * rapier_config.scale;
        } else {
            rb.linvel.y = 0.0;
        }
    }
}

enum GameControl {
    Up,
    Down,
    Left,
    Right,
}

impl GameControl {
    fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right)
            }
        }
    }
}
