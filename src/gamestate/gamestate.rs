use bevy::prelude::*;
use crate::actors::lillies::create_lily;
use crate::actors::frog::Frog;

const PIXEL_RATIO : f32 = 3.0;

pub fn init_game(
    mut commands : Commands,
    asset_server : Res<AssetServer>,
)
{
    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.8)));

    commands.spawn(Camera2d::default());

    setup_lilies(&mut commands, &asset_server);

    commands.spawn((Sprite{
            image: asset_server.load("frogit.png"),
            ..Default::default()
            },
        Transform::IDENTITY.with_scale(Vec3::splat(PIXEL_RATIO)),
        Frog{ velocity: 3., direction: Vec3::ZERO , position: 5})
    );
}

fn setup_lilies(
    commands : &mut Commands,
    asset_server : &Res<AssetServer>
)
{
    let grid = 5;
    let lily_size = 32.0 * PIXEL_RATIO;
    let mut counter: u8 = 0;
    for row in 0..grid {
        for col in 0..grid {
            let x = (col as f32 - (grid as f32 - 1.0) / 2.0) * lily_size;
            let y = ((grid as f32 - 1.0) / 2.0 - row as f32) * lily_size;
            create_lily(commands, asset_server, Vec3::new(x, y, 0.0), counter);
            counter += 1;
        }
    }
}