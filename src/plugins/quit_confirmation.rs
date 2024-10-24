use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::consts; // Make sure to include your constants module

pub struct QuitConfirmationPlugin;
#[derive(Component)]
pub struct QuitConfirmationPopup;

#[derive(Component)]
pub struct ConfirmButton;

#[derive(Component)]
pub struct CancelButton;

pub fn setup_confirm_quit_popup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let _window: &Window = window_query.get_single().unwrap();

    commands.spawn(NodeBundle {
        style: Style {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            width: Val::Percent(50.0),
            height: Val::Percent(30.0),
            ..Default::default()
        },
        background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(), // Semi-transparent background
        ..Default::default()
    })
    .insert(QuitConfirmationPopup)
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Are you sure you want to quit?",
            TextStyle {
                font: asset_server.load("fonts/Debrosee-ALPnL.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        ));

        parent.spawn(ButtonBundle {
            style: Style {
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                ..Default::default()
            },
            background_color: consts::NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(ConfirmButton)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Yes, Quit",
                TextStyle {
                    font: asset_server.load("fonts/Debrosee-ALPnL.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ));
        });

        parent.spawn(ButtonBundle {
            style: Style {
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                ..Default::default()
            },
            background_color: consts::NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(CancelButton)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Cancel",
                TextStyle {
                    font: asset_server.load("fonts/Debrosee-ALPnL.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ));
        });
    });
}

pub fn handle_confirmation_buttons(
    mut interaction_query: Query<(
        &Interaction,
        &mut BackgroundColor,
        Option<&ConfirmButton>,
        Option<&CancelButton>,
    ), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut commands: Commands,
    query: Query<Entity, With<QuitConfirmationPopup>>, // Query to find the popup
) {
    for (interaction, mut color, confirm_button, cancel_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if confirm_button.is_some() {
                    println!("Confirmed Quit");
                    app_exit_events.send(AppExit::Success); // Send exit event
                } else if cancel_button.is_some() {
                    println!("Cancelled Quit");
                    for entity in query.iter() {
                        commands.entity(entity).despawn_recursive(); // Remove the popup
                    }
                }
            }
            Interaction::Hovered => {
                *color = consts::HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = consts::NORMAL_BUTTON.into();
            }
        }
    }
}
