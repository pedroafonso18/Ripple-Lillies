use bevy::prelude::*;

mod gamestate;
mod actors;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin{
                primary_window: Some(Window{
                    title : String::from("Ripple-Lillies"),
                    position : WindowPosition::Centered(MonitorSelection::Primary),
                    resolution : UVec2::new(512, 512).into(),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest()))
        .add_systems(Startup, gamestate::gamestate::init_game)
        .add_systems(Update, actors::frog::update_frog)
        .run();
}