use tokio::sync::{Mutex, Notify};
use std::sync::Arc;
use tokio::task;
use tokio::fs;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use reqwest::Client;
use futures::StreamExt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use zip::ZipArchive;
use std::path::Path;
use tokio::fs::File;
use tokio::process::Command;

#[derive(Serialize, Deserialize)]
struct Metadata {
    metadata_url: String,
    update_details: UpdateDetails,
}

#[derive(Serialize, Deserialize)]
struct UpdateDetails {
    drivers_version: f32,
    major_update_version: f32,

    drivers_files: Option<std::collections::HashMap<String, DriverFiles>>,
    major_update_files: Option<std::collections::HashMap<String, MajorUpdateFiles>>,

    executable: Option<ExecutableSettings>,
}

#[derive(Serialize, Deserialize, Clone)]
struct DriverFiles {
    url: String,
    driver_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct MajorUpdateFiles {
    url: String,
    folder_name: String,
    delta: bool,
}

#[derive(Serialize, Deserialize)]
struct ExecutableSettings {
    executable_path: String,
    use_token_generator: bool,
}

impl Metadata {
    fn default() -> Self {
        let default_update_details = UpdateDetails {
            drivers_version: 0.0,
            major_update_version: 0.0,
            drivers_files: None,
            major_update_files: None,
            executable: None,
        };

        Metadata {
            metadata_url: "https://raw.githubusercontent.com/SanseL4462/zone-launcher-data/refs/heads/main/data.json".to_string(),
            update_details: default_update_details,
        }
    }
}

// downloading thread 

struct Downloader {
    shared_data: Arc<Mutex<f64>>, 
    notify: Arc<Notify>, 
}

impl Downloader {
    fn new(shared_data: Arc<Mutex<f64>>, notify: Arc<Notify>) -> Self {
        Downloader { shared_data, notify }
    }

	async fn handle_bad_connection(url: &str) -> bool {
	    let client = reqwest::Client::builder()
	        .timeout(std::time::Duration::from_secs(10))
	        .build()
	        .unwrap();

	    let response = client.get(url).send().await;

	    match response {
	        Ok(resp) => resp.status().is_success(),
	        Err(_) => false,
	    }
	} // can be changed

	async fn handle_delta_updates(&self, url: &str, bin_dir: &str) {
	} // can be changed

    async fn stream_downloading(&self, url: &str, dir: &str) {
        let client = Client::new();
        let response = client.get(url).send().await.unwrap();

        let total_size = response
            .headers()
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|header| header.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        let mut file = File::create(dir).await.unwrap();
        let mut downloaded_size: f64 = 0.0;

        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(data) => {
                    if let Err(e) = file.write_all(&data).await {
                        eprintln!("Error writing to file: {}", e);
                        return;
                    }

                    downloaded_size += data.len() as f64;

                    println!("{}", downloaded_size / total_size as f64);

                    let mut shared_data_1 = self.shared_data.lock().await; 
                    *shared_data_1 = downloaded_size / total_size as f64;
                    self.notify.notify_one();
                }
                Err(e) => {
                    eprintln!("Error reading chunk: {}", e);
                    return;
                }
            }
        }
    } // fundomental cannot be changed so i need to make this work well

    async fn should_install_driver(driver_name: &str) -> bool {
        let driver = Command::new(driver_name)
            .output();

        match driver.await {
            Ok(_) => false,
            Err(_) => true,
        }
    } // can be changed

    async fn extract_zip_async(file_path: &str, dest_dir: &str) {
        let file_path = file_path.to_string();
        let dest_dir = dest_dir.to_string();
        fs::create_dir(&dest_dir).await.unwrap();

        task::spawn_blocking(move || {

            if !std::path::Path::new(&file_path).exists() {
                panic!("ZIP file does not exist at path: {}", file_path);
            }

            let file = match std::fs::File::open(&file_path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Failed to open ZIP file {}: {}", file_path, e);
                    return;
                }
            };

            let mut archive = match ZipArchive::new(file) {
                Ok(a) => a,
                Err(e) => {
                    eprintln!("Failed to read ZIP archive {}: {}", file_path, e);
                    return;
                }
            };
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).unwrap();
                let file_name = file.name().to_string();

                let out_path = Path::new(&dest_dir).join(&file_name);

                if file.name().ends_with('/') {
                    std::fs::create_dir_all(&out_path).unwrap();
                } else {
                    let mut out_file = std::fs::File::create(out_path).unwrap();
                    std::io::copy(&mut file, &mut out_file).unwrap();
                }
            }
        })
        .await
        .unwrap();
    } // can be changed

    pub async fn install_drivers(&self, url: &str, file_name: &str, driver_name: &str) {
        if Downloader::should_install_driver(driver_name).await {
            self.stream_downloading(url, file_name).await;

            let mut attempt = 0;
            let max_attempts = 3;

            while attempt < max_attempts {
                let result = Command::new(format!("./{}", file_name))
                    .status()
                    .await;

                match result {
                    Ok(_) => {
                        break;
                    }
                    Err(e) => {
                        if attempt < max_attempts - 1 {
                            eprintln!("Error occurred: {:?}, retrying in 3 seconds...", e);
                            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                        } else {
                            eprintln!("Failed after {} attempts: {:?}", max_attempts, e);
                        }
                    }
                }

                attempt += 1;
            }

            fs::remove_file(file_name).await.unwrap();
        }
    } // can be changed

    pub async fn apply_major_update(&self, url: &str, file_name: &str, folder_name: &str) {
        println!("apply_major_update");
        if let Err(e) = fs::remove_dir_all(folder_name).await {
            if e.kind() != std::io::ErrorKind::NotFound {
                panic!("Failed to remove directory: {:?}", e);
            }
        }

        self.stream_downloading(url, file_name).await;
        Downloader::extract_zip_async(file_name, folder_name).await;
        fs::remove_file(file_name).await.unwrap();
    } // can be changed
}

async fn local_metadata() -> Metadata {
    let mut contents = String::new();

    if let Ok(mut file) = fs::File::open("metadata.json").await {

        if let Err(_) = file.read_to_string(&mut contents).await {
            return Metadata::default();
        };

    } else {
        return Metadata::default();
    }

    serde_json::from_str::<Metadata>(&contents).unwrap_or(Metadata::default()) 
}

async fn fetch_metadata(url: &str) -> Metadata {
    let client = Client::new();

    let response = match client.get(url).send().await {
        Ok(resp) => resp,
        Err(_) => return Metadata::default(),
    };

    let json: Value = match response.json().await {
        Ok(json) => json,
        Err(_) => return Metadata::default(), 
    };

    serde_json::from_value(json).unwrap_or_else(|e| {println!("{}", e); Metadata::default()})
}

use sha2::{Sha256, Digest};

async fn execute_launcher_token(executable_path: &str) {
    let mut hasher = Sha256::new();
    hasher.update("Easy way by SanseL".as_bytes());
    let result = hasher.finalize();

	Command::new(executable_path).arg(format!("{:x}", result));
} // beta 

async fn execute_launcher(execute_path: &str) {
    Command::new(execute_path);
}

// MAIN THREAD

use druid::widget::prelude::*;
use druid::widget::{Align, Svg, SvgData, Label, Controller, Flex, SizedBox};
use druid::{Screen, ExtEventSink, Point, AppLauncher, Color, Data, Lens, Rect, RenderContext, Widget, WidgetExt, WindowDesc};
use druid::commands::QUIT_APP;

fn truncate_with_ellipsis(s: &str, max_len: usize) -> String {
    if s.chars().count() > max_len {
        let mut truncated: String = s.chars().take(max_len - 3).collect();
        truncated.push_str("...");
        truncated
    } else {
        s.to_string()
    }
}

fn bridge(ext_ctx: ExtEventSink) {
    task::spawn(async move {
        let shared_data = Arc::new(Mutex::new(0.0));
        let notify = Arc::new(Notify::new());
        let downloader = Arc::new(Mutex::new(Downloader::new(shared_data.clone(), notify.clone())));
        
        let local_metadata = local_metadata().await;
        let fetch_metadata = fetch_metadata(&local_metadata.metadata_url.clone()).await;

        let shared_data_clone = Arc::clone(&shared_data);
        let notify_clone = Arc::clone(&notify);
        let clone_ext_ctx = ext_ctx.clone();
        let streamout = tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                notify_clone.notified().await;

                let shared_data_lock = shared_data_clone.lock().await;
                let progress_value = *shared_data_lock;

                clone_ext_ctx.add_idle_callback(move |data: &mut AppState| {
                    data.progress = progress_value;
                });
            }
        });

        let downloader = downloader.lock().await;

        let metadata_url = fetch_metadata.metadata_url.clone();
        ext_ctx.add_idle_callback(move |data: &mut AppState| {
            data.progress_message = "Сверяем метаданные".to_string();
            data.detail_progress_message = format!("{}", truncate_with_ellipsis(&metadata_url, 40));
        });

        tokio::time::sleep(std::time::Duration::from_millis(130)).await;

        ext_ctx.add_idle_callback(move |data: &mut AppState| {
            data.progress = 1.0;
        });

        if !Downloader::handle_bad_connection(&fetch_metadata.metadata_url).await {
            ext_ctx.add_idle_callback(move |data: &mut AppState| {
                data.progress_message = "Сверяем метаданные! [Плохое подключение]".to_string();
            });   
        }

        tokio::time::sleep(std::time::Duration::from_millis(130)).await;

        if local_metadata.update_details.drivers_version != fetch_metadata.update_details.drivers_version {
            let drivers_files = fetch_metadata.update_details.drivers_files.clone().unwrap();
            for (file_name, info) in drivers_files {
                let driver_url_clone = info.url.clone();
                ext_ctx.add_idle_callback(move |data: &mut AppState| {
                    data.progress_message = "Установка драйверов!".to_string();
                    data.detail_progress_message = format!("{}", truncate_with_ellipsis(&driver_url_clone, 40));
                    data.progress = 0.0;
                });

               	if !Downloader::handle_bad_connection(&info.url).await {
            		ext_ctx.add_idle_callback(move |data: &mut AppState| {
			            data.progress_message = "Установка драйверов! [Плохое подключение]".to_string();
			        });
            	} 

            	downloader.install_drivers(&info.url, &file_name, &info.driver_name).await;
            }
        }

        if local_metadata.update_details.major_update_version != fetch_metadata.update_details.major_update_version {
            let major_files = fetch_metadata.update_details.major_update_files.clone().unwrap();
            for (file_name, info) in major_files {

                let major_url_clone = info.url.clone();
                ext_ctx.add_idle_callback(move |data: &mut AppState| {
                    data.progress_message = "Установка основных файлов!".to_string();
                    data.detail_progress_message = format!("{}", truncate_with_ellipsis(&major_url_clone, 40));
                    data.progress = 0.0;
                });

                if !Downloader::handle_bad_connection(&info.url).await {
                	ext_ctx.add_idle_callback(move |data: &mut AppState| {
			            data.progress_message = "Установка основных файлов! [Плохое подключение]".to_string();
			        });
                }

                downloader.apply_major_update(&info.url, &file_name, ".zone-launcher").await;
            }
        }

        streamout.abort();

        let json = serde_json::to_string_pretty(&fetch_metadata).unwrap();
        let mut metadata_file = fs::File::create("metadata.json").await.unwrap();
        metadata_file.write_all(json.as_bytes()).await.unwrap();

        ext_ctx.add_idle_callback(move |data: &mut AppState| {
            data.progress_message = "Приятной игры!".to_string();
            data.detail_progress_message = "Поддержите создателя лаунчера [ .4462 ]".to_string();
            data.progress = 1.0;
        });

        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        if fetch_metadata.update_details.executable.as_ref().unwrap().use_token_generator {
            execute_launcher_token(&fetch_metadata.update_details.executable.unwrap().executable_path).await;
        } else {
            execute_launcher(&fetch_metadata.update_details.executable.unwrap().executable_path).await;
        }
        
        std::process::exit(0);
    });
}

#[derive(Clone, Data, Lens)]
struct AppState {
    progress: f64,
    progress_message: String,
    detail_progress_message: String,
    drag_offset: Point,
    is_dragging: bool,
}

struct Bridge;
impl<W: Widget<AppState>> Controller<AppState, W> for Bridge {
    fn event(
            &mut self, 
            child: &mut W, 
            ctx: &mut EventCtx, 
            event: &Event, 
            data: &mut AppState, 
            env: &Env
        ) {
        match event {
            Event::WindowCloseRequested => {
                println!("Window are being closed");
                ctx.submit_command(QUIT_APP);
            }
            Event::WindowConnected => {
                bridge(ctx.get_external_handle());  
            }
            _ => {}
        }

        child.event(ctx, event, data, env);
    }
}

struct DragController;
impl<W: Widget<AppState>> Controller<AppState, W> for DragController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,  
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::MouseDown(mouse) => {
                data.is_dragging = true;
                data.drag_offset = mouse.pos;
                ctx.set_active(true);
            }
            Event::MouseMove(mouse) if data.is_dragging => {
                let delta = mouse.pos - data.drag_offset;
                let new_pos = ctx.window().get_position() + delta;
                ctx.window().set_position(new_pos);
            }
            Event::MouseUp(_) => {
                data.is_dragging = false;
                ctx.set_active(false);
            }
            _ => {}
        }

        child.event(ctx, event, data, env);
    }
}

struct ExitButton;
impl<W: Widget<AppState>> Controller<AppState, W> for ExitButton {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.submit_command(QUIT_APP);
            }
            _ => {}
        }
        child.event(ctx, event, data, env);
    }
}

const GRAY_COLOR: &str = "3C3C3C";
const BLACK_COLOR: &str = "282828";
const RED_COLOR: &str = "B03B3B";

use std::str::FromStr;
fn build_ui() -> impl Widget<AppState> {
    let panel = Flex::row()
        .with_child(Svg::new(SvgData::from_str(include_str!("../icon.svg")).unwrap())
                .fix_width(16.0).padding((3.0, 0.0, 2.0, 0.0)))
        .with_child(Label::new("ZONE UPDATER").with_text_size(10.0))
        .fix_width(390.0)
        .fix_height(20.0)
        .background(Color::from_hex_str(BLACK_COLOR).unwrap());

    let main_progress_message = Label::new(|data: &String, _env: &_| data.clone())
        .lens(AppState::progress_message).align_left()
        .padding((10.0, 10.0, 10.0, 5.0));

    let progress_painter = druid::widget::Painter::new(|ctx, data: &f64, _env| {
        let size = ctx.size();
        let background_rect = size.to_rect();

        ctx.fill(background_rect, &Color::from_hex_str("C6C6C6").unwrap());

        let progress_width = size.width * *data;
        let progress_rect = Rect::from_origin_size((0.0, 0.0), (progress_width, size.height));

        ctx.fill(progress_rect, &Color::from_hex_str(RED_COLOR).unwrap());
    });

    let button_with_detail = Flex::row()
        .with_child(
            SizedBox::new(
                Label::new(|data: &String, _: &_| data.clone())
                .lens(AppState::detail_progress_message)
            ).width(300.0)
        )
        .with_child(
            SizedBox::new(Align::centered(Label::new("Отменить").with_text_size(12.0)))
                .width(70.0)
                .height(30.0)
                .background(Color::from_hex_str(BLACK_COLOR).unwrap())
                .controller(ExitButton)
        )
        .fix_width(390.0)
        .fix_height(20.0)
        .padding((10.0, 5.0, 0.0, 0.0));

    let layout = Flex::column()
        .with_child(panel.controller(DragController))
        .with_child(main_progress_message)
        .with_child(
            SizedBox::new(progress_painter)
                .fix_width(350.0)
                .fix_height(25.0)
                .lens(AppState::progress)
        )
        .with_child(button_with_detail)
        .background(druid::Color::from_hex_str(GRAY_COLOR).unwrap())
        .controller(Bridge);

    layout
}

#[tokio::main]
async fn main() {
	let monitors = Screen::get_display_rect();
	let window_size = Size::new(390.0, 120.0);

    let window = WindowDesc::new(build_ui())
        .title("main-thread")
        .window_size(window_size)
        .resizable(false)
        .show_titlebar(false)
        .set_position(Point::new((monitors.x1 - window_size.width) / 2.0, (monitors.y1 - window_size.height) / 2.0));

    let initial_state = AppState {
        progress: 0.0,
        progress_message: "Сверяем метаданные".to_string(), 
        detail_progress_message: "".to_string(), 
        drag_offset: Point::ZERO,
        is_dragging: false,
    };

    AppLauncher::with_window(window)
        .launch(initial_state)
        .expect("Failed to launch application");
}