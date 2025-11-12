use bevy::prelude::*;
use crate::actors::lillies::setup_lilies;
use crate::gamestate::score::setup_score;
use crate::actors::frog::setup_frog;
use crate::actors::trash::setup_trash;

pub fn init_game(
    mut commands : Commands,
    asset_server : Res<AssetServer>,
)
{
    let background = asset_server.load("background.gif");
    commands.spawn((Sprite {
        image: background,
        ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, -10.0)
    ));

    commands.spawn(Camera2d::default());

    setup_score(&mut commands, &asset_server);
    
    setup_lilies(&mut commands, &asset_server);

    setup_frog(&mut commands, &asset_server);

    setup_trash(&mut commands);
}

pub fn get_coordinate_grid(grid_pos: &u8) -> Vec3 {
    let grid = 5;
    let mut counter: u8 = 0;
    let trash_size = 32.0 * 3.;
    for row in 0..grid {
        for col in 0..grid {
            let x = (col as f32 - (grid as f32 - 1.0) / 2.0) * trash_size;
            let y = ((grid as f32 - 1.0) / 2.0 - row as f32) * trash_size;
            if &counter == grid_pos {
                return Vec3::new(x,y,0.);
            }
            counter += 1;
        }
    }
    return Vec3::ZERO;
}