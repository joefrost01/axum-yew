use crate::models::dag::{DAGGraph as DAGGraphModel, TaskStatus};
use crate::utils::api;
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlElement, MouseEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DAGGraphProps {
    pub dag_id: String,
}

pub enum Msg {
    LoadGraph,
    GraphLoaded(Result<DAGGraphModel, String>),
    ShowContextMenu(MouseEvent, String),  // event, task_id
    HideContextMenu,
    TaskAction(String, String),  // task_id, action
}

pub struct DAGGraph {
    graph: Option<DAGGraphModel>,
    loading: bool,
    error: Option<String>,
    cytoscape_initialized: bool,
    context_menu_visible: bool,
    context_menu_position: (i32, i32),
    selected_task: Option<String>,
}

impl Component for DAGGraph {
    type Message = Msg;
    type Properties = DAGGraphProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::LoadGraph);
        
        Self {
            graph: None,
            loading: true,
            error: None,
            cytoscape_initialized: false,
            context_menu_visible: false,
            context_menu_position: (0, 0),
            selected_task: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadGraph => {
                self.loading = true;
                
                let dag_id = ctx.props().dag_id.clone();
                let link = ctx.link().clone();
                spawn_local(async move {
                    let result = api::fetch_dag_graph(&dag_id).await;
                    link.send_message(Msg::GraphLoaded(result));
                });
                
                false
            }
            Msg::GraphLoaded(result) => {
                self.loading = false;
                
                match result {
                    Ok(graph) => {
                        self.graph = Some(graph);
                        self.cytoscape_initialized = false;  // Trigger re-initialization
                    }
                    Err(err) => {
                        self.error = Some(err);
                    }
                }
                
                true
            }
            Msg::ShowContextMenu(event, task_id) => {
                event.prevent_default();  // Prevent default context menu
                
                // Get position relative to viewport
                let x = event.client_x();
                let y = event.client_y();
                
                self.context_menu_visible = true;
                self.context_menu_position = (x, y);
                self.selected_task = Some(task_id);
                
                true
            }
            Msg::HideContextMenu => {
                self.context_menu_visible = false;
                self.selected_task = None;
                
                true
            }
            Msg::TaskAction(task_id, action) => {
                self.context_menu_visible = false;
                
                let dag_id = ctx.props().dag_id.clone();
                let task_id_clone = task_id.clone();
                
                // Handle task actions
                match action.as_str() {
                    "view" => {
                        log::info!("View task details: {}", task_id);
                        // In a real app, you'd navigate to the task details page
                    }
                    "force-success" => {
                        // Update task status
                        let link = ctx.link().clone();
                        spawn_local(async move {
                            let _ = api::update_task_status(&dag_id, &task_id_clone, "SUCCEEDED").await;
                            link.send_message(Msg::LoadGraph);
                        });
                    }
                    "retry" => {
                        let link = ctx.link().clone();
                        spawn_local(async move {
                            let _ = api::update_task_status(&dag_id, &task_id_clone, "RUNNING").await;
                            link.send_message(Msg::LoadGraph);
                        });
                    }
                    "skip" => {
                        let link = ctx.link().clone();
                        spawn_local(async move {
                            let _ = api::update_task_status(&dag_id, &task_id_clone, "SKIPPED").await;
                            link.send_message(Msg::LoadGraph);
                        });
                    }
                    "pause" => {
                        let link = ctx.link().clone();
                        spawn_local(async move {
                            let _ = api::update_task_status(&dag_id, &task_id_clone, "PAUSED").await;
                            link.send_message(Msg::LoadGraph);
                        });
                    }
                    "play" => {
                        let link = ctx.link().clone();
                        spawn_local(async move {
                            let _ = api::update_task_status(&dag_id, &task_id_clone, "RUNNING").await;
                            link.send_message(Msg::LoadGraph);
                        });
                    }
                    _ => {}
                }
                
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let hide_menu = ctx.link().callback(|_| Msg::HideContextMenu);
        
        html! {
            <div class="dag-graph-container fluid-container">
                {
                    if self.loading {
                        html! { <div class="loading">{ "Loading DAG graph..." }</div> }
                    } else if let Some(error) = &self.error {
                        html! { <div class="error">{ format!("Error: {}", error) }</div> }
                    } else {
                        html! {
                            <>
                                <div class="header-row">
                                    <div class="dag-title-panel">
                                            { format!("DAG: {}", &ctx.props().dag_id) }
                                            {
                                                if let Some(graph) = &self.graph {
                                                    let node_count = graph.tasks.len();
                                                    if node_count > 0 {
                                                        html! { <span class="node-count">{ format!(" ({} nodes)", node_count) }</span> }
                                                    } else {
                                                        html! {}
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                        </div>
                                        <div class="legend-panel">
                                            { self.render_legend() }
                                        </div>
                                </div>
                                <div id="cytoscape-container" class="cytoscape-container"></div>
                                { self.render_context_menu(ctx) }
                                
                                // Invisible overlay to catch clicks outside the context menu
                                {
                                    if self.context_menu_visible {
                                        html! {
                                            <div 
                                                class="context-menu-overlay"
                                                onclick={hide_menu}
                                            ></div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                            </>
                        }
                    }
                }
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        // Initialize Cytoscape.js when the component is first rendered
        // or when the graph data changes
        if !self.cytoscape_initialized && self.graph.is_some() {
            self.initialize_cytoscape(ctx);
            self.cytoscape_initialized = true;
        }
    }
}

impl DAGGraph {
    // Helper function for handling mouse events
    fn create_mouse_hover_callback(bg_color: &'static str) -> Callback<MouseEvent> {
        Callback::from(move |e: MouseEvent| {
            if let Some(target) = e.target() {
                if let Ok(el) = target.dyn_into::<HtmlElement>() {
                    let style = el.style();
                    let _ = style.set_property("background-color", bg_color);
                }
            }
        })
    }
    
    fn initialize_cytoscape(&self, ctx: &Context<Self>) {
        if let Some(graph) = &self.graph {
            // We'll use JavaScript interop here to initialize Cytoscape.js
            // In a real app, you'd use a more robust solution like a proper Rust wrapper
            // for Cytoscape.js, but for this demo we'll keep it simple
            
            let window = web_sys::window().expect("no global window exists");
            let document = window.document().expect("no document on window");
            
            // Check if Cytoscape.js is loaded
            if !Reflect::has(&window, &JsValue::from_str("cytoscape"))
                .unwrap_or(false) 
            {
                log::error!("Cytoscape.js is not loaded. Make sure to include it in your HTML.");
                return;
            }
            
            // Create elements for Cytoscape
            let mut elements = Vec::new();
            
            // Add nodes
            for task in &graph.tasks {
                let node = Object::new();
                let data = Object::new();
                
                let task_id = task.id.clone();
                
                Reflect::set(&data, &JsValue::from_str("id"), &JsValue::from_str(&task.id)).unwrap();
                Reflect::set(&data, &JsValue::from_str("label"), &JsValue::from_str(&task.name)).unwrap();
                Reflect::set(&data, &JsValue::from_str("status"), &JsValue::from_str(&format!("{:?}", task.status))).unwrap();
                Reflect::set(&data, &JsValue::from_str("taskId"), &JsValue::from_str(&task_id)).unwrap();
                
                Reflect::set(&node, &JsValue::from_str("data"), &data).unwrap();
                Reflect::set(&node, &JsValue::from_str("group"), &JsValue::from_str("nodes")).unwrap();
                
                elements.push(node);
            }
            
            // Add edges
            for edge in &graph.edges {
                let edge_obj = Object::new();
                let data = Object::new();
                
                Reflect::set(&data, &JsValue::from_str("source"), &JsValue::from_str(&edge.source)).unwrap();
                Reflect::set(&data, &JsValue::from_str("target"), &JsValue::from_str(&edge.target)).unwrap();
                
                Reflect::set(&edge_obj, &JsValue::from_str("data"), &data).unwrap();
                Reflect::set(&edge_obj, &JsValue::from_str("group"), &JsValue::from_str("edges")).unwrap();
                
                elements.push(edge_obj);
            }
            
            // Create a Cytoscape instance
            let cytoscape = Reflect::get(&window, &JsValue::from_str("cytoscape"))
                .expect("cytoscape not found");
            
            let container = document.get_element_by_id("cytoscape-container")
                .expect("container not found");
            
            // Configure Cytoscape
            let config = Object::new();
            Reflect::set(&config, &JsValue::from_str("container"), &container).unwrap();
            
            let js_elements = Array::new();
            for elem in elements {
                js_elements.push(&elem);
            }
            
            Reflect::set(&config, &JsValue::from_str("elements"), &js_elements).unwrap();
            
            // Set up the layout - adjust parameters based on graph size
            let layout = Object::new();
            Reflect::set(&layout, &JsValue::from_str("name"), &JsValue::from_str("dagre")).unwrap();
            Reflect::set(&layout, &JsValue::from_str("rankDir"), &JsValue::from_str("LR")).unwrap(); // Left to right layout
            
            // Adjust spacing based on graph size
            let node_count = graph.tasks.len();
            log::info!("Initializing graph with {} nodes", node_count);
            
            if node_count > 500 {
                // Very large graphs: minimal spacing
                Reflect::set(&layout, &JsValue::from_str("nodeSep"), &JsValue::from_f64(20.0)).unwrap();
                Reflect::set(&layout, &JsValue::from_str("rankSep"), &JsValue::from_f64(40.0)).unwrap();
                Reflect::set(&layout, &JsValue::from_str("padding"), &JsValue::from_f64(5.0)).unwrap();
                Reflect::set(&layout, &JsValue::from_str("animate"), &JsValue::from_bool(false)).unwrap(); // Disable animation for performance
            } else if node_count > 100 {
                // Large graphs: reduced spacing
                Reflect::set(&layout, &JsValue::from_str("nodeSep"), &JsValue::from_f64(30.0)).unwrap();
                Reflect::set(&layout, &JsValue::from_str("rankSep"), &JsValue::from_f64(60.0)).unwrap();
                Reflect::set(&layout, &JsValue::from_str("padding"), &JsValue::from_f64(10.0)).unwrap();
                Reflect::set(&layout, &JsValue::from_str("animate"), &JsValue::from_bool(true)).unwrap();
                Reflect::set(&layout, &JsValue::from_str("animationDuration"), &JsValue::from_f64(300.0)).unwrap();
            } else {
                // Small/medium graphs: standard spacing
                Reflect::set(&layout, &JsValue::from_str("nodeSep"), &JsValue::from_f64(40.0)).unwrap();
                Reflect::set(&layout, &JsValue::from_str("rankSep"), &JsValue::from_f64(80.0)).unwrap();
                Reflect::set(&layout, &JsValue::from_str("padding"), &JsValue::from_f64(20.0)).unwrap();
                Reflect::set(&layout, &JsValue::from_str("animate"), &JsValue::from_bool(true)).unwrap();
                Reflect::set(&layout, &JsValue::from_str("animationDuration"), &JsValue::from_f64(500.0)).unwrap();
            }
            
            Reflect::set(&layout, &JsValue::from_str("fit"), &JsValue::from_bool(true)).unwrap();
            
            Reflect::set(&config, &JsValue::from_str("layout"), &layout).unwrap();
            
            // Set up the style
            let style = Array::new();
            
            // Node style
            let node_style = Object::new();
            Reflect::set(&node_style, &JsValue::from_str("selector"), &JsValue::from_str("node")).unwrap();
            
            let node_style_props = Object::new();
            Reflect::set(&node_style_props, &JsValue::from_str("content"), &JsValue::from_str("data(label)")).unwrap();
            Reflect::set(&node_style_props, &JsValue::from_str("text-valign"), &JsValue::from_str("center")).unwrap();
            Reflect::set(&node_style_props, &JsValue::from_str("text-halign"), &JsValue::from_str("center")).unwrap();
            // For large graphs, use fixed width nodes
            let node_count = graph.tasks.len();
            if node_count > 100 {
                let node_width = if node_count > 500 { 75.0 } else { 90.0 };
                let node_height = if node_count > 500 { 30.0 } else { 35.0 };
                
                Reflect::set(&node_style_props, &JsValue::from_str("width"), &JsValue::from_f64(node_width)).unwrap();
                Reflect::set(&node_style_props, &JsValue::from_str("height"), &JsValue::from_f64(node_height)).unwrap();
            } else {
                Reflect::set(&node_style_props, &JsValue::from_str("width"), &JsValue::from_str("label")).unwrap();
            }
            Reflect::set(&node_style_props, &JsValue::from_str("padding"), &JsValue::from_str("8px")).unwrap();
            Reflect::set(&node_style_props, &JsValue::from_str("shape"), &JsValue::from_str("roundrectangle")).unwrap();
            Reflect::set(&node_style_props, &JsValue::from_str("text-wrap"), &JsValue::from_str("wrap")).unwrap();
            Reflect::set(&node_style_props, &JsValue::from_str("text-max-width"), &JsValue::from_f64(100.0)).unwrap();
            Reflect::set(&node_style_props, &JsValue::from_str("font-size"), &JsValue::from_str("12px")).unwrap();
            
            Reflect::set(&node_style, &JsValue::from_str("style"), &node_style_props).unwrap();
            style.push(&node_style);
            
            // Edge style
            let edge_style = Object::new();
            Reflect::set(&edge_style, &JsValue::from_str("selector"), &JsValue::from_str("edge")).unwrap();
            
            let edge_style_props = Object::new();
            Reflect::set(&edge_style_props, &JsValue::from_str("width"), &JsValue::from_f64(2.0)).unwrap();
            Reflect::set(&edge_style_props, &JsValue::from_str("line-color"), &JsValue::from_str("#ccc")).unwrap();
            Reflect::set(&edge_style_props, &JsValue::from_str("target-arrow-color"), &JsValue::from_str("#ccc")).unwrap();
            Reflect::set(&edge_style_props, &JsValue::from_str("target-arrow-shape"), &JsValue::from_str("triangle")).unwrap();
            Reflect::set(&edge_style_props, &JsValue::from_str("curve-style"), &JsValue::from_str("bezier")).unwrap();
            
            Reflect::set(&edge_style, &JsValue::from_str("style"), &edge_style_props).unwrap();
            style.push(&edge_style);
            
            // Status-specific styles
            for status in [
                TaskStatus::SUCCEEDED, 
                TaskStatus::FAILED, 
                TaskStatus::RUNNING, 
                TaskStatus::PENDING,
                TaskStatus::QUEUED,
                TaskStatus::SKIPPED,
                TaskStatus::PAUSED
            ].iter() {
                let status_selector = format!("node[status = '{:?}']", status);
                let status_style = Object::new();
                Reflect::set(&status_style, &JsValue::from_str("selector"), &JsValue::from_str(&status_selector)).unwrap();
                
                let status_props = Object::new();
                Reflect::set(&status_props, &JsValue::from_str("background-color"), &JsValue::from_str(status.color())).unwrap();
                Reflect::set(&status_props, &JsValue::from_str("color"), &JsValue::from_str("white")).unwrap();
                
                Reflect::set(&status_style, &JsValue::from_str("style"), &status_props).unwrap();
                style.push(&status_style);
            }
            
            Reflect::set(&config, &JsValue::from_str("style"), &style).unwrap();
            
            // Create the Cytoscape instance
            let args = Array::of1(&config);
            let cytoscape_fn = js_sys::Function::from(cytoscape);
            let cy = Reflect::apply(&cytoscape_fn, &JsValue::NULL, &args)
                .expect("failed to create cytoscape instance");
            
            // Add right-click event listener for context menu
            let link = ctx.link().clone();
            
            // Use JS to add the right-click event listener directly to cy
            let js_code = r#"
                cy.on('cxttap', 'node', function(evt){
                    var node = evt.target;
                    var nodeId = node.data('taskId');
                    window.showContextMenu(evt.originalEvent, nodeId);
                });
            "#;
            
            // Add graph size info to the Cytoscape instance
            let graph_size_meta = Object::new();
            let node_count = graph.tasks.len();
            Reflect::set(&graph_size_meta, &JsValue::from_str("nodeCount"), &JsValue::from_f64(node_count as f64)).unwrap();
            Reflect::set(&cy, &JsValue::from_str("graphMeta"), &graph_size_meta).unwrap();
            
            log::info!("Graph initialized with {} nodes and {} edges", node_count, graph.edges.len());
            
            // Set up additional Cytoscape options for large graphs
            if node_count > 200 {
                // Add viewport manipulation controls for large graphs
                let pan_factor = if graph.tasks.len() > 500 { 3.0 } else { 2.0 };
                let js_setup_viewport = format!(r#"
                    cy.userPanningEnabled(true);
                    cy.userZoomingEnabled(true);
                    cy.minZoom(0.1);
                    cy.maxZoom(2.5);
                    cy.zoom(0.8);
                    cy.panningEnabled(true);
                    cy.boxSelectionEnabled(false);
                    cy.autoungrabify(false);
                    cy.autounselectify(false);
                    
                    // Set reasonable initial viewport
                    cy.zoom({{
                        level: 0.5,
                        position: {{x: cy.width() / 2, y: cy.height() / 2}}
                    }});
                    cy.pan({{x: cy.width() / {}, y: cy.height() / 2}});
                "#, pan_factor);
                
                let viewport_fn = js_sys::Function::new_with_args("cy", &js_setup_viewport);
                Reflect::apply(&viewport_fn, &JsValue::NULL, &Array::of1(&cy))
                    .expect("Failed to apply viewport settings");
            }
            
            // Create a global function for the event handler
            let show_context_menu_fn = Closure::wrap(Box::new(move |event: web_sys::MouseEvent, task_id: String| {
                link.send_message(Msg::ShowContextMenu(event, task_id));
            }) as Box<dyn FnMut(web_sys::MouseEvent, String)>);
            
            // Attach it to the window
            Reflect::set(
                &window,
                &JsValue::from_str("showContextMenu"),
                &show_context_menu_fn.as_ref().unchecked_ref(),
            ).expect("Failed to set showContextMenu function");
            
            // Execute the JS code that uses our global function
            let function = js_sys::Function::new_with_args("cy", js_code);
            Reflect::apply(&function, &JsValue::NULL, &Array::of1(&cy))
                .expect("Failed to apply context menu function");
            
            // Keep the handler alive
            show_context_menu_fn.forget();
        }
    }
    
    fn render_legend(&self) -> Html {
        html! {
            <ul class="legend-items-horizontal">
                <li>
                    <span class="legend-color" style="background-color: #4caf50;"></span>
                    <span class="legend-label">{ "Succeeded" }</span>
                </li>
                <li>
                    <span class="legend-color" style="background-color: #f44336;"></span>
                    <span class="legend-label">{ "Failed" }</span>
                </li>
                <li>
                    <span class="legend-color" style="background-color: #2196f3;"></span>
                    <span class="legend-label">{ "Running" }</span>
                </li>
                <li>
                    <span class="legend-color" style="background-color: #9e9e9e;"></span>
                    <span class="legend-label">{ "Pending" }</span>
                </li>
                <li>
                    <span class="legend-color" style="background-color: #ff9800;"></span>
                    <span class="legend-label">{ "Queued" }</span>
                </li>
                <li>
                    <span class="legend-color" style="background-color: #673ab7;"></span>
                    <span class="legend-label">{ "Skipped" }</span>
                </li>
                <li>
                    <span class="legend-color" style="background-color: #795548;"></span>
                    <span class="legend-label">{ "Paused" }</span>
                </li>
            </ul>
        }
    }
    
    fn render_context_menu(&self, ctx: &Context<Self>) -> Html {
        if !self.context_menu_visible || self.selected_task.is_none() {
            return html! {};
        }
        
        let task_id = self.selected_task.as_ref().unwrap();
        let (x, y) = self.context_menu_position;
        
        // Find the current task to determine available actions
        let task = if let Some(graph) = &self.graph {
            graph.tasks.iter().find(|t| t.id == *task_id)
        } else {
            None
        };
        
        let style = format!(
            "position: fixed; left: {}px; top: {}px; z-index: 1000; background: white; border: 1px solid #ccc; border-radius: 4px; box-shadow: 0 2px 10px rgba(0,0,0,0.2); padding: 8px 0;",
            x, y
        );
        
        let on_action = |action: &'static str| {
            let task_id = self.selected_task.clone();
            let callback = ctx.link().callback(move |_| {
                if let Some(id) = &task_id {
                    Msg::TaskAction(id.clone(), action.to_string())
                } else {
                    Msg::HideContextMenu
                }
            });
            callback
        };
        
        let menu_item_style = "padding: 8px 16px; cursor: pointer; display: block; text-decoration: none; color: #333; white-space: nowrap;";
        
        html! {
            <div class="context-menu" style={style}>
                <div 
                    class="context-menu-item"
                    style={menu_item_style}
                    onmouseover={Self::create_mouse_hover_callback("#f5f5f5")}
                    onmouseout={Self::create_mouse_hover_callback("")}
                    onclick={on_action("view")}
                >
                    { "View Task Details" }
                </div>
                
                {
                    if let Some(task) = task {
                        html! {
                            <>
                                <div class="context-menu-divider" style="height: 1px; background-color: #e0e0e0; margin: 4px 0;"></div>
                                
                                {
                                    if task.status == TaskStatus::FAILED {
                                        html! {
                                            <>
                                                <div 
                                                    class="context-menu-item"
                                                    style={menu_item_style}
                                                    onmouseover={Self::create_mouse_hover_callback("#f5f5f5")}
                                                    onmouseout={Self::create_mouse_hover_callback("")}
                                                    onclick={on_action("force-success")}
                                                >
                                                    { "Force Success" }
                                                </div>
                                                <div 
                                                    class="context-menu-item"
                                                    style={menu_item_style}
                                                    onmouseover={Self::create_mouse_hover_callback("#f5f5f5")}
                                                    onmouseout={Self::create_mouse_hover_callback("")}
                                                    onclick={on_action("retry")}
                                                >
                                                    { "Retry" }
                                                </div>
                                            </>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                                
                                {
                                    if task.status == TaskStatus::PENDING || task.status == TaskStatus::QUEUED {
                                        html! {
                                            <div 
                                                class="context-menu-item"
                                                style={menu_item_style}
                                                onmouseover={Self::create_mouse_hover_callback("#f5f5f5")}
                                                onmouseout={Self::create_mouse_hover_callback("")}
                                                onclick={on_action("skip")}
                                            >
                                                { "Skip" }
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                                
                                {
                                    if task.status == TaskStatus::RUNNING {
                                        html! {
                                            <div 
                                                class="context-menu-item"
                                                style={menu_item_style}
                                                onmouseover={Self::create_mouse_hover_callback("#f5f5f5")}
                                                onmouseout={Self::create_mouse_hover_callback("")}
                                                onclick={on_action("pause")}
                                            >
                                                { "Pause" }
                                            </div>
                                        }
                                    } else if task.status == TaskStatus::PAUSED {
                                        html! {
                                            <div 
                                                class="context-menu-item"
                                                style={menu_item_style}
                                                onmouseover={Self::create_mouse_hover_callback("#f5f5f5")}
                                                onmouseout={Self::create_mouse_hover_callback("")}
                                                onclick={on_action("play")}
                                            >
                                                { "Resume" }
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                            </>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    }
}