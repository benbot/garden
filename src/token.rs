use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ascii_terminal::Terminal;

#[derive(Component, Debug, Eq, PartialEq, Hash)]
pub struct Position(pub i32, pub i32);

#[derive(Component, Debug)]
pub struct Token(pub char);

#[derive(Bundle)]
pub struct TokenBundle {
    pub token: Token,
    pub pos: Position,
}

impl TokenBundle {
    pub fn new(x: i32, y: i32, token: char) -> Self {
        TokenBundle {
            token: Token(token),
            pos: Position(x, y),
        }
    }
}

impl Default for TokenBundle {
    fn default() -> Self {
        TokenBundle::new(0, 0, '_')
    }
}

pub struct TokenPlugin;

fn render(mut term_q: Query<&mut Terminal>, token_q: Query<(&Position, &Token)>, time: Res<Time>) {
    let mut term = term_q.single_mut();
    term.clear();
    term.draw_border_single();

    let mut positions_seen: HashMap<(i32, i32), Vec<char>> = HashMap::new();
    for (pos, Token(token)) in token_q.iter() {
        match positions_seen.get_mut(&(pos.0, pos.1)) {
            Some(tokens) => tokens.push(*token),
            None => {
                let mut vec = Vec::new();
                vec.push(*token);
                positions_seen.insert((pos.0, pos.1), vec);
            }
        }
    }

    for (k, v) in positions_seen.iter_mut() {
        v.sort();
        let which_to_show = v[time.seconds_since_startup().trunc() as usize % v.len()];

        term.put_char([k.0, k.1], which_to_show);
    }
}

impl Plugin for TokenPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_before(
            CoreStage::Last,
            "ascii_render",
            SystemStage::single_threaded(),
        )
        .add_system_to_stage("ascii_render", render);
    }
}
