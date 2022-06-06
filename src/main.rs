mod plant;
mod player;
mod token;

use bevy::{prelude::*, DefaultPlugins};
use bevy_ascii_terminal::*;
use bevy_tiled_camera::*;
use plant::PlantPlugin;
use player::PlayerPlugin;
use token::TokenPlugin;

fn start(mut cmd: Commands) {
    let size = [100, 100];
    let mut t = TerminalBundle::new().with_size(size);
    let term = &mut t.terminal;

    term.draw_border_single();

    cmd.spawn_bundle(t);
    cmd.spawn_bundle(TiledCameraBundle::default().with_tile_count(size));
}

fn adjust_cam(mut q: Query<&mut TiledProjection>, mut w: ResMut<Windows>) {
    let p = q.single_mut();

    let win = w.get_primary_mut().expect("There's a window");

    let size = (p.pixels_per_tile * 100) as u16;
    win.set_resolution(size.into(), size.into());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_plugin(TiledCameraPlugin)
        .add_plugin(TokenPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(PlantPlugin)
        .add_startup_system(start)
        .add_system(adjust_cam)
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
