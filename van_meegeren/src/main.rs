use druid::{WindowDesc, AppLauncher, 
    theme::{BACKGROUND_DARK, 
        BACKGROUND_LIGHT, 
        TEXTBOX_BORDER_WIDTH, 
        BORDER_LIGHT,
        SELECTED_TEXT_BACKGROUND_COLOR,
        BUTTON_LIGHT, BUTTON_DARK}, 
    im::Vector, Color};

mod ui;
mod data;
mod controllers;
mod fetching;

use crate::data::{AppState, 
    ToggleSettings, InputData, RED_COLOR, BLUE_COLOR, 
    GRAY_COLOR, DARK_COLOR};
use crate::ui::ui_builder;

#[tokio::main]
async fn main() {
    let window = WindowDesc::new(ui_builder())
        .title("VanMeegeren")
        .window_size( (500., 616.) )
        .resizable(false);

    let initial_state = AppState {
        input: String::new(),
        mini_logs: String::new(),
        console: vec!["Вначале необходимо ввести токен, затем ID желаемого сервера, а после — ID вашего сервера.".to_string()].into(),
        settings: ToggleSettings {
            clean_up: false,
            copy_roles: true,
            copy_channels: true,
            copy_params_roles: false,
        },
        input_data: InputData {
            counter: 0,
            token: "".to_string(),
            your_id: "".to_string(),
            wanted_id: "".to_string(),
        }
    };

    AppLauncher::with_window(window)
        .configure_env(|env, _state| {
            env.set(BUTTON_LIGHT, Color::from_hex_str(GRAY_COLOR).unwrap());
            env.set(BUTTON_DARK, Color::from_hex_str(GRAY_COLOR).unwrap());
            env.set(BACKGROUND_DARK, Color::from_hex_str(BLUE_COLOR).unwrap());
            env.set(BACKGROUND_LIGHT, Color::from_hex_str(BLUE_COLOR).unwrap());
            env.set(BORDER_LIGHT, Color::from_hex_str(BLUE_COLOR).unwrap());
            env.set(SELECTED_TEXT_BACKGROUND_COLOR, Color::from_hex_str(GRAY_COLOR).unwrap());
        }).launch( initial_state )
        .expect("Failed to launch application");
}