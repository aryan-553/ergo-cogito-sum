use bevy::prelude::*;
use crate::GameState;


pub struct LobbyHostPlugin;
#[derive(Component)] 
struct BackButton;

impl Plugin for LobbyHostPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::LobbyHost), setup_lobby_host_ui)
        .add_systems(Update, button_interaction_system.run_if(in_state(GameState::LobbyHost))) 
        .add_systems(OnExit(GameState::LobbyHost), cleanup_lobby_host);
    }
}

// System to setup the lobby UI
fn setup_lobby_host_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let font = asset_server.load("fonts/Debrosee-ALPnL.ttf");

    // Setup basic lobby UI with a scrollable list
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                position_type: PositionType::Relative,
                overflow: Overflow {
                    x: OverflowAxis::Visible,  // Set overflow behavior for the x-axis
                    y: OverflowAxis::Visible,  // Set overflow behavior for the y-axis
                },
                direction: Direction::Inherit, // Inherit the direction from the parent (default)
                left: Val::Auto,  // Use 'Auto' if positioning is flexible
                right: Val::Auto, // You can set these for absolute positioning if needed
                top: Val::Auto,
                bottom: Val::Auto,
                justify_content: JustifyContent::Center, // Centers the contents in the container
                align_items: AlignItems::Center,         // Aligns items along the cross axis (center in this case)
                flex_direction: FlexDirection::ColumnReverse, // Lays out the children from bottom to top
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(ButtonBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: crate::consts::NORMAL_BUTTON.into(),
                ..Default::default()
            })
            .insert(BackButton) // Insert the BackButton component
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Back to Create Room Menu",
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                ));
            });
            for i in 0..5 {
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            display: Display::Flex,
                            position_type: PositionType::Relative,
                            overflow: Overflow {
                                x: OverflowAxis::Visible,  // Set overflow behavior for the x-axis
                                y: OverflowAxis::Visible,  // Set overflow behavior for the y-axis
                            },
                            direction: Direction::Inherit,
                            width: Val::Percent(100.0),   // 100% width for each button
                            height: Val::Px(50.0),        // Fixed height of 50px for each button
                            margin: UiRect::all(Val::Px(5.0)), // Margin around each button
                            justify_content: JustifyContent::Center, // Centers text inside the button
                            align_items: AlignItems::Center,         // Centers text vertically
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            format!("Host_Lobby {}", i + 1),
                            TextStyle {
                                font: font.clone(),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ));
                    });
            }
        });
}

fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&BackButton>),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, back_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if back_button.is_some() {
                    println!("Back Button Clicked");
                    game_state.set(GameState::CreateRoom); 
                }
            }
            Interaction::Hovered => {
                *color = crate::consts::HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = crate::consts::NORMAL_BUTTON.into();
            }
        }
    }
}

// System to cleanup the lobby UI when exiting the Lobby state
fn cleanup_lobby_host(mut commands: Commands, query: Query<Entity, With<Node>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}