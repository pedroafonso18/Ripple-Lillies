use bevy::prelude::*;
use crate::actors::lillies::Lily;

const FROG_VELOCITY : f32 = 550.;
const PIXEL_RATIO : f32 = 3.;

#[derive(Component)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct Frog{
    pub velocity : f32,
    pub position : u8,
    pub health : u8
}

pub fn update_frog(
    mut frog_query : Query<(&mut Frog, &mut Transform), Without<Lily>>,
    lily_query: Query<(&mut Lily, &mut Transform), Without<Frog>>,
    keys : Res<ButtonInput<KeyCode>>,
)
{
    const GRID: u8 = 5;

    if let Ok((mut player, mut transform)) = frog_query.single_mut(){
        player.velocity = FROG_VELOCITY;

        let mut new_pos = player.position;

        let row = new_pos / GRID;
        let col = new_pos % GRID;

        if keys.just_pressed(KeyCode::KeyA) {
            if col > 0 {
                new_pos = new_pos - 1;
            }
        }
        if keys.just_pressed(KeyCode::KeyD) {
            if col < GRID - 1 {
                new_pos = new_pos + 1;
            }
        }
        if keys.just_pressed(KeyCode::KeyW) {
            if row > 0 {
                new_pos = new_pos - GRID;
            }
        }
        if keys.just_pressed(KeyCode::KeyS) {
            if row < GRID - 1 {
                new_pos = new_pos + GRID;
            }
        }
        let mut pos_changed : bool = false;
        if new_pos != player.position {
            for (lily_comp, lily_tf) in lily_query.iter() {
                if lily_comp.grid_pos == new_pos {
                    transform.translation = lily_tf.translation;
                    transform.translation.z = 1.;
                    pos_changed = true;
                    break;
                }
            }
            if pos_changed {
                player.position = new_pos;
            }
        }
    }
}

pub fn setup_frog(
    commands : &mut Commands,
    asset_server : &Res<AssetServer>,
)
{
    commands.spawn((
        Sprite{
            image: asset_server.load("frog-front1.png"),
            ..Default::default()
        },
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)), 
        Transform::from_xyz(0.0, 0.0, 1.0).with_scale(Vec3::splat(PIXEL_RATIO)),
        Frog{ velocity: 3.0, position: 5, health: 3}
    ));

}