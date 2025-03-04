use crate::models::dag::{DAGGraph as DAGGraphModel, TaskStatus};
use crate::utils::api;
use crate::utils::cytoscape::{
    Cytoscape, CytoscapeEdge, CytoscapeLayout, CytoscapeNode, CytoscapeStyle, 
    NodeContextMenuEvent, highlight_connected_edges, reset_highlights
};
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;
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
    HighlightConnections(String), // task_id
    ResetHighlights,
    StoreGraph(JsValue),
}

pub struct DAGGraph {
    graph: Option<DAGGraphModel>,
    loading: bool,
    error: Option<String>,
    context_menu_visible: bool,
    context_menu_position: (i32, i32),
    selected_task: Option<String>,
    cy_instance: Option<JsValue>,
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
            context_menu_visible: false,
            context_menu_position: (0, 0),
            selected_task: None,
            cy_instance: None,
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
            Msg::HighlightConnections(task_id) => {
                if let Some(cy) = &self.cy_instance {
                    let _ = highlight_connected_edges(cy, &task_id, "#ff0000");
                }
                false
            }
            Msg::ResetHighlights => {
                if let Some(cy) = &self.cy_instance {
                    let _ = reset_highlights(cy);
                }
                false
            }
            Msg::StoreGraph(cy) => {
                self.cy_instance = Some(cy);
                false
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
                                { self.render_graph(ctx) }
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
}

impl DAGGraph {
    // Helper function for handling mouse events
    fn create_mouse_hover_callback(bg_color: &'static str) -> Callback<MouseEvent> {
        Callback::from(move |e: MouseEvent| {
            if let Some(target) = e.target() {
                if let Ok(el) = target.dyn_into::<web_sys::HtmlElement>() {
                    let style = el.style();
                    let _ = style.set_property("background-color", bg_color);
                }
            }
        })
    }
    
    fn render_graph(&self, ctx: &Context<Self>) -> Html {
        if let Some(graph) = &self.graph {
            // Create nodes for Cytoscape
            let mut nodes = Vec::new();
            let mut edges = Vec::new();
            
            // Add nodes
            for task in &graph.tasks {
                let node = CytoscapeNode::new(&task.id, &task.name)
                    .with_data("status", format!("{:?}", task.status))
                    .with_data("taskId", task.id.clone());
                
                nodes.push(node);
            }
            
            // Add edges
            for edge in &graph.edges {
                let edge_obj = CytoscapeEdge::new(&edge.source, &edge.target);
                edges.push(edge_obj);
            }
            
            // Create styles
            let mut styles = vec![
                // Node style
                CytoscapeStyle::new("node")
                    .with_property("content", "data(label)")
                    .with_property("text-valign", "center")
                    .with_property("text-halign", "center")
                    .with_property("padding", "8px")
                    .with_property("shape", "roundrectangle")
                    .with_property("text-wrap", "wrap")
                    .with_property("text-max-width", 100.0)
                    .with_property("font-size", "12px"),
                
                // Edge style
                CytoscapeStyle::new("edge")
                    .with_property("width", 2.0)
                    .with_property("line-color", "#ccc")
                    .with_property("target-arrow-color", "#ccc")
                    .with_property("target-arrow-shape", "triangle")
                    .with_property("curve-style", "bezier"),
            ];
            
            // Add status-specific styles
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
                let status_style = CytoscapeStyle::new(&status_selector)
                    .with_property("background-color", status.color())
                    .with_property("color", "white");
                
                styles.push(status_style);
            }
            
            // Create layout
            let node_count = graph.tasks.len();
            let mut layout = CytoscapeLayout::new("dagre")
                .with_option("rankDir", "LR")
                .with_option("edgeSep", 50.0)
                .with_option("fit", true);
            
            // Adjust layout based on graph size
            if node_count > 500 {
                // Very large graphs: minimal spacing
                layout = layout
                    .with_option("nodeSep", 20.0)
                    .with_option("rankSep", 40.0)
                    .with_option("padding", 5.0)
                    .with_option("animate", false);
            } else if node_count > 100 {
                // Large graphs: reduced spacing
                layout = layout
                    .with_option("nodeSep", 30.0)
                    .with_option("rankSep", 60.0)
                    .with_option("padding", 10.0)
                    .with_option("animate", true)
                    .with_option("animationDuration", 300.0);
            } else {
                // Small/medium graphs: standard spacing
                layout = layout
                    .with_option("nodeSep", 40.0)
                    .with_option("rankSep", 80.0)
                    .with_option("padding", 20.0)
                    .with_option("animate", true)
                    .with_option("animationDuration", 500.0);
            }
            
            // Handle node context menu
            let on_context_menu = {
                let link = ctx.link().clone();
                Callback::from(move |evt: NodeContextMenuEvent| {
                    link.send_message(Msg::ShowContextMenu(evt.event, evt.node_id));
                })
            };
            
            // Handle node clicks for highlighting
            let on_node_click = {
                let link = ctx.link().clone();
                Callback::from(move |evt: crate::utils::cytoscape::NodeClickEvent| {
                    link.send_message(Msg::HighlightConnections(evt.node_id));
                })
            };
            
            // Background click for clearing highlights
            let _on_init = {
                let link = ctx.link().clone();
                Callback::from(move |_cy: JsValue| {
                    // Store the Cytoscape instance
                    link.send_message_batch(vec![
                        Msg::ResetHighlights,
                    ]);
                })
            };
            
            // Define zoom settings based on graph size
            let (min_zoom, max_zoom, initial_zoom) = if node_count > 500 {
                (0.1, 2.5, Some(0.4))
            } else if node_count > 100 {
                (0.1, 2.5, Some(0.6))
            } else {
                (0.1, 2.5, Some(0.8))
            };
            
            // Store the Cytoscape instance
            let on_cy_init = {
                let link = ctx.link().clone();
                Callback::from(move |cy: JsValue| {
                    link.send_message_batch(vec![
                        Msg::StoreGraph(cy),
                    ]);
                })
            };
            
            html! {
                <Cytoscape
                    id={format!("dag-{}", ctx.props().dag_id)}
                    class="cytoscape-container"
                    style="height: 600px; width: 100%; border: 1px solid #e5e7eb; border-radius: 0.375rem;"
                    {nodes}
                    {edges}
                    {styles}
                    layout={Some(layout)}
                    on_node_click={Some(on_node_click)}
                    on_node_context_menu={Some(on_context_menu)}
                    on_init={Some(on_cy_init)}
                    min_zoom={Some(min_zoom)}
                    max_zoom={Some(max_zoom)}
                    initial_zoom={initial_zoom}
                />
            }
        } else {
            html! {
                <div class="p-4 text-center text-gray-500">
                    {"No graph data available"}
                </div>
            }
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