use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

use nokhwa::{CameraFormat, FrameFormat, ThreadedCamera};
use slint::ComponentHandle;

use config::Config;
use ui::MainWindow;

use crate::common::filter::Wmaf32;
use crate::common::{state::ConditionalEdge, Graph, State};
use crate::core::Core;

pub mod common;
mod config;
mod core;
pub mod mediapipe;
mod ui;

type StateIndex = i32;
type ConditionalGraph =
    Graph<StateIndex, State<StateIndex>, ConditionalEdge<StateIndex, Option<String>>>;

pub const TARGET_FPS: u64 = 30;
pub const TARGET_MPF: u64 = ((1f32 / TARGET_FPS as f32) * 1000f32) as u64;
const CONFIG_PATH: &str = "config.yaml";

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let interrupted = Arc::new(Mutex::new(false));
    let config = Arc::new(Mutex::new(
        Config::from_file(CONFIG_PATH).unwrap_or_default(),
    ));

    let window = MainWindow::new();
    let mut camera = ThreadedCamera::new(
        0,
        Some(CameraFormat::new_from(
            640,
            480,
            FrameFormat::MJPEG,
            crate::TARGET_FPS as u32,
        )),
    )
    .expect("Capturing device initialization failed.");

    camera
        .open_stream(|_| ())
        .expect("Opening capture stream failed.");

    let window_weak = window.as_weak();
    let config_clone = Arc::clone(&config);
    let processing_thread = thread::spawn({
        let interrupted = interrupted.clone();

        move || {
            let mut core = Core::new(window_weak, camera, config_clone);
            let mut mpf = Wmaf32::new(5);
            let mut last_fps;

            while !*interrupted.lock().unwrap() {
                let last_time = SystemTime::now();

                core.tick();

                mpf.set_value(
                    SystemTime::now()
                        .duration_since(last_time)
                        .unwrap()
                        .as_millis() as f32,
                );

                let sleep_time_ms = (TARGET_MPF - (*mpf as u64).min(TARGET_MPF)).max(1);
                thread::sleep(Duration::from_millis(sleep_time_ms));

                last_fps = 1.0 / SystemTime::now().duration_since(last_time).unwrap().as_secs_f32();
                println!("{}", last_fps);
            }
        }
    });

    window.run();
    *interrupted.lock().unwrap() = true;
    config.lock().unwrap().save(CONFIG_PATH)?;
    processing_thread
        .join()
        .expect("Failed to shutdown the processing thread");

    Ok(())
}
