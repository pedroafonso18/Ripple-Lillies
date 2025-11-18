use bevy::ui::Val;
use crate::actors::frog::Frog;
use crate::gamestate::gamestate::GameState;
use bevy::prelude::*;
use bevy::text::Justify::Center;
use crate::gamestate::score::ScoreText;

#[derive(Component)]
pub struct HeartsUiRoot;

pub fn setup_hearts(
    mut commands: Commands,
    asset_server: Res<AssetServer>
)
{
    commands.spawn((
            Node {
                justify_content: JustifyContent::End,
                justify_items: JustifyItems::End,
                align_items: AlignItems::End,
                ..Default::default()
            },
            HeartsUiRoot,
    ))
        .with_children(|parent| {
            for _ in 0..3 {
                parent.spawn((
                    ImageNode {
                        image: asset_server.load("HEARTH.png"),
                        ..Default::default()
                    },
                    Node {
                        justify_content: JustifyContent::End,
                        align_items: AlignItems::End,
                        padding: UiRect::all(Val::Px(32.)),
                        ..Default::default()
                    }
                ));
            }
        });
}

pub fn update_hearts_ui(
    frog_query: Query<&Frog>,
    mut ui_query: Query<&mut Visibility, (With<ImageNode>, Without<Frog>)>,
) {
    if let Ok(frog) = frog_query.single() {
        let health = frog.health as usize;

        for (i, mut visibility) in ui_query.iter_mut().enumerate() {
            if i < health {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

pub fn update_gamestate_on_death(
    frog_query: Query<&Frog>,
    state : Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
)
{
    if state.get() == &GameState::Playing {
        if let Ok(frog) = frog_query.single() {
            if frog.health == 0 {
                next_state.set(GameState::Dead);
            }
        }
    }
}

pub fn display_death_text(
    mut commands : Commands,
    asset_server : Res<AssetServer>
)
{
    commands.spawn((
        Text::new("You died."),
        TextFont {
            font : asset_server.load("scorefont.ttf"),
            font_size: 100.,
            ..Default::default()
        },
        TextColor {
            0 : Color::WHITE
        },
        Node {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            ..Default::default()
        }
    ));
}