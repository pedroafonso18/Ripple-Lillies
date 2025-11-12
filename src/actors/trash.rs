use bevy::prelude::*;
use rand::{self, Rng};
use crate::gamestate::gamestate::get_coordinate_grid;

#[derive(Component)]
pub struct Trash{
   pub grid_pos : u8 
}

#[derive(Resource)]
pub struct TrashTimer(Timer);

pub fn spawn_trash(
    mut commands : Commands,
    asset_server : Res<AssetServer>,
    time : Res<Time>,
    mut timer : ResMut<TrashTimer>,
)
{
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::rng();
        let trash_number = rng.random_range(1..4);
        let spawn_tile = rng.random_range(1..26);
        let image_str = format!("trash{}.png", trash_number);
        let pos = get_coordinate_grid(&spawn_tile);
        commands.spawn((
            Sprite{
                image : asset_server.load(image_str),
                ..Default::default()
            },
            Trash{ grid_pos: spawn_tile },
            Transform::from_xyz(pos.x,pos.y,pos.z).with_scale(Vec3::splat(3.))
        ));
    }
}

pub fn setup_trash(
    commands : &mut Commands,
)
{
    commands.insert_resource(TrashTimer(Timer::from_seconds(3.0, TimerMode::Repeating)));
}