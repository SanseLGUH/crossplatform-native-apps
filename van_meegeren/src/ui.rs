use druid::{
    widget::{List, LineBreaking, Scroll, Flex, Label, Align, TextBox, SizedBox, Checkbox, Controller, Container, Button},
    AppLauncher, WindowDesc, Color, Widget, WidgetExt, Env,
    FontDescriptor, FontWeight, KeyEvent, Data, Lens, LensExt,ExtEventSink,
};
use crate::data::{AppState, ToggleSettings, GRAY_COLOR, BLUE_COLOR, DARK_COLOR};
use crate::controllers::{EnterExitController};


use crate::fetching;

use tokio::sync::{Mutex, Notify};
use std::sync::Arc;

use tokio::task;
use druid::im::Vector;

fn console() -> impl Widget<AppState> {
    Scroll::new(List::new(|| {
        Label::new(|item: &String, _env: &Env| item.clone())
            .with_line_break_mode(LineBreaking::WordWrap)
        }).padding(3.))
        .vertical()
        .lens(AppState::console)
        .fix_size(445.0, 435.0)
        .background(Color::from_hex_str(GRAY_COLOR).unwrap())
}

fn toggle_settings() -> impl Widget<AppState> {
    Container::new(
        SizedBox::new(
            Align::centered(Flex::row()
                .with_child(Flex::column()
                    .with_child(Checkbox::new("Копировать каналы")
                            .lens(AppState::settings.then(ToggleSettings::copy_channels)).align_left()).with_spacer(5.)
                    .with_child(Checkbox::new("Копировать роли")
                                .lens(AppState::settings.then(ToggleSettings::copy_roles)).align_left()).with_spacer(5.)
                    .with_child(Checkbox::new("Очистка перед копированием")
                                .lens(AppState::settings.then(ToggleSettings::clean_up)).align_left())
                    .fix_size(100., 70.).padding( (0., 0., 60., 0.) )
                )
                
                .with_child(
                    Flex::column()
                        .with_child(Checkbox::new("Параметры")
                            .lens(AppState::settings.then(ToggleSettings::copy_params_roles)).align_left().padding( (0., 0., 0., 5.) ))
                        .fix_size(100., 70.)
                ))
        )
        .fix_width(275.0)
        .background(Color::from_hex_str(GRAY_COLOR).unwrap())
    ).border(Color::from_hex_str(BLUE_COLOR).unwrap(), 1.)
}

fn run_button() -> impl Widget<AppState> {
    SizedBox::new(Label::new("СКОПИРОВАТЬ")
            .with_text_color(Color::from_hex_str(GRAY_COLOR).unwrap())
            .with_text_size(20.).center())
        .fix_size(160., 45.)
        .background(Color::from_hex_str(BLUE_COLOR).unwrap())
        .on_click(|ctx, data: &mut AppState, _| {
            let mut count: u8 = 0;
            if count == 0 {
                count += 1;
                let progress_data = Arc::new(Mutex::new("".to_string()));
                let notify = Arc::new(Notify::new());

                let ext_ctx = ctx.get_external_handle();

                let progress_data_ref = progress_data.clone();
                let notify_ref = Arc::clone(&notify);

                let token = data.input_data.token.clone();
                let your_id = data.input_data.your_id.clone();
                let wanted_id = data.input_data.wanted_id.clone();
                let toggle = data.settings.clone();

                task::spawn(    
                    async move {
                        let mut run_event = fetching::FetchingEvent::new(progress_data_ref,
                            notify_ref, your_id, wanted_id, token);

                        if toggle.clean_up {
                            run_event.clean_up().await;                            
                        }

                        if toggle.copy_params_roles {
                            run_event.copy_roles().await;
                        }

                        if toggle.copy_channels {
                            run_event.copy_channels(toggle.copy_params_roles).await;                               
                        }
                });

                let notify_ref_two = Arc::clone(&notify);
                let progress_data_ref_two = progress_data.clone();

                let streamout = task::spawn(async move {
                    loop {
                        notify_ref_two.notified().await;

                        let shared_data_lock = progress_data_ref_two.lock().await;
                        let progress_data = shared_data_lock.clone();

                        ext_ctx.add_idle_callback(move |data: &mut AppState| {
                            data.console.insert(0, progress_data);
                        });
                    }
                });
            }
         })
}

fn input_for_all_things() -> impl Widget<AppState> { 
    TextBox::new()
        .with_placeholder("Пиши здесь!").with_text_size(22.).lens(AppState::input)
        .fix_size(160., 40. )
        .background(Color::from_hex_str(DARK_COLOR).unwrap())
        .controller(EnterExitController)
}

pub fn ui_builder() -> impl Widget<AppState> {
    Flex::column()
        .with_child(console().padding((0., 16., 0. , 16.)))
        .with_child(
            Flex::row()
                .with_child(toggle_settings().padding((0., 0., 7., 0.)))
                .with_child(
                    Flex::column()
                        .with_child(input_for_all_things().padding((0. , 0. , 0. , 5.)))
                        .with_child(run_button())
                ).fix_size( 445., 90.)
        )
        .background(Color::from_hex_str(DARK_COLOR).unwrap())
}