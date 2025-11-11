use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score{
    pub value : u32
}

#[derive(Resource)]
pub struct ScoreTimer(Timer);

#[derive(Component)]
pub struct ScoreText;

pub fn update_score(
    time : Res<Time>,
    mut timer : ResMut<ScoreTimer>,
    mut score : ResMut<Score>,
)
{
    if timer.0.tick(time.delta()).just_finished() {
        score.value += 1;
    }
}


pub fn update_score_text(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        if let Ok(mut text) = query.single_mut() {
            text.0 = format!("{}",score.value);
        }
    }
}

pub fn setup_score(
    commands : &mut Commands,
    asset_server : &Res<AssetServer>
)
{
    commands.insert_resource(Score::default());
    commands.insert_resource(ScoreTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));

    commands.spawn((
        Text::new("0"),
        TextFont {
            font : asset_server.load("scorefont.ttf"),
            font_size: 40.,
            ..Default::default()
        },
        TextColor {
            0 : Color::WHITE
        }
    ))
    .insert(ScoreText);
}