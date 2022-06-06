use crate::player::Player;
use bevy::prelude::*;

use crate::token::{Position, Token, TokenBundle};

#[derive(Component)]
struct SpawnTime(f64);

#[derive(Component)]
struct Plant;

#[derive(Bundle)]
struct PlantBundle {
    plant: Plant,
    spawn_time: SpawnTime,

    #[bundle]
    token: TokenBundle,
}

fn grow(mut q: Query<(&mut Token, &mut SpawnTime), With<Plant>>, time: Res<Time>) {
    for (mut p, mut spawn_time) in q.iter_mut() {
        if time.seconds_since_startup() - spawn_time.0 > 10.0 {
            p.0 = '|';
            spawn_time.0 = time.seconds_since_startup();
        }
    }
}

fn plant_plant(
    mut cmd: Commands,
    q: Query<&Position, With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if input.just_pressed(KeyCode::Z) {
        let player = q.single();

        cmd.spawn_bundle(PlantBundle {
            spawn_time: SpawnTime(time.seconds_since_startup()),
            plant: Plant,
            token: TokenBundle::new(player.0, player.1, 'P'),
        });
    }
}

pub struct PlantPlugin;
impl Plugin for PlantPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(plant_plant).add_system(grow);
    }
}
