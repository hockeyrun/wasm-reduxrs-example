pub mod state;
mod settings;
mod game;

use redux_rs::{Store, combine_reducers};
use cgmath::Vector2;

use crate::maps::World;
use crate::theme::ThemeMode;
use crate::models::geometry::Size;

use state::{State, OnChangeCallback, state_reducer};
use settings::settings_reducer;
use game::game_reducer;

pub static DEFAULT_WORLD_SCALE: i32 = 2;
pub static DEFAULT_TILE_SIZE: Size<i32> = Size{w: 16, h: 16};
pub static DEFAULT_RESOLUTION: Size<i32> = Size{w: 800, h: 600};

pub enum Actions {
    AppValidateTest(bool),
    AppSetOnChangeCallback(OnChangeCallback),
    SettingsSetThemeMode(ThemeMode),
    SettingsSetScale(i32),
    SettingsSetResolution(Size<i32>),
    GameHandleKey(char),
    GameSetElapsedTime(f64),
    GameSetGameCursor(Vector2<i32>),
    GameSetWorld(World),
    GameSetViewForPosition(Vector2<i32>),
}

pub fn create_store() -> Store<State, Actions> {
    let reducers = combine_reducers!(
        State,
        &Actions,
        state_reducer,
        settings_reducer,
        game_reducer
    );
    Store::new(reducers, State::default())
}
