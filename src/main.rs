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
        .add_systems(Startup, gamestate::health::setup_hearts)
        .add_systems(Update, actors::trash::move_trash)
        .add_systems(Update, actors::frog::update_frog)
        .add_systems(Update, actors::trash::spawn_trash)
        .add_systems(Update, (gamestate::score::update_score, gamestate::score::update_score_text))
        .add_systems(Update, actors::trash::remove_trash)
        .add_systems(Update, gamestate::health::update_hearts_ui)
        .run();
}