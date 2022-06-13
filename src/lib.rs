use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use nokhwa::{CameraFormat, FrameFormat, ThreadedCamera};
use slint::ComponentHandle;

use config::Config;
use ui::MainWindow;

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

pub const FPS: u64 = 30;
pub const MPF: u64 = ((1f32 / FPS as f32) * 1000f32) as u64;
const CONFIG_PATH: &str = "config.yaml";

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel();
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
            crate::FPS as u32,
        )),
    )
    .expect("Capturing device initialization failed.");

    camera
        .open_stream(|_| ())
        .expect("Opening capture stream failed.");

    let window_weak = window.as_weak();
    let config_clone = Arc::clone(&config);
    let processing_thread = thread::spawn(move || {
        let mut core = Core::new(window_weak, camera, config_clone);

        while rx.try_recv().is_err() {
            core.tick();
            thread::sleep(Duration::from_millis(MPF));
        }
    });

    window.run();
    tx.send(0)?;
    config.lock().unwrap().save(CONFIG_PATH)?;
    processing_thread
        .join()
        .expect("Failed to shutdown the processing thread");

    Ok(())
}
