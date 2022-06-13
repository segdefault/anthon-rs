use std::rc::Rc;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use slint::{Model, SharedString, VecModel, Weak};
use strum::{IntoEnumIterator, VariantNames};

use slint_generatedMainWindow::{
    Command as SlintCommand, Edge as SlintEdge, Node as SlintNode, Sign as SlintSign,
};

use crate::common::state::{StateEvent, StateType};
use crate::common::{
    Axis, Command as CoreCommand, CommandDiscriminants, Edge as CoreEdge, Feature, KeyEvent,
    MouseButton, Node as CoreNode, ScrollCommand, Sign as CoreSign,
};
use crate::config::INITIAL_STATE_INDEX;
use crate::{ConditionalEdge, ConditionalGraph, Config, State, StateIndex};

slint::include_modules!();

pub struct WindowModel {
    nodes: Rc<VecModel<SlintNode>>,
    edges: Rc<VecModel<SlintEdge>>,
    signs: Rc<VecModel<SlintSign>>,
}

impl WindowModel {
    pub fn nodes(&self) -> Rc<VecModel<SlintNode>> {
        self.nodes.clone()
    }

    pub fn edges(&self) -> Rc<VecModel<SlintEdge>> {
        self.edges.clone()
    }

    pub fn signs(&self) -> Rc<VecModel<SlintSign>> {
        self.signs.clone()
    }
}

impl Default for WindowModel {
    fn default() -> Self {
        WindowModel {
            nodes: Rc::new(VecModel::from(vec![])),
            edges: Rc::new(VecModel::from(vec![])),
            signs: Rc::new(VecModel::from(vec![])),
        }
    }
}

impl MainWindow {
    pub fn attach_config_callbacks(
        &self,
        config: Arc<Mutex<Config>>,
        window_model: Rc<WindowModel>,
    ) {
        self.on_add_sign({
            let window = self.as_weak();
            let config = Arc::clone(&config);
            let window_model = window_model.clone();

            move || {
                window
                    .unwrap()
                    .add_sign(config.clone(), window_model.signs.clone())
            }
        });

        self.on_delete_sign({
            let window = self.as_weak();
            let config = Arc::clone(&config);
            let window_model = window_model.clone();

            move |sign_name| {
                window.unwrap().delete_sign(
                    sign_name.to_string(),
                    config.clone(),
                    window_model.edges.clone(),
                    window_model.signs.clone(),
                )
            }
        });

        self.on_set_feature({
            let window = self.as_weak();
            let config = Arc::clone(&config);

            move |sign_name, feature_index, irrelevant, required| {
                window.unwrap().set_feature(
                    sign_name.to_string(),
                    feature_index as usize,
                    irrelevant,
                    required,
                    config.clone(),
                )
            }
        });

        self.on_set_sign_name({
            let window = self.as_weak();
            let config = Arc::clone(&config);
            let window_model = window_model.clone();

            move |old_name, new_name| {
                window.unwrap().set_sign_name(
                    old_name.to_string(),
                    new_name.to_string(),
                    config.clone(),
                    window_model.edges.clone(),
                );
            }
        });

        self.on_add_node({
            let config = config.clone();
            let window_model = window_model.clone();
            let window = self.as_weak();

            move |x, y| {
                window
                    .unwrap()
                    .add_node(x, y, config.clone(), window_model.nodes.clone());
            }
        });

        self.on_delete_node({
            let window_weak = self.as_weak();
            let config = Arc::clone(&config);
            let window_model = window_model.clone();

            move |node| {
                window_weak
                    .unwrap()
                    .delete_node(node, config.clone(), window_model.clone());
            }
        });

        self.on_rename_node({
            let window = self.as_weak();
            let config = Arc::clone(&config);
            let window_model = window_model.clone();

            move |node| {
                window
                    .unwrap()
                    .rename_node(node, config.clone(), window_model.nodes.clone())
            }
        });

        self.on_node_moved({
            let window = self.as_weak();
            let config = Arc::clone(&config);
            let window_model = window_model.clone();

            move |node| {
                window
                    .unwrap()
                    .node_moved(node, config.clone(), window_model.edges.clone());
            }
        });

        let window_weak = self.as_weak();
        let config_clone = Arc::clone(&config);
        self.on_node_command_updated(move |node, command| {
            window_weak
                .unwrap()
                .update_node_command(node, command, config_clone.clone());
        });

        self.on_add_edge({
            let window = self.as_weak();
            let config = Arc::clone(&config);
            let window_model = window_model.clone();

            move |from_node, to_node| {
                window.unwrap().add_edge(
                    from_node,
                    to_node,
                    config.clone(),
                    window_model.edges.clone(),
                );
            }
        });

        self.on_delete_edge({
            let window = self.as_weak();
            let config = Arc::clone(&config);
            let window_model = window_model.clone();

            move |edge| {
                window
                    .unwrap()
                    .delete_edge(edge, config.clone(), window_model.edges.clone())
            }
        });

        self.on_set_edge_trigger({
            let window = self.as_weak();
            let config = Arc::clone(&config);
            let window_model = window_model;

            move |edge, trigger| {
                window.unwrap().set_edge_trigger(
                    edge,
                    trigger.to_string(),
                    config.clone(),
                    window_model.edges.clone(),
                )
            }
        });
    }

    fn add_sign(&self, config: Arc<Mutex<Config>>, signs: Rc<VecModel<SlintSign>>) {
        let config_clone = config.clone();
        let mut config = config.lock().unwrap();
        let sign_name = config.sign_dictionary_mut().next_valid_name();
        let sign = CoreSign::default();

        signs.push(SlintSign {
            name: sign_name.clone().into(),
            required_flags: Rc::new(VecModel::from(Vec::<bool>::from(
                sign.required_attributes(),
            )))
            .into(),
            irrelevant_flags: Rc::new(VecModel::from(Vec::<bool>::from(
                sign.irrelevant_attributes(),
            )))
            .into(),
        });

        config
            .sign_dictionary_mut()
            .signs_mut()
            .insert(sign_name, sign);
        std::mem::drop(config);

        self.refresh_triggers(config_clone);
    }

    fn delete_sign(
        &self,
        name: String,
        config: Arc<Mutex<Config>>,
        edges: Rc<VecModel<SlintEdge>>,
        signs: Rc<VecModel<SlintSign>>,
    ) {
        config
            .lock()
            .unwrap()
            .sign_dictionary_mut()
            .signs_mut()
            .remove(&name);

        for (i, sign) in signs.iter().enumerate() {
            if sign.name == name {
                signs.remove(i);
                break;
            }
        }

        self.broadcast_trigger_update(&mut config.lock().unwrap(), &name, None, edges);
        self.refresh_triggers(config);
    }

    fn set_feature(
        &self,
        name: String,
        feature_index: usize,
        irrelevant: bool,
        required: bool,
        config: Arc<Mutex<Config>>,
    ) {
        config
            .lock()
            .unwrap()
            .sign_dictionary_mut()
            .signs_mut()
            .get_mut(&name)
            .expect("BUG: Unknown sign.")
            .set_feature(feature_index, irrelevant, required);
    }

    fn set_sign_name(
        &self,
        old_name: String,
        new_name: String,
        config: Arc<Mutex<Config>>,
        edges: Rc<VecModel<SlintEdge>>,
    ) {
        let config_clone = config.clone();
        let mut config = config.lock().unwrap();
        let signs = config.sign_dictionary_mut().signs_mut();

        let sign = signs.remove(&old_name).expect("BUG: Unknown sign.");
        signs.insert(new_name.clone(), sign);

        self.broadcast_trigger_update(&mut config, &old_name, Some(&new_name), edges);

        std::mem::drop(config);
        self.refresh_triggers(config_clone);
    }

    fn broadcast_trigger_update(
        &self,
        config: &mut Config,
        old_trigger: &String,
        new_trigger: Option<&String>,
        edges: Rc<VecModel<SlintEdge>>,
    ) {
        for mut edge in edges.iter() {
            if edge.title == old_trigger {
                edge.title = new_trigger.unwrap_or(&String::default()).into();
            }
        }

        for mut edge in config
            .state_graph_mut()
            .edge_iter_mut()
            .flat_map(|(_, e)| e.values_mut())
        {
            if let Some(ref mut edge_trigger) = edge.trigger {
                if edge_trigger == old_trigger {
                    edge.trigger = new_trigger.cloned();
                }
            }
        }
    }

    pub fn update_features(&self) {
        let features: Vec<SharedString> = Feature::iter().map(|f| f.to_string().into()).collect();
        self.set_sign_flag_names(Rc::new(VecModel::from(features)).into());
    }

    pub fn update_signs(&self, config: Arc<Mutex<Config>>, signs: Rc<VecModel<SlintSign>>) {
        signs.set_vec(
            config
                .lock()
                .unwrap()
                .sign_dictionary()
                .signs()
                .iter()
                .map(|(name, sign)| SlintSign {
                    name: name.into(),
                    required_flags: Rc::new(VecModel::from(Vec::<bool>::from(
                        sign.required_attributes(),
                    )))
                    .into(),
                    irrelevant_flags: Rc::new(VecModel::from(Vec::<bool>::from(
                        sign.irrelevant_attributes(),
                    )))
                    .into(),
                })
                .collect::<Vec<SlintSign>>(),
        );

        self.refresh_triggers(config);
    }

    pub fn refresh_triggers(&self, config: Arc<Mutex<Config>>) {
        let config = config.lock().unwrap();
        let signs = config.sign_dictionary().signs();

        let triggers: Vec<SharedString> = signs.keys().map(SharedString::from).collect();
        self.set_triggers(Rc::new(VecModel::from(triggers)).into());
    }

    pub fn update_state_graph(&self, config: Arc<Mutex<Config>>, window_model: Rc<WindowModel>) {
        let config = config.lock().unwrap();
        let graph = config.state_graph();

        let node_types: Vec<SharedString> =
            StateType::iter().map(|t| t.to_string().into()).collect();
        let command_types: Vec<SharedString> = CoreCommand::VARIANTS
            .iter()
            .map(|s| SharedString::from(*s))
            .collect();
        let mouse_buttons: Vec<SharedString> = MouseButton::VARIANTS
            .iter()
            .map(|s| SharedString::from(*s))
            .collect();
        let key_events: Vec<SharedString> = KeyEvent::VARIANTS
            .iter()
            .map(|s| SharedString::from(*s))
            .collect();
        let axes: Vec<SharedString> = Axis::VARIANTS
            .iter()
            .map(|s| SharedString::from(*s))
            .collect();

        window_model.nodes.set_vec(
            graph
                .nodes()
                .values()
                .into_iter()
                .map(|node| node.into())
                .collect::<Vec<SlintNode>>(),
        );

        window_model.edges.set_vec(
            graph
                .edges()
                .iter()
                .flat_map(|core_node_edge_map| {
                    let from = Node::from(
                        graph
                            .nodes()
                            .get(core_node_edge_map.0)
                            .expect("ERROR: Invalid node from_id"),
                    );

                    core_node_edge_map.1.values().map(move |core_edge| {
                        let from = from.clone();
                        let to = Node::from(
                            graph
                                .nodes()
                                .get(&core_edge.next())
                                .expect("ERROR: Invalid node to_id"),
                        );
                        let title =
                            SharedString::from(core_edge.trigger().clone().unwrap_or_default());

                        Edge { from, to, title }
                    })
                })
                .collect::<Vec<SlintEdge>>(),
        );

        self.set_node_types(Rc::new(VecModel::from(node_types)).into());
        self.set_command_types(Rc::new(VecModel::from(command_types)).into());
        self.set_mouse_buttons(Rc::new(VecModel::from(mouse_buttons)).into());
        self.set_key_events(Rc::new(VecModel::from(key_events)).into());
        self.set_axes(Rc::new(VecModel::from(axes)).into());
        self.set_nodes(window_model.nodes.clone().into());
        self.set_edges(window_model.edges.clone().into());
    }

    fn add_node(&self, x: f32, y: f32, config: Arc<Mutex<Config>>, nodes: Rc<VecModel<SlintNode>>) {
        let mut config = config.lock().unwrap();
        let mut state = config.new_state();
        println!("ADDING NODE WITH ID: {}", state.id());

        state.x = x;
        state.y = y;

        nodes.push(Node::from(state as &State<StateIndex>));
    }

    fn delete_node(
        &self,
        node: SlintNode,
        config: Arc<Mutex<Config>>,
        window_model: Rc<WindowModel>,
    ) {
        if node.id == INITIAL_STATE_INDEX {
            // TODO: Show an error message
            return;
        }

        let nodes = window_model.nodes();
        let edges = window_model.edges();
        let mut config = config.lock().unwrap();
        println!("DELETING NODE WITH ID: {}", node.id);

        config.state_graph_mut().delete_node(&node.id);

        for (i, other) in nodes.iter().enumerate() {
            if other.id == node.id {
                nodes.remove(i);
                break;
            }
        }

        edges.set_vec(
            edges
                .iter()
                .filter(|e| e.from.id != node.id && e.to.id != node.id)
                .collect::<Vec<SlintEdge>>(),
        );
    }

    fn rename_node(
        &self,
        node: SlintNode,
        config: Arc<Mutex<Config>>,
        nodes: Rc<VecModel<SlintNode>>,
    ) {
        let mut config = config.lock().unwrap();
        let core_node = config
            .state_graph_mut()
            .get_node_mut(&node.id)
            .expect("ERROR: Invalid node id.");
        core_node.name = String::from(node.title.clone());

        for mut n in nodes.iter() {
            if n.id == node.id {
                n.title = node.title.clone();
            }
        }
    }

    fn node_moved(
        &self,
        node: SlintNode,
        config: Arc<Mutex<Config>>,
        edges: Rc<VecModel<SlintEdge>>,
    ) {
        let mut config = config.lock().unwrap();
        let state_graph = config.state_graph_mut();
        let core_node = state_graph
            .get_node_mut(&node.id)
            .expect("ERROR: Invalid Node ID.");

        core_node.x = node.x;
        core_node.y = node.y;

        self.move_edges_of(&node, edges.clone());
        self.update_edges(state_graph, edges);
    }

    fn update_node_command(
        &self,
        node: SlintNode,
        command: SlintCommand,
        config: Arc<Mutex<Config>>,
    ) {
        let mut config = config.lock().unwrap();
        let core_node = config
            .state_graph_mut()
            .get_node_mut(&node.id)
            .expect("ERROR: Invalid Node ID.");

        core_node.set_command(
            StateEvent::from_str(command.title.as_str()).unwrap(),
            (&command).into(),
        );
    }

    fn move_edges_of(&self, node: &SlintNode, edges: Rc<VecModel<SlintEdge>>) {
        println!("{:?}", node);

        for mut e in edges.iter() {
            if e.from.id == node.id {
                println!("hit");
                e.from.x = node.x;
                e.from.y = node.y;
            } else if e.to.id == node.id {
                println!("hit");
                e.to.x = node.x;
                e.to.y = node.y;
            }
        }
    }

    pub fn update_active_node_id(window: Weak<MainWindow>, node_id: StateIndex) {
        slint::invoke_from_event_loop(move || {
            window.unwrap().set_active_node_id(node_id);
        });
    }

    pub fn add_edge(
        &self,
        from_node: SlintNode,
        to_node: SlintNode,
        config: Arc<Mutex<Config>>,
        edges: Rc<VecModel<SlintEdge>>,
    ) {
        let mut config = config.lock().unwrap();
        let graph = config.state_graph_mut();

        if graph.add_edge(&from_node.id, ConditionalEdge::new(to_node.id, None)) {
            edges.push(Edge {
                title: SharedString::default(),
                from: from_node,
                to: to_node,
            });
        }
    }

    pub fn delete_edge(
        &self,
        edge: SlintEdge,
        config: Arc<Mutex<Config>>,
        edges: Rc<VecModel<SlintEdge>>,
    ) {
        let mut config = config.lock().unwrap();
        let graph = config.state_graph_mut();

        graph.delete_edge(&edge.from.id, &edge.to.id);
        for (i, other) in edges.iter().enumerate() {
            if edge == other {
                edges.remove(i);

                break;
            }
        }
    }

    pub fn set_edge_trigger(
        &self,
        edge: SlintEdge,
        trigger: String,
        config: Arc<Mutex<Config>>,
        edges: Rc<VecModel<SlintEdge>>,
    ) {
        let mut config = config.lock().unwrap();
        let graph = config.state_graph_mut();

        graph
            .get_edge_mut(&edge.from.id, &edge.to.id)
            .expect("Consistency Error: Invalid edge")
            .trigger = Some(trigger.clone());

        for mut other in edges.iter() {
            if other == edge {
                other.title = trigger.clone().into();
            }
        }
    }

    pub fn update_edges(&self, graph: &mut ConditionalGraph, edges: Rc<VecModel<SlintEdge>>) {
        let core_edge_map = graph.edges();
        edges.set_vec(
            core_edge_map
                .iter()
                .flat_map(|core_node_edge_map| {
                    core_node_edge_map.1.values().map({
                        |core_edge| {
                            let from = Node::from(
                                graph
                                    .nodes()
                                    .get(core_node_edge_map.0)
                                    .expect("ERROR: Invalid node from_id"),
                            );

                            let to = Node::from(
                                graph
                                    .nodes()
                                    .get(&core_edge.next())
                                    .expect("ERROR: Invalid node to_id"),
                            );
                            let title =
                                SharedString::from(core_edge.trigger().clone().unwrap_or_default());

                            SlintEdge { from, to, title }
                        }
                    })
                })
                .collect::<Vec<SlintEdge>>(),
        );
    }
}

impl From<&State<StateIndex>> for SlintNode {
    fn from(state: &State<StateIndex>) -> Self {
        let commands: Vec<SlintCommand> = state
            .events()
            .iter()
            .map(|(e, c)| SlintCommand::from((e, c)))
            .collect();

        Self {
            id: state.id(),
            title: state.name.clone().into(),
            x: state.x,
            y: state.y,

            r#type: state.r#type.to_string().into(),
            commands: Rc::new(VecModel::from(commands)).into(),
        }
    }
}

// impl SlintCommand {
//     fn default() -> Self {
//         Self {
//             title: "PLACEHOLDER".into(),
//             r#type: CoreCommand::Disabled.to_string().into(),
//             exec_command: "".into(),
//             mouse_button: MouseButton::VARIANTS[0].into(),
//             key_event: KeyEvent::VARIANTS[0].into(),
//             scroll_axis: "X".into(),
//             scroll_custom_command: "".into(),
//             scroll_custom_command_enabled: false,
//         }
//     }
// }

impl From<(&StateEvent, &Option<CoreCommand>)> for SlintCommand {
    fn from(event: (&StateEvent, &Option<CoreCommand>)) -> Self {
        if let Some(command) = event.1 {
            (event.0, command).into()
        } else {
            Self {
                title: event.0.to_string().into(),
                ..SlintCommand::default()
            }
        }
    }
}

impl From<(&StateEvent, &CoreCommand)> for SlintCommand {
    fn from(event: (&StateEvent, &CoreCommand)) -> Self {
        let command = event.1;
        let (mouse_button, key_event) = if let CoreCommand::Mouse(mouse_button, key_event) = command
        {
            (
                mouse_button.to_string().into(),
                key_event.to_string().into(),
            )
        } else {
            (
                MouseButton::Left.to_string().into(),
                KeyEvent::Click.to_string().into(),
            )
        };

        let (scroll_custom_command_enabled, scroll_custom_command, scroll_axis, scroll_factor) =
            if let CoreCommand::Scroll(cmd) = command {
                (
                    cmd.custom_command.is_some(),
                    if let Some(ref cmd) = cmd.custom_command {
                        cmd.into()
                    } else {
                        "".into()
                    },
                    cmd.axis.to_string().into(),
                    cmd.factor,
                )
            } else {
                (false, "".into(), Axis::VARIANTS[0].into(), 100f32)
            };

        Self {
            title: event.0.to_string().into(),
            r#type: command.to_string().into(),
            exec_command: if let CoreCommand::Execute(exec_command) = command {
                exec_command.into()
            } else {
                "".into()
            },
            mouse_button,
            key_event,
            scroll_custom_command_enabled,
            scroll_custom_command,
            scroll_axis,
            scroll_factor,
        }
    }
}

impl From<&SlintCommand> for CoreCommand {
    fn from(command: &SlintCommand) -> Self {
        let discriminant = CommandDiscriminants::from_str(command.r#type.as_str())
            .expect("ERROR: Invalid command type.");

        match discriminant {
            CommandDiscriminants::Disabled => Self::Disabled,
            CommandDiscriminants::Execute => Self::Execute(command.exec_command.to_string()),
            CommandDiscriminants::Mouse => Self::Mouse(
                MouseButton::from_str(command.mouse_button.as_str()).unwrap(),
                KeyEvent::from_str(command.key_event.as_str()).unwrap(),
            ),
            CommandDiscriminants::Scroll => Self::Scroll(ScrollCommand {
                custom_command: if command.scroll_custom_command_enabled {
                    Some(command.scroll_custom_command.clone().into())
                } else {
                    None
                },
                factor: command.scroll_factor,
                axis: Axis::from_str(command.scroll_axis.as_str()).unwrap(),
            }),
        }
    }
}
