use bevy::prelude::*;
use plugins::game_runner::GameRunnerPlugin;
use plugins::lobby::LobbyPlugin;
use plugins::lobby_host::LobbyHostPlugin;
use plugins::main_menu::MainMenuPlugin;
use plugins::create_room::RoomCreator;

mod components;
mod resources;
mod systems;
mod plugins;
mod consts; 

#[derive(Debug, Eq, PartialEq, Hash, Resource, States, Default, Clone)]
enum GameState {
    #[default]
    MainMenu,
    Lobby,
    CreateRoom,
    LobbyHost,  
    InGame,
}
 
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins((GameRunnerPlugin,MainMenuPlugin,LobbyPlugin,LobbyHostPlugin,RoomCreator))
        .run();
}