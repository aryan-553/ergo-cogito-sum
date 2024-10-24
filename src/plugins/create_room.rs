use bevy::prelude::*;
use crate::GameState;
use crate::consts;
use bevy::window::PrimaryWindow;

pub struct RoomCreator;

#[derive(Component)]
struct OnRoomCreatorScreen;

#[derive(Component)]
struct PublicButton;

#[derive(Component)]
struct PrivateButton;

#[derive(Component)]
struct BackToMainMenuButton;

impl Plugin for RoomCreator {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::CreateRoom),setup_room_selector)
        .add_systems(Update, room_button_interaction_system.run_if(in_state(GameState::CreateRoom)))
        .add_systems(OnExit(GameState::CreateRoom),cleanup_menu);
    }
}

// System to setup the main menu UI
fn setup_room_selector(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let _window: &Window = window_query.get_single().unwrap();
    // UI setup with Public and Private Room buttons
    
    commands
        .spawn((NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        },
        OnRoomCreatorScreen,))
        .with_children(|parent| {
            parent
                // Public Room Button
                .spawn(ButtonBundle {
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
                .insert(PublicButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Public Room",
                        TextStyle {
                            font: asset_server.load("fonts/Debrosee-ALPnL.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ));
                    
                });

            parent
                // Private Button
                .spawn(ButtonBundle {
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
                .insert(PrivateButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Private Room",
                        TextStyle {
                            font: asset_server.load("fonts/Debrosee-ALPnL.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ));

                });
            // Back Button
            parent.spawn(ButtonBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Relative,
                    ..Default::default()
                },
                background_color: crate::consts::NORMAL_BUTTON.into(),
                ..Default::default()
            })
            .insert(BackToMainMenuButton) // Insert the BackButton component
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Back to Main Menu",
                    TextStyle {
                        font: asset_server.load("fonts/Debrosee-ALPnL.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                ));
            });
        });
}

// System to handle button interaction
fn room_button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&PublicButton>, Option<&PrivateButton>, Option<&BackToMainMenuButton>),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>
) {
    for (interaction, mut color, public_button, private_button, back_to_main_menu) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if public_button.is_some() {
                    println!("Public Game Button Clicked");// Switch to Lobby state
                    game_state.set(GameState::LobbyHost);
                } else if private_button.is_some() {
                    println!("Pivate Game Button Clicked");// Switch to Lobby state
                    game_state.set(GameState::LobbyHost);
                } else if back_to_main_menu.is_some() {
                    println!("Back Button Clicked"); // Switch to MainMenu state
                    game_state.set(GameState::MainMenu);
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

// System to cleanup menu when exiting MainMenu state
fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<OnRoomCreatorScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}