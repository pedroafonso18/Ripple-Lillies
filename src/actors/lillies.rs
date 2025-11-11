use bevy::prelude::*;

const PIXEL_RATIO : f32 = 3.0;

#[derive(Component)]
pub struct Lily
{
    pub grid_pos : u8
}

pub fn create_lily(
    commands : &mut Commands,
    asset_server : &Res<AssetServer>,
    position : Vec3,
    grid_pos : u8
)
{
    commands.spawn((Sprite{
            image: asset_server.load("lilypad.png"),
            ..Default::default()
        },
        Transform::from_translation(position).with_scale(Vec3::splat(PIXEL_RATIO)),
        Lily{grid_pos: grid_pos}
    ));
}


pub fn setup_lilies(
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