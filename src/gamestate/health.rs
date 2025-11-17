use bevy::ui::Val;
use crate::actors::frog::Frog;
use bevy::prelude::*;

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
                align_items: AlignItems::End,
                padding: UiRect::all(Val::Px(20.)),
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
                        padding: UiRect::all(Val::Px(20.)),
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