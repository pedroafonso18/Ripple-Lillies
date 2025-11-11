use bevy::prelude::*;
use crate::actors::lillies::Lily;
const FROG_VELOCITY : f32 = 550.;

#[derive(Component)]
pub struct Frog{
    pub velocity : f32,
    pub direction : Vec3,
    pub position : u8,
}

pub fn update_frog(
    mut frog_query : Query<(&mut Frog, &mut Transform)>,
    lily_query: Query<(&Transform, &Lily)>,
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

        if new_pos != player.position {
            player.position = new_pos;
            for (lily_tf, lily_comp) in lily_query.iter() {
                if lily_comp.grid_pos == new_pos {
                    transform.translation = lily_tf.translation;
                    break;
                }
            }
        }
    }
}