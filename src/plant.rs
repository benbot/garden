use crate::player::Player;
use bevy::{ecs::component, prelude::*};
use rand::prelude::*;

use crate::token::{Position, Token, TokenBundle};

#[derive(Component)]
struct SpawnTime(f64);

#[derive(Component)]
struct Growing(bool);

#[derive(Component)]
struct Plant(PlantType);

#[derive(Bundle)]
struct PlantBundle {
    plant: Plant,
    spawn_time: SpawnTime,
    growing: Growing,

    #[bundle]
    token: TokenBundle,
}

pub enum PlantType {
    WHEAT,
}

impl PlantBundle {
    fn new(x: i32, y: i32, time: &Time) -> Self {
        Self {
            spawn_time: SpawnTime(time.seconds_since_startup()),
            growing: Growing(true),
            plant: Plant(PlantType::WHEAT),
            token: TokenBundle::new(x, y, 'P'),
        }
    }
}

fn grow(
    mut cmd: Commands,
    mut q: Query<(&mut Token, &Position, &mut SpawnTime, &mut Growing, &Plant)>,
    time: Res<Time>,
) {
    for (mut p, pos, mut spawn_time, mut growing, Plant(plant_type)) in q.iter_mut() {
        if !growing.0 {
            continue;
        }

        match plant_type {
            PlantType::WHEAT => {
                if time.seconds_since_startup() - spawn_time.0 > 3.0
                    && thread_rng().gen_range(0..100) > 30
                {
                    match p.0 {
                        'P' => p.0 = '|',
                        '\\' => {}
                        '/' => {}
                        '|' => {
                            p.0 = '|';
                            let u: i32 = thread_rng().gen_range(-1..=1);
                            let mut plant = PlantBundle::new(pos.0 + u, pos.1 + 1, time.as_ref());
                            plant.token.token = match u {
                                0 => Token('|'),
                                1 => Token('/'),
                                -1 => Token('\\'),
                                _ => panic!("woah {} not expected!", u),
                            };
                            cmd.spawn_bundle(plant);
                        }
                        _ => panic!("Woah {} not expected!", p.0),
                    }
                    spawn_time.0 = time.seconds_since_startup();
                    if thread_rng().gen_range(0..100) as u32 > 90 {
                        growing.0 = false;
                    }
                }
            }
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

        cmd.spawn_bundle(PlantBundle::new(player.0, player.1, time.as_ref()));
    }
}

pub struct PlantPlugin;
impl Plugin for PlantPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(plant_plant).add_system(grow);
    }
}
