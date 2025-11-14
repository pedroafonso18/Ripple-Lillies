use bevy::prelude::*;
use rand::{self, Rng};
use crate::gamestate::gamestate::get_coordinate_grid;
use crate::actors::frog::Frog;
use crate::actors::lillies::Lily;

#[derive(Component)]
pub struct Trash{
   pub grid_pos : u8 
}

#[derive(Component)]
pub struct TrashTimer(Timer);

#[derive(Resource)]
pub struct TrashSpawnTimer(Timer);

pub fn spawn_trash(
    mut commands : Commands,
    asset_server : Res<AssetServer>,
    time : Res<Time>,
    mut timer : ResMut<TrashSpawnTimer>,
)
{
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::rng();
        let trash_number = rng.random_range(1..4);
        let spawn_tile = rng.random_range(0..25);
        let image_str = format!("trash{}.png", trash_number);
        let pos = get_coordinate_grid(&spawn_tile);
        commands.spawn((
            Sprite{
                image : asset_server.load(image_str),
                ..Default::default()
            },
            Trash{ grid_pos: spawn_tile },
            TrashTimer(Timer::from_seconds(5.0, TimerMode::Once)), 
            Transform::from_xyz(pos.x,pos.y,pos.z).with_scale(Vec3::splat(3.))
        ));
    }
}

pub fn setup_trash(
    commands : &mut Commands,
)
{
    commands.insert_resource(TrashSpawnTimer(Timer::from_seconds(3.0, TimerMode::Repeating)));
}

pub fn move_trash(
    keys : Res<ButtonInput<KeyCode>>,
    mut commands : Commands,
    mut trash_query : Query<(Entity, &mut Trash, &mut Transform), Without<Frog>>,
    frog_query : Query<&mut Frog, Without<Trash>>
)
{
    if let Ok(player) = frog_query.single(){
        let old_pos = player.position;

        const GRID: u8 = 5;

        let row = old_pos / GRID;
        let col = old_pos % GRID;

        let mut new_pos: Option<u8> = None;

        if keys.just_pressed(KeyCode::KeyW) {
            if row > 0 {
                new_pos = Some(old_pos - GRID);
            } else {
                new_pos = Some(0);
            }
        }
        if keys.just_pressed(KeyCode::KeyS) {
            if row < GRID - 1 {
                new_pos = Some(old_pos + GRID);
            } else {
                new_pos = Some(0);
            }
        }
        if keys.just_pressed(KeyCode::KeyD) {
            if col < GRID - 1 {
                new_pos = Some(old_pos + 1);
            } else {
                new_pos = Some(0);
            }
        }
        if keys.just_pressed(KeyCode::KeyA) {
            if col > 0 {
                new_pos = Some(old_pos - 1);
            } else {
                new_pos = Some(0);
            }
        }

        if let Some(new_pos_val) = new_pos {
            for (entity, mut trash, mut transform) in trash_query.iter_mut() {
                if trash.grid_pos == old_pos {
                    if new_pos_val == 0 {
                        commands.entity(entity).despawn();   
                    } else {
                        trash.grid_pos = new_pos_val;
                        transform.translation = get_coordinate_grid(&new_pos_val);
                    } 
                }
            }
        }
    }
}

pub fn remove_trash(
    mut commands: Commands,
    time: Res<Time>,
    mut trash_query: Query<(Entity, &mut TrashTimer, &mut Trash)>,
    mut lily_query: Query<(Entity, &mut Lily)>
)
{
    for (entity, mut timer, trash) in trash_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
            for (lily_entity, lily) in lily_query.iter_mut() {
                if lily.grid_pos == trash.grid_pos {
                    commands.entity(lily_entity).despawn();
                }
            }
        }
    }
}