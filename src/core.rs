use std::rc::Rc;
use std::sync::{Arc, Mutex};

use image::imageops;
use nokhwa::ThreadedCamera;
#[allow(unused_imports)]
use slint::ComponentHandle;
use slint::Weak;
use slint::{Image, SharedPixelBuffer};

use crate::common::state::{StateMachine, StateType};
use crate::common::{pointer, PointerTracker, ProbabilityVector, Sign};
use crate::config::INITIAL_STATE_INDEX;
use crate::mediapipe::Mediapipe;
use crate::ui::{MainWindow, WindowModel};
use crate::Config;

use super::StateIndex;

pub struct Core {
    camera: ThreadedCamera,
    mediapipe: Mediapipe,
    window: Weak<MainWindow>,
    config: Arc<Mutex<Config>>,

    state_machine: StateMachine<StateIndex>,
    pointer_tracker: PointerTracker,
    probability_vector: ProbabilityVector,
}

impl Core {
    pub fn new(
        window: Weak<MainWindow>,
        camera: ThreadedCamera,
        config: Arc<Mutex<Config>>,
    ) -> Self {
        let (num_signs, probability_vector_sensitivity) = {
            let config = Arc::clone(&config);
            let config = config.lock().unwrap();

            (
                config.sign_dictionary().signs().len(),
                config.sign_switching_smoothness,
            )
        };

        let pointer_tracker = PointerTracker::new(pointer::DEFAULT_WMA_ORDER)
            .expect("ERROR: Unable to initialize pointer tracker.");
        let probability_vector = ProbabilityVector::new(num_signs, probability_vector_sensitivity);
        let state_machine = StateMachine::new(INITIAL_STATE_INDEX);

        let core = Core {
            camera,
            mediapipe: Mediapipe::default(),
            window,
            config: Arc::clone(&config),
            state_machine,
            pointer_tracker,
            probability_vector,
        };

        core.init_window();

        core
    }

    fn init_window(&self) {
        MainWindow::update_active_node_id(self.window.clone(), INITIAL_STATE_INDEX);

        let window = self.window.clone();
        let config = Arc::clone(&self.config);
        slint::invoke_from_event_loop(move || {
            let window_model = Rc::new(WindowModel::default());
            let window = window.unwrap();

            window.update_features();
            window.update_signs(config.clone(), window_model.signs());
            window.update_state_graph(config.clone(), window_model.clone());
            window.attach_config_callbacks(config, window_model);
        });
    }

    fn update_pointer_freeze(&mut self) {
        let config = self.config.lock().unwrap();

        let current_state = config
            .state_graph()
            .get_node(self.state_machine.current_state())
            .unwrap();
        self.pointer_tracker.freeze = !current_state.r#type.eq(&StateType::Pointing);
    }

    fn check_sign_count_update(&mut self) {
        let (num_signs, probability_vector_sensitivity) = {
            let config = self.config.lock().unwrap();

            (
                config.sign_dictionary().signs().len(),
                config.sign_switching_smoothness,
            )
        };

        if num_signs != self.probability_vector.probabilities().len() {
            self.probability_vector =
                ProbabilityVector::new(num_signs, probability_vector_sensitivity);
        }
    }

    pub fn tick(&mut self) {
        self.update_pointer_freeze();
        self.check_sign_count_update();

        let config = self.config.lock().unwrap();
        let frame = imageops::flip_horizontal(&self.camera.last_frame());
        let packet = self.mediapipe.process(frame);

        self.pointer_tracker
            .track(&packet)
            .expect("ERROR: Tracking error.");

        if let Some(landmarks) = packet.landmarks {
            let sign: Sign = landmarks.as_slice().into();
            let similar = config.sign_dictionary().find_similar(&sign);

            if let Some(similar) = similar {
                self.probability_vector.adjust(similar.index);

                let (probable_sign_index, probability) = self
                    .probability_vector
                    .max()
                    .expect("BUG: Value can't be represented as f32.");
                let probable_sign = config
                    .sign_dictionary()
                    .get_by_index(probable_sign_index)
                    .expect("BUG: Sign index doesn't exist.");

                if probability > config.sign_probability_threshold
                    && similar.sign == probable_sign.1
                {
                    let state_updated = self.state_machine.process(
                        config.state_graph(),
                        probable_sign.0,
                        &mut self.pointer_tracker,
                    );
                    if state_updated {
                        MainWindow::update_active_node_id(
                            self.window.clone(),
                            *self.state_machine.current_state(),
                        );
                    }
                }
            }
        } else {
            self.probability_vector.rebalance();
        }

        slint::invoke_from_event_loop({
            let window_clone = self.window.clone();
            let frame = self.camera.last_frame();

            move || {
                let buffer = SharedPixelBuffer::clone_from_slice(
                    frame.as_raw(),
                    frame.width(),
                    frame.height(),
                );
                let image = Image::from_rgb8(buffer);
                window_clone.unwrap().set_webcam_image(image);
            }
        });
    }
}
