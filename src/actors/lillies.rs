use bevy::prelude::*;

const PIXEL_RATIO : f32 = 3.0;

#[derive(Component)]
pub struct Lily
{
    pub position : Vec3,
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
        Lily{position : position, grid_pos: grid_pos},
    ));
}