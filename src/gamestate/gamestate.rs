use bevy::prelude::*;
use crate::actors::lillies::setup_lilies;
use crate::gamestate::score::setup_score;
use crate::actors::frog::setup_frog;

pub fn init_game(
    mut commands : Commands,
    asset_server : Res<AssetServer>,
)
{
    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.8)));

    commands.spawn(Camera2d::default());

    setup_score(&mut commands, &asset_server);
    
    setup_lilies(&mut commands, &asset_server);

    setup_frog(&mut commands, &asset_server);
}
