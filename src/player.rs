use crate::token::{Position, Token, TokenBundle};
use bevy::{
    input::Input,
    prelude::{Bundle, Commands, Component, KeyCode, Plugin, Query, Res, With},
};

pub struct PlayerPlugin;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,

    #[bundle]
    token: TokenBundle,
}

impl PlayerBundle {
    fn new(x: i32, y: i32) -> Self {
        Self {
            player: Player,
            token: TokenBundle::new(x, y, '@'),
        }
    }
}

#[derive(Component, Default)]
pub struct Player;

fn move_player(mut p_q: Query<&mut Position, With<Player>>, input: Res<Input<KeyCode>>) {
    let mut player = p_q.single_mut();

    let mut del = [0, 0];

    for k in input.get_just_pressed() {
        match k {
            KeyCode::Up => del[1] += 1,
            KeyCode::Down => del[1] -= 1,
            KeyCode::Left => del[0] -= 1,
            KeyCode::Right => del[0] += 1,
            _ => {}
        }
    }

    player.0 += del[0];
    player.1 += del[1];
}

fn setup(mut cmd: Commands) {
    cmd.spawn_bundle(PlayerBundle::new(10, 10));
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup).add_system(move_player);
    }
}
