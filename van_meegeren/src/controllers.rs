use druid::{Event, EventCtx, Cursor, KeyEvent, Env, Widget, LensExt};
use druid::widget::{Controller};
use druid::keyboard_types::{Code};

use crate::data::AppState;
use crate::data::InputData;

pub struct EnterExitController;
impl<W: Widget<AppState>> Controller<AppState, W> for EnterExitController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            Event::KeyDown(KeyEvent { code, .. } ) => {
                match code {
                    druid::keyboard_types::Code::Enter => {
                        match data.input_data.counter {
                            0 => {
                                data.input_data.token = data.input.clone();
                                data.console.insert(0, format!("Ваш токен: [ {} ]", data.input.clone()));
                                data.input_data.counter += 1;
                            }
                            1 => {
                                data.input_data.wanted_id = data.input.clone();
                                data.console.insert(0, format!("Желаемый сервер: [ {} ]", data.input.clone()));
                                data.input_data.counter += 1;
                            }
                            _ => {
                                data.input_data.your_id = data.input.clone();
                                data.console.insert(0, format!("Ваш сервер: [ {} ]", data.input.clone()));
                                data.input_data.counter = 0;
                            }
                        }

                        data.input = String::new();
                        ctx.resign_focus();
                    },
                    druid::keyboard_types::Code::Escape => {
                        ctx.resign_focus();
                    }
                    _ => {}
                }
            }
            _  => {}
        }

        child.event(ctx, event, data, env);
    }
}

pub struct Hover;
impl<W: Widget<AppState>> Controller<AppState, W> for Hover {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        // match event {
            
        // }
    }
}