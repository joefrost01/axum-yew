use js_sys::{Array, Function, Object, Reflect};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MouseEvent};
use yew::prelude::*;
use serde::{Serialize, Deserialize};

/// Event emitted when a node is clicked
#[derive(Clone)]
pub struct NodeClickEvent {
    pub node_id: String,
    pub event: MouseEvent,
}

/// Event emitted when a node is right-clicked for context menu
#[derive(Clone)]
pub struct NodeContextMenuEvent {
    pub node_id: String,
    pub event: MouseEvent,
}

/// Style definition for Cytoscape elements
#[derive(Clone, Debug, PartialEq)]
pub struct CytoscapeStyle {
    pub selector: String,
    pub properties: HashMap<String, JsValue>,
}

impl CytoscapeStyle {
    pub fn new(selector: &str) -> Self {
        Self {
            selector: selector.to_string(),
            properties: HashMap::new(),
        }
    }

    pub fn with_property(mut self, key: &str, value: impl Into<JsValue>) -> Self {
        self.properties.insert(key.to_string(), value.into());
        self
    }

    /// Converts the style to a JS Object
    pub fn to_js_object(&self) -> Result<Object, JsValue> {
        let style_obj = Object::new();
        Reflect::set(&style_obj, &JsValue::from_str("selector"), &JsValue::from_str(&self.selector))?;
        
        let props = Object::new();
        for (key, value) in &self.properties {
            Reflect::set(&props, &JsValue::from_str(key), value)?;
        }
        
        Reflect::set(&style_obj, &JsValue::from_str("style"), &props)?;
        Ok(style_obj)
    }
}

/// Layout options for Cytoscape
#[derive(Clone, Debug, PartialEq)]
pub struct CytoscapeLayout {
    pub name: String,
    pub options: HashMap<String, JsValue>,
}

impl CytoscapeLayout {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            options: HashMap::new(),
        }
    }

    pub fn with_option(mut self, key: &str, value: impl Into<JsValue>) -> Self {
        self.options.insert(key.to_string(), value.into());
        self
    }

    /// Converts the layout to a JS Object
    pub fn to_js_object(&self) -> Result<Object, JsValue> {
        let layout_obj = Object::new();
        Reflect::set(&layout_obj, &JsValue::from_str("name"), &JsValue::from_str(&self.name))?;
        
        for (key, value) in &self.options {
            Reflect::set(&layout_obj, &JsValue::from_str(key), value)?;
        }
        
        Ok(layout_obj)
    }
}

/// Node data for Cytoscape
#[derive(Clone, Debug, PartialEq)]
pub struct CytoscapeNode {
    pub id: String,
    pub label: String,
    pub data: HashMap<String, JsValue>,
}

impl CytoscapeNode {
    pub fn new(id: &str, label: &str) -> Self {
        let mut data = HashMap::new();
        data.insert("id".to_string(), JsValue::from_str(id));
        data.insert("label".to_string(), JsValue::from_str(label));
        
        Self {
            id: id.to_string(),
            label: label.to_string(),
            data,
        }
    }

    pub fn with_data(mut self, key: &str, value: impl Into<JsValue>) -> Self {
        self.data.insert(key.to_string(), value.into());
        self
    }

    /// Converts the node to a JS Object
    pub fn to_js_object(&self) -> Result<Object, JsValue> {
        let node_obj = Object::new();
        let data_obj = Object::new();
        
        for (key, value) in &self.data {
            Reflect::set(&data_obj, &JsValue::from_str(key), value)?;
        }
        
        Reflect::set(&node_obj, &JsValue::from_str("data"), &data_obj)?;
        Reflect::set(&node_obj, &JsValue::from_str("group"), &JsValue::from_str("nodes"))?;
        
        Ok(node_obj)
    }
    
    #[allow(dead_code)]
    pub fn to_js_value(&self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from(self.to_js_object()?))
    }
}

/// Edge data for Cytoscape
#[derive(Clone, Debug, PartialEq)]
pub struct CytoscapeEdge {
    pub source: String,
    pub target: String,
    pub data: HashMap<String, JsValue>,
}

impl CytoscapeEdge {
    pub fn new(source: &str, target: &str) -> Self {
        let mut data = HashMap::new();
        data.insert("source".to_string(), JsValue::from_str(source));
        data.insert("target".to_string(), JsValue::from_str(target));
        
        Self {
            source: source.to_string(),
            target: target.to_string(),
            data,
        }
    }

    pub fn with_data(mut self, key: &str, value: impl Into<JsValue>) -> Self {
        self.data.insert(key.to_string(), value.into());
        self
    }

    /// Converts the edge to a JS Object
    pub fn to_js_object(&self) -> Result<Object, JsValue> {
        let edge_obj = Object::new();
        let data_obj = Object::new();
        
        for (key, value) in &self.data {
            Reflect::set(&data_obj, &JsValue::from_str(key), value)?;
        }
        
        Reflect::set(&edge_obj, &JsValue::from_str("data"), &data_obj)?;
        Reflect::set(&edge_obj, &JsValue::from_str("group"), &JsValue::from_str("edges"))?;
        
        Ok(edge_obj)
    }
    
    #[allow(dead_code)]
    pub fn to_js_value(&self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from(self.to_js_object()?))
    }
}

/// Properties for the Cytoscape component
#[derive(Properties, PartialEq)]
pub struct CytoscapeProps {
    pub id: String,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub nodes: Vec<CytoscapeNode>,
    #[prop_or_default]
    pub edges: Vec<CytoscapeEdge>,
    #[prop_or_default]
    pub styles: Vec<CytoscapeStyle>,
    #[prop_or_default]
    pub layout: Option<CytoscapeLayout>,
    #[prop_or_default]
    pub on_node_click: Option<Callback<NodeClickEvent>>,
    #[prop_or_default]
    pub on_node_context_menu: Option<Callback<NodeContextMenuEvent>>,
    #[prop_or_default]
    pub on_init: Option<Callback<JsValue>>,
    #[prop_or_default]
    pub min_zoom: Option<f64>,
    #[prop_or_default]
    pub max_zoom: Option<f64>,
    #[prop_or_default]
    pub initial_zoom: Option<f64>,
    #[prop_or_default]
    pub style: Option<String>,
}

/// A safe Rust wrapper around Cytoscape.js
#[function_component(Cytoscape)]
pub fn cytoscape(props: &CytoscapeProps) -> Html {
    let cy_ref = use_state(|| None::<JsValue>);
    
    // Generate a unique container ID if not provided
    let container_id = format!("cytoscape-container-{}", &props.id);
    
    // Make a clone of props we need for our hooks
    let id = props.id.clone();
    let nodes = props.nodes.clone();
    let edges = props.edges.clone();
    let styles = props.styles.clone();
    let layout = props.layout.clone();
    let class = props.class.clone();
    let style = props.style.clone();
    let min_zoom = props.min_zoom;
    let max_zoom = props.max_zoom;
    let initial_zoom = props.initial_zoom;
    let on_node_click = props.on_node_click.clone();
    let on_node_context_menu = props.on_node_context_menu.clone();
    let on_init = props.on_init.clone();
    
    // Initialize and update Cytoscape when props change
    {
        let cy_ref = cy_ref.clone();
        let container_id = container_id.clone();
        let container_id = container_id.clone();
        
        use_effect_with(
            (nodes.clone(), edges.clone(), styles.clone(), layout.clone()),
            move |(nodes, edges, styles, layout)| {
                let container_id = container_id.clone();
                let min_zoom = min_zoom;
                let max_zoom = max_zoom; 
                let initial_zoom = initial_zoom;
                let on_node_click = on_node_click.clone();
                let on_node_context_menu = on_node_context_menu.clone();
                let on_init = on_init.clone();
                let cy_ref = cy_ref.clone();
                
                // Initialization or update function
                let init_or_update = move || {
                    // Check if Cytoscape.js is loaded
                    let window = web_sys::window().expect("no global window exists");
                    if !Reflect::has(&window, &JsValue::from_str("cytoscape"))
                        .unwrap_or(false) 
                    {
                        log::error!("Cytoscape.js is not loaded. Make sure to include it in your HTML.");
                        return;
                    }
                    
                    let document = window.document().expect("no document on window");
                    let container = document.get_element_by_id(&container_id)
                        .expect(&format!("Cytoscape container with id {} not found", container_id));
                    
                    // Create the configuration object
                    let config = Object::new();
                    Reflect::set(&config, &JsValue::from_str("container"), &container).unwrap();
                    
                    // Add elements (nodes and edges)
                    let js_elements = Array::new();
                    
                    for node in nodes.iter() {
                        if let Ok(node_obj) = node.to_js_object() {
                            js_elements.push(&node_obj);
                        }
                    }
                    
                    for edge in edges.iter() {
                        if let Ok(edge_obj) = edge.to_js_object() {
                            js_elements.push(&edge_obj);
                        }
                    }
                    
                    Reflect::set(&config, &JsValue::from_str("elements"), &js_elements).unwrap();
                    
                    // Add styles
                    let js_styles = Array::new();
                    
                    for style in styles.iter() {
                        if let Ok(style_obj) = style.to_js_object() {
                            js_styles.push(&style_obj);
                        }
                    }
                    
                    Reflect::set(&config, &JsValue::from_str("style"), &js_styles).unwrap();
                    
                    // Add layout if provided
                    if let Some(layout) = layout {
                        if let Ok(layout_obj) = layout.to_js_object() {
                            Reflect::set(&config, &JsValue::from_str("layout"), &layout_obj).unwrap();
                        }
                    }
                    
                    // Set zoom constraints if provided
                    if let Some(min_zoom_val) = min_zoom {
                        Reflect::set(&config, &JsValue::from_str("minZoom"), &JsValue::from_f64(min_zoom_val)).unwrap();
                    }
                    
                    if let Some(max_zoom_val) = max_zoom {
                        Reflect::set(&config, &JsValue::from_str("maxZoom"), &JsValue::from_f64(max_zoom_val)).unwrap();
                    }
                    
                    // Initialize Cytoscape with the configuration
                    let cytoscape_fn = Reflect::get(&window, &JsValue::from_str("cytoscape"))
                        .expect("cytoscape not found");
                    
                    let args = Array::of1(&config);
                    let cy = Reflect::apply(&Function::from(cytoscape_fn), &JsValue::NULL, &args)
                        .expect("failed to create cytoscape instance");
                    
                    // Set initial zoom if provided
                    if let Some(initial_zoom_val) = initial_zoom {
                        let js_code = format!(r#"
                            cy.zoom({{
                                level: {},
                                position: {{x: cy.width() / 2, y: cy.height() / 2}}
                            }});
                        "#, initial_zoom_val);
                        
                        let function = Function::new_with_args("cy", &js_code);
                        let _ = Reflect::apply(&function, &JsValue::NULL, &Array::of1(&cy));
                    }
                    
                    // Set up event handlers
                    if on_node_click.is_some() || on_node_context_menu.is_some() {
                        // Create a global event handler for node clicks
                        if let Some(node_click_callback) = &on_node_click {
                            let callback = node_click_callback.clone();
                            let on_click_fn = Closure::wrap(Box::new(move |event: MouseEvent, node_id: String| {
                                callback.emit(NodeClickEvent { node_id, event });
                            }) as Box<dyn FnMut(MouseEvent, String)>);
                            
                            Reflect::set(
                                &window,
                                &JsValue::from_str("handleCytoscapeNodeClick"),
                                &on_click_fn.as_ref().unchecked_ref(),
                            ).expect("Failed to set handleCytoscapeNodeClick function");
                            
                            on_click_fn.forget();
                            
                            let js_code = r#"
                                cy.on('tap', 'node', function(evt){
                                    var node = evt.target;
                                    var nodeId = node.id();
                                    window.handleCytoscapeNodeClick(evt.originalEvent, nodeId);
                                });
                            "#;
                            
                            let function = Function::new_with_args("cy", js_code);
                            let _ = Reflect::apply(&function, &JsValue::NULL, &Array::of1(&cy));
                        }
                        
                        // Create a global event handler for node context menus
                        if let Some(context_menu_callback) = &on_node_context_menu {
                            let callback = context_menu_callback.clone();
                            let on_context_menu_fn = Closure::wrap(Box::new(move |event: MouseEvent, node_id: String| {
                                callback.emit(NodeContextMenuEvent { node_id, event });
                            }) as Box<dyn FnMut(MouseEvent, String)>);
                            
                            Reflect::set(
                                &window,
                                &JsValue::from_str("handleCytoscapeNodeContextMenu"),
                                &on_context_menu_fn.as_ref().unchecked_ref(),
                            ).expect("Failed to set handleCytoscapeNodeContextMenu function");
                            
                            on_context_menu_fn.forget();
                            
                            let js_code = r#"
                                cy.on('cxttap', 'node', function(evt){
                                    var node = evt.target;
                                    var nodeId = node.id();
                                    window.handleCytoscapeNodeContextMenu(evt.originalEvent, nodeId);
                                });
                            "#;
                            
                            let function = Function::new_with_args("cy", js_code);
                            let _ = Reflect::apply(&function, &JsValue::NULL, &Array::of1(&cy));
                        }
                    }
                    
                    // Save the Cytoscape instance
                    cy_ref.set(Some(cy.clone()));
                    
                    // Call the onInit callback if provided
                    if let Some(init_callback) = &on_init {
                        init_callback.emit(cy);
                    }
                };
                
                // Run the init/update function
                init_or_update();
                
                || {}
            }
        );
    }
    
    // Combine the classes for the container
    let mut classes = class.clone();
    classes.push("cytoscape-container");
    
    // Combine the styles
    let style_str = if let Some(custom_style) = &style {
        format!("min-height: 500px; {}", custom_style)
    } else {
        "min-height: 500px;".to_string()
    };
    
    html! {
        <div id={container_id} class={classes} style={style_str}></div>
    }
}

/// Highlight a path in the graph
pub fn highlight_path(cy: &JsValue, path: &[String], color: &str) -> Result<(), JsValue> {
    let path_array = Array::new();
    for item in path {
        path_array.push(&JsValue::from_str(item));
    }
    
    let js_code = format!(
        r#"
        // Reset all edges
        cy.edges().style({{
            'line-color': '#ccc',
            'target-arrow-color': '#ccc',
            'width': 2
        }});
        
        // Highlight specific edges in the path
        var path = path_array;
        for (var i = 0; i < path.length - 1; i++) {{
            var source = path[i];
            var target = path[i+1];
            cy.edges(`[source = "${{source}}"][target = "${{target}}"]`).style({{
                'line-color': '{}',
                'target-arrow-color': '{}',
                'width': 4
            }});
        }}
        "#,
        color,
        color
    );
    
    let function = Function::new_with_args("cy, path_array", &js_code);
    Reflect::apply(&function, &JsValue::NULL, &Array::of2(cy, &path_array))?;
    
    Ok(())
}

/// Highlight connected edges for a node
pub fn highlight_connected_edges(cy: &JsValue, node_id: &str, color: &str) -> Result<(), JsValue> {
    let js_code = format!(
        r#"
        // Reset all edges
        cy.edges().style({{
            'line-color': '#ccc',
            'target-arrow-color': '#ccc',
            'width': 2
        }});
        
        // Get the node and highlight its connected edges
        var node = cy.getElementById('{}');
        node.connectedEdges().style({{
            'line-color': '{}',
            'target-arrow-color': '{}',
            'width': 4
        }});
        "#,
        node_id,
        color,
        color
    );
    
    let function = Function::new_with_args("cy", &js_code);
    Reflect::apply(&function, &JsValue::NULL, &Array::of1(cy))?;
    
    Ok(())
}

/// Reset all highlighting in the graph
pub fn reset_highlights(cy: &JsValue) -> Result<(), JsValue> {
    let js_code = r#"
        cy.edges().style({
            'line-color': '#ccc',
            'target-arrow-color': '#ccc',
            'width': 2
        });
    "#;
    
    let function = Function::new_with_args("cy", js_code);
    Reflect::apply(&function, &JsValue::NULL, &Array::of1(cy))?;
    
    Ok(())
}

/// Center the graph
pub fn center_graph(cy: &JsValue) -> Result<(), JsValue> {
    let js_code = r#"
        cy.fit();
        cy.center();
    "#;
    
    let function = Function::new_with_args("cy", js_code);
    Reflect::apply(&function, &JsValue::NULL, &Array::of1(cy))?;
    
    Ok(())
}

/// Zoom to a specific node
pub fn zoom_to_node(cy: &JsValue, node_id: &str, zoom_level: f64) -> Result<(), JsValue> {
    let js_code = format!(
        r#"
        var node = cy.getElementById('{}');
        if (node.length > 0) {{
            cy.zoom({{
                level: {},
                position: node.position()
            }});
            cy.center(node);
        }}
        "#,
        node_id,
        zoom_level
    );
    
    let function = Function::new_with_args("cy", &js_code);
    Reflect::apply(&function, &JsValue::NULL, &Array::of1(cy))?;
    
    Ok(())
}