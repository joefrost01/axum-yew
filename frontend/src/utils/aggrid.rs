use js_sys::{Array, Function, Object, Reflect};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use std::collections::HashMap;
use std::fmt::Debug;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

/// Callback type for cell value changed events
pub type CellValueChangedCallback = Callback<GridCellChangeEvent>;

/// Callback type for cell clicked events
pub type CellClickedCallback = Callback<GridCellClickEvent>;

/// Callback type for row selection events
pub type RowSelectionCallback = Callback<Vec<JsValue>>;

/// Callback type for pagination changed events
pub type PaginationChangedCallback = Callback<GridPaginationEvent>;

/// Sorting direction for a column
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum SortDirection {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

/// Column definition for AG-Grid
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColumnDef {
    pub field: String,
    #[serde(rename = "headerName")]
    pub header_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sortable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resizable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cellClass")]
    pub cell_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkboxable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin: Option<bool>,
    #[serde(skip)]
    pub extra_props: HashMap<String, JsValue>,
}

impl ColumnDef {
    pub fn new(field: &str, header_name: &str) -> Self {
        Self {
            field: field.to_string(),
            header_name: header_name.to_string(),
            width: None,
            sortable: None,
            filter: None,
            resizable: None,
            editable: None,
            sort: None,
            cell_class: None,
            checkboxable: None,
            pin: None,
            extra_props: HashMap::new(),
        }
    }

    pub fn with_width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = Some(sortable);
        self
    }

    pub fn filter(mut self, filter: bool) -> Self {
        self.filter = Some(filter);
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = Some(resizable);
        self
    }

    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = Some(editable);
        self
    }

    pub fn with_sort(mut self, direction: SortDirection) -> Self {
        self.sort = Some(direction);
        self
    }

    pub fn with_cell_class(mut self, cell_class: &str) -> Self {
        self.cell_class = Some(cell_class.to_string());
        self
    }

    pub fn checkboxable(mut self, checkboxable: bool) -> Self {
        self.checkboxable = Some(checkboxable);
        self
    }

    pub fn with_pin(mut self, pin: bool) -> Self {
        self.pin = Some(pin);
        self
    }

    pub fn with_extra_prop(mut self, key: &str, value: impl Into<JsValue>) -> Self {
        self.extra_props.insert(key.to_string(), value.into());
        self
    }

    /// Converts the column definition to a JS Object
    pub fn to_js_object(&self) -> Result<Object, JsValue> {
        let col_obj = Object::new();
        
        Reflect::set(&col_obj, &JsValue::from_str("field"), &JsValue::from_str(&self.field))?;
        Reflect::set(&col_obj, &JsValue::from_str("headerName"), &JsValue::from_str(&self.header_name))?;
        
        if let Some(width) = self.width {
            Reflect::set(&col_obj, &JsValue::from_str("width"), &JsValue::from_f64(width as f64))?;
        }
        
        if let Some(sortable) = self.sortable {
            Reflect::set(&col_obj, &JsValue::from_str("sortable"), &JsValue::from_bool(sortable))?;
        }
        
        if let Some(filter) = self.filter {
            Reflect::set(&col_obj, &JsValue::from_str("filter"), &JsValue::from_bool(filter))?;
        }
        
        if let Some(resizable) = self.resizable {
            Reflect::set(&col_obj, &JsValue::from_str("resizable"), &JsValue::from_bool(resizable))?;
        }
        
        if let Some(editable) = self.editable {
            Reflect::set(&col_obj, &JsValue::from_str("editable"), &JsValue::from_bool(editable))?;
        }
        
        if let Some(sort) = &self.sort {
            let sort_str = match sort {
                SortDirection::Ascending => "asc",
                SortDirection::Descending => "desc",
            };
            Reflect::set(&col_obj, &JsValue::from_str("sort"), &JsValue::from_str(sort_str))?;
        }
        
        if let Some(cell_class) = &self.cell_class {
            Reflect::set(&col_obj, &JsValue::from_str("cellClass"), &JsValue::from_str(cell_class))?;
        }
        
        if let Some(checkboxable) = self.checkboxable {
            if checkboxable {
                Reflect::set(&col_obj, &JsValue::from_str("checkboxSelection"), &JsValue::from_bool(true))?;
            }
        }
        
        if let Some(pin) = self.pin {
            if pin {
                Reflect::set(&col_obj, &JsValue::from_str("pinned"), &JsValue::from_str("left"))?;
            }
        }
        
        // Add any extra properties
        for (key, value) in &self.extra_props {
            Reflect::set(&col_obj, &JsValue::from_str(key), value)?;
        }
        
        Ok(col_obj)
    }
}

/// Event data for cell value changes
#[derive(Clone, Debug)]
pub struct GridCellChangeEvent {
    pub node_id: String,
    pub data: JsValue,
    pub old_value: JsValue,
    pub new_value: JsValue,
    pub column: String,
}

/// Event data for cell clicks
#[derive(Clone, Debug)]
pub struct GridCellClickEvent {
    pub node_id: String,
    pub data: JsValue,
    pub value: JsValue,
    pub column: String,
}

/// Event data for pagination changes
#[derive(Clone, Debug)]
pub struct GridPaginationEvent {
    pub current_page: usize,
    pub page_size: usize,
    pub total_pages: usize,
    pub row_count: usize,
}

/// Properties for the AG-Grid component
#[derive(Properties, PartialEq)]
pub struct AgGridProps {
    pub id: String,
    #[prop_or_default]
    pub class: Classes,
    /// Row data as JS values 
    #[prop_or_default]
    pub row_data: Vec<JsValue>,
    /// Column definitions
    pub column_defs: Vec<ColumnDef>,
    /// Height style for the grid
    #[prop_or_default]
    pub height: Option<String>,
    /// Enable pagination
    #[prop_or_default]
    pub pagination: bool,
    /// Number of rows per page
    #[prop_or_default]
    pub page_size: Option<usize>,
    /// Enable row selection
    #[prop_or_default]
    pub row_selection: bool,
    /// Selection mode: "single" or "multiple"
    #[prop_or_default]
    pub selection_mode: Option<String>,
    /// Dark theme
    #[prop_or_default]
    pub dark_mode: bool,
    /// Custom grid theme
    #[prop_or_default]
    pub theme: Option<String>,
    /// Custom grid options
    #[prop_or_default]
    pub custom_options: Option<HashMap<String, JsValue>>,
    /// Callback for cell value changes
    #[prop_or_default]
    pub on_cell_value_changed: Option<CellValueChangedCallback>,
    /// Callback for cell clicks
    #[prop_or_default]
    pub on_cell_clicked: Option<CellClickedCallback>,
    /// Callback for row selection changes
    #[prop_or_default]
    pub on_selection_changed: Option<RowSelectionCallback>,
    /// Callback for pagination changes
    #[prop_or_default]
    pub on_pagination_changed: Option<PaginationChangedCallback>,
    /// Callback when grid is ready
    #[prop_or_default]
    pub on_grid_ready: Option<Callback<JsValue>>,
}

/// A Yew component wrapper for AG-Grid
#[function_component(AgGrid)]
pub fn ag_grid(props: &AgGridProps) -> Html {
    let id = props.id.clone();
    let container_id = format!("ag-grid-container-{}", &id);
    let class = props.class.clone();
    let height_str = match &props.height {
        Some(h) => h.clone(),
        None => "600px".to_string()
    };
    
    let styles = format!(
        "width: 100%; height: {}; min-height: 400px; display: block;", 
        height_str
    );
    
    // Store these in a struct that can be moved into the effect closure
    let grid_options = GridOptions {
        row_data: props.row_data.clone(),
        column_defs: props.column_defs.clone(),
        pagination: props.pagination,
        page_size: props.page_size,
        dark_mode: props.dark_mode,
        row_selection: props.row_selection,
        selection_mode: props.selection_mode.clone(),
        theme: props.theme.clone(),
        custom_options: props.custom_options.clone(),
        on_cell_value_changed: props.on_cell_value_changed.clone(),
        on_cell_clicked: props.on_cell_clicked.clone(),
        on_selection_changed: props.on_selection_changed.clone(),
        on_pagination_changed: props.on_pagination_changed.clone(),
        on_grid_ready: props.on_grid_ready.clone(),
    };
    
    // Store and init GridAPI
    let grid_api = use_state(|| None::<JsValue>);
    let grid_api_for_effect = grid_api.clone();
    
    // Create and mount the grid when the component mounts - with dependency tracking to avoid reinitialization
    let grid_initialized = use_state(|| false);
    let initialized = grid_initialized.clone();
    
    use_effect_with(
        (container_id.clone(), grid_options, grid_initialized.clone()),
        move |(container_id, grid_options, initialized)| {
            if !**initialized {
                web_sys::console::log_1(&"Initializing AG-Grid (first time)...".into());
                init_grid(container_id, grid_options, grid_api_for_effect);
                initialized.set(true);
            }
            || ()
        }
    );
    
    // Debug logging for the mounted container
    let container_id_clone = container_id.clone();
    use_effect_with(
        (),
        move |_| {
            web_sys::console::log_1(&format!("AG-Grid container mounted with ID: {}", container_id_clone).into());
            || ()
        }
    );

    // Render the grid container
    html! {
        <div id={container_id} class={class} style={styles}></div>
    }
}

// Helper struct to make it easier to move the props
#[derive(Clone, PartialEq)]
struct GridOptions {
    row_data: Vec<JsValue>,
    column_defs: Vec<ColumnDef>,
    pagination: bool,
    page_size: Option<usize>,
    dark_mode: bool,
    row_selection: bool,
    selection_mode: Option<String>,
    theme: Option<String>,
    custom_options: Option<HashMap<String, JsValue>>,
    on_cell_value_changed: Option<CellValueChangedCallback>,
    on_cell_clicked: Option<CellClickedCallback>,
    on_selection_changed: Option<RowSelectionCallback>,
    on_pagination_changed: Option<PaginationChangedCallback>,
    on_grid_ready: Option<Callback<JsValue>>,
}

// Separate function to initialize the grid
fn init_grid(container_id: &str, grid_options: &GridOptions, grid_api: UseStateHandle<Option<JsValue>>) {
    web_sys::console::log_1(&"Initializing AG-Grid...".into());
    
    // Check if AG-Grid is loaded
    let window = match web_sys::window() {
        Some(w) => w,
        None => {
            web_sys::console::error_1(&"No global window exists".into());
            return;
        }
    };
    
    if !Reflect::has(&window, &JsValue::from_str("agGrid"))
        .unwrap_or(false) 
    {
        let error_msg = "AG-Grid is not loaded. Make sure to include it in your HTML.";
        log::error!("{}", error_msg);
        web_sys::console::error_1(&error_msg.into());
        return;
    }
    
    // Log AG-Grid version
    if let Ok(ag_grid_obj) = Reflect::get(&window, &JsValue::from_str("agGrid")) {
        if let Ok(version) = Reflect::get(&ag_grid_obj, &JsValue::from_str("version")) {
            web_sys::console::log_1(&format!("AG-Grid version: {:?}", version).into());
        }
    }
    
    let document = match window.document() {
        Some(doc) => doc,
        None => {
            web_sys::console::error_1(&"No document found on window".into());
            return;
        }
    };
    
    // Try to find the container element
    let container_element_ref = match document.get_element_by_id(container_id) {
        Some(el) => {
            web_sys::console::log_1(&format!("Container found with id: {}", container_id).into());
            el
        },
        None => {
            let error_msg = format!("AG-Grid container with id {} not found", container_id);
            web_sys::console::error_1(&error_msg.into());
            return;
        }
    };
    
    // Convert column definitions to JS array
    let js_columns = Array::new();
    for col in grid_options.column_defs.iter() {
        if let Ok(col_obj) = col.to_js_object() {
            js_columns.push(&col_obj);
        }
    }
    
    // Create the grid options
    let js_grid_options = Object::new();
    
    // Set basic options
    let _ = Reflect::set(&js_grid_options, &JsValue::from_str("columnDefs"), &js_columns);
    
    // Ensure row data is not empty and log for debugging
    let row_data_array = Array::from_iter(grid_options.row_data.iter());
    web_sys::console::log_1(&format!("Setting row data with {} elements", row_data_array.length()).into());
    if row_data_array.length() > 0 {
        web_sys::console::log_1(&format!("First row: {:?}", row_data_array.get(0)).into());
    }
    
    let _ = Reflect::set(&js_grid_options, &JsValue::from_str("rowData"), &row_data_array);
    
    // Configure pagination
    if grid_options.pagination {
        let _ = Reflect::set(&js_grid_options, &JsValue::from_str("pagination"), &JsValue::from_bool(true));
        
        if let Some(size) = grid_options.page_size {
            let _ = Reflect::set(&js_grid_options, &JsValue::from_str("paginationPageSize"), 
                               &JsValue::from_f64(size as f64));
        }
    }
    
    // Configure row selection
    if grid_options.row_selection {
        let selection_type = grid_options.selection_mode.clone()
            .unwrap_or_else(|| "multiple".to_string());
        let _ = Reflect::set(&js_grid_options, &JsValue::from_str("rowSelection"), 
                           &JsValue::from_str(&selection_type));
    }
    
    // Configure theme
    let mut theme_classes = vec![];
    
    if grid_options.dark_mode {
        theme_classes.push("ag-theme-alpine-dark");
    } else {
        theme_classes.push("ag-theme-alpine");
    }
    
    if let Some(custom_theme) = &grid_options.theme {
        theme_classes.push(custom_theme);
    }
    
    let theme_class = theme_classes.join(" ");
    
    // Add any custom options
    if let Some(custom_options_map) = &grid_options.custom_options {
        for (key, value) in custom_options_map {
            let _ = Reflect::set(&js_grid_options, &JsValue::from_str(key), value);
        }
    }
    
    // Add event handlers
    
    // Setup cell value changed handler
    if let Some(on_cell_value_changed) = &grid_options.on_cell_value_changed {
        let callback = on_cell_value_changed.clone();
        let handler = Closure::wrap(Box::new(move |params: JsValue| {
            handle_cell_value_changed(params, &callback);
        }) as Box<dyn FnMut(JsValue)>);
        
        let _ = Reflect::set(
            &js_grid_options,
            &JsValue::from_str("onCellValueChanged"),
            &handler.as_ref().unchecked_ref(),
        );
        
        handler.forget();
    }
    
    // Setup cell clicked handler
    if let Some(on_cell_clicked) = &grid_options.on_cell_clicked {
        let callback = on_cell_clicked.clone();
        let handler = Closure::wrap(Box::new(move |params: JsValue| {
            handle_cell_clicked(params, &callback);
        }) as Box<dyn FnMut(JsValue)>);
        
        let _ = Reflect::set(
            &js_grid_options,
            &JsValue::from_str("onCellClicked"),
            &handler.as_ref().unchecked_ref(),
        );
        
        handler.forget();
    }
    
    // Setup selection changed handler
    if let Some(on_selection_changed) = &grid_options.on_selection_changed {
        let callback = on_selection_changed.clone();
        let handler = Closure::wrap(Box::new(move |params: JsValue| {
            handle_selection_changed(params, &callback);
        }) as Box<dyn FnMut(JsValue)>);
        
        let _ = Reflect::set(
            &js_grid_options,
            &JsValue::from_str("onSelectionChanged"),
            &handler.as_ref().unchecked_ref(),
        );
        
        handler.forget();
    }
    
    // Setup pagination changed handler
    if let Some(on_pagination_changed) = &grid_options.on_pagination_changed {
        let callback = on_pagination_changed.clone();
        let handler = Closure::wrap(Box::new(move |params: JsValue| {
            handle_pagination_changed(params, &callback);
        }) as Box<dyn FnMut(JsValue)>);
        
        let _ = Reflect::set(
            &js_grid_options,
            &JsValue::from_str("onPaginationChanged"),
            &handler.as_ref().unchecked_ref(),
        );
        
        handler.forget();
    }
    
    // Setup grid ready handler
    let api_state = grid_api.clone();
    let on_grid_ready_opt = grid_options.on_grid_ready.clone();
    let grid_ready_handler = Closure::wrap(Box::new(move |params: JsValue| {
        handle_grid_ready(params, &api_state, &on_grid_ready_opt);
    }) as Box<dyn FnMut(JsValue)>);
    
    let _ = Reflect::set(
        &js_grid_options,
        &JsValue::from_str("onGridReady"),
        &grid_ready_handler.as_ref().unchecked_ref(),
    );
    
    grid_ready_handler.forget();
    
    // Create a new AG-Grid instance
    if let Ok(ag_grid) = Reflect::get(&window, &JsValue::from_str("agGrid")) {
        // Create a clone for the element operations
        let container_element = container_element_ref.clone();
        
        // Update container classes for theme
        if let Ok(element) = container_element.dyn_into::<HtmlElement>() {
            // Add theme class manually
            let class_attr = format!("{} {}", element.class_name(), theme_class);
            element.set_class_name(&class_attr);
            
            // Set fixed height for grid container
            let _ = element.style().set_property("height", "600px");
            let _ = element.style().set_property("width", "100%");
        }
        
        // Create the grid
        web_sys::console::log_1(&"Creating AG-Grid instance...".into());
        
        // Debug the row data
        web_sys::console::log_1(&"Row data for grid:".into());
        let row_data_array = js_sys::Array::from_iter(grid_options.row_data.iter());
        web_sys::console::log_1(&format!("Row data length: {}", row_data_array.length()).into());
        
        if row_data_array.length() > 0 {
            web_sys::console::log_1(&row_data_array.get(0));
        }
        
        // Log grid options for debugging
        web_sys::console::log_1(&"Grid options:".into());
        web_sys::console::log_1(&js_grid_options);
        
        if let Ok(grid) = Reflect::get(&ag_grid, &JsValue::from_str("Grid")) {
            let grid_constructor = Function::from(grid);
            
            web_sys::console::log_1(&format!("Grid constructor found, creating grid in container: {}", container_id).into());
            
            // Instantiate the grid using 'new' keyword
            let js_code = r#"
                try {
                    console.log("Creating grid with row data:", gridOptions.rowData);
                    return new gridConstructor(container, gridOptions);
                } catch (error) {
                    console.error("Error creating grid instance:", error);
                    throw error;
                }
            "#;
            
            let create_grid_fn = Function::new_with_args("gridConstructor, container, gridOptions", js_code);
            
            match Reflect::apply(
                &create_grid_fn,
                &JsValue::NULL,
                &Array::of3(&grid_constructor, &container_element_ref, &js_grid_options),
            ) {
                Ok(_) => {
                    web_sys::console::log_1(&"AG-Grid instance created successfully".into());
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("Error creating grid: {:?}", e).into());
                }
            }
        } else {
            web_sys::console::error_1(&"Could not find ag-Grid.Grid constructor".into());
        }
    }
}

// Handler functions for events
fn handle_cell_value_changed(params: JsValue, callback: &CellValueChangedCallback) {
    if let Ok(obj) = Reflect::get(&params, &JsValue::from_str("node")) {
        if let Ok(node_id) = Reflect::get(&obj, &JsValue::from_str("id")) {
            let node_id_str = node_id.as_string().unwrap_or_default();
            
            let data = Reflect::get(&params, &JsValue::from_str("data"))
                .unwrap_or(JsValue::NULL);
            
            let old_value = Reflect::get(&params, &JsValue::from_str("oldValue"))
                .unwrap_or(JsValue::NULL);
            
            let new_value = Reflect::get(&params, &JsValue::from_str("newValue"))
                .unwrap_or(JsValue::NULL);
            
            let column = Reflect::get(&params, &JsValue::from_str("column"))
                .unwrap_or(JsValue::NULL);
            
            let column_field = if column.is_object() {
                Reflect::get(&column, &JsValue::from_str("colId"))
                    .unwrap_or(JsValue::from_str(""))
                    .as_string()
                    .unwrap_or_default()
            } else {
                "".to_string()
            };
            
            callback.emit(GridCellChangeEvent {
                node_id: node_id_str,
                data,
                old_value,
                new_value,
                column: column_field,
            });
        }
    }
}

fn handle_cell_clicked(params: JsValue, callback: &CellClickedCallback) {
    if let Ok(obj) = Reflect::get(&params, &JsValue::from_str("node")) {
        if let Ok(node_id) = Reflect::get(&obj, &JsValue::from_str("id")) {
            let node_id_str = node_id.as_string().unwrap_or_default();
            
            let data = Reflect::get(&params, &JsValue::from_str("data"))
                .unwrap_or(JsValue::NULL);
            
            let value = Reflect::get(&params, &JsValue::from_str("value"))
                .unwrap_or(JsValue::NULL);
            
            let column = Reflect::get(&params, &JsValue::from_str("column"))
                .unwrap_or(JsValue::NULL);
            
            let column_field = if column.is_object() {
                Reflect::get(&column, &JsValue::from_str("colId"))
                    .unwrap_or(JsValue::from_str(""))
                    .as_string()
                    .unwrap_or_default()
            } else {
                "".to_string()
            };
            
            callback.emit(GridCellClickEvent {
                node_id: node_id_str,
                data,
                value,
                column: column_field,
            });
        }
    }
}

fn handle_selection_changed(params: JsValue, callback: &RowSelectionCallback) {
    let grid_api = Reflect::get(&params, &JsValue::from_str("api"))
        .unwrap_or(JsValue::NULL);
    
    if grid_api.is_object() {
        let get_selected_rows_fn = Reflect::get(&grid_api, &JsValue::from_str("getSelectedRows"))
            .unwrap_or(JsValue::NULL);
        
        if get_selected_rows_fn.is_function() {
            let selected_rows = Reflect::apply(
                &Function::from(get_selected_rows_fn),
                &grid_api,
                &Array::new(),
            ).unwrap_or(JsValue::NULL);
            
            if let Ok(rows_array) = selected_rows.dyn_into::<Array>() {
                let mut selected = Vec::with_capacity(rows_array.length() as usize);
                for i in 0..rows_array.length() {
                    selected.push(rows_array.get(i));
                }
                callback.emit(selected);
            }
        }
    }
}

fn handle_pagination_changed(params: JsValue, callback: &PaginationChangedCallback) {
    let grid_api = Reflect::get(&params, &JsValue::from_str("api"))
        .unwrap_or(JsValue::NULL);
    
    if grid_api.is_object() {
        // Get current page
        let get_current_page = Reflect::get(&grid_api, &JsValue::from_str("paginationGetCurrentPage"))
            .unwrap_or(JsValue::NULL);
        
        let current_page = if get_current_page.is_function() {
            Reflect::apply(
                &Function::from(get_current_page),
                &grid_api,
                &Array::new(),
            ).unwrap_or(JsValue::from_f64(0.0)).as_f64().unwrap_or(0.0) as usize + 1 // AG-Grid pages are 0-based
        } else {
            1
        };
        
        // Get page size
        let get_page_size = Reflect::get(&grid_api, &JsValue::from_str("paginationGetPageSize"))
            .unwrap_or(JsValue::NULL);
        
        let page_size = if get_page_size.is_function() {
            Reflect::apply(
                &Function::from(get_page_size),
                &grid_api,
                &Array::new(),
            ).unwrap_or(JsValue::from_f64(10.0)).as_f64().unwrap_or(10.0) as usize
        } else {
            10
        };
        
        // Get row count
        let get_row_count = Reflect::get(&grid_api, &JsValue::from_str("paginationGetRowCount"))
            .unwrap_or(JsValue::NULL);
        
        let row_count = if get_row_count.is_function() {
            Reflect::apply(
                &Function::from(get_row_count),
                &grid_api,
                &Array::new(),
            ).unwrap_or(JsValue::from_f64(0.0)).as_f64().unwrap_or(0.0) as usize
        } else {
            0
        };
        
        // Calculate total pages
        let total_pages = if row_count == 0 {
            0
        } else {
            (row_count as f64 / page_size as f64).ceil() as usize
        };
        
        callback.emit(GridPaginationEvent {
            current_page,
            page_size,
            total_pages,
            row_count,
        });
    }
}

fn handle_grid_ready(params: JsValue, grid_api: &UseStateHandle<Option<JsValue>>, callback: &Option<Callback<JsValue>>) {
    let api = Reflect::get(&params, &JsValue::from_str("api"))
        .unwrap_or(JsValue::NULL);
    
    grid_api.set(Some(api.clone()));
    
    if let Some(on_grid_ready) = callback {
        on_grid_ready.emit(api);
    }
}

/// Add row data to the grid
pub fn add_row(grid_api: &JsValue, row_data: impl Serialize) -> Result<(), JsValue> {
    let js_row = to_value(&row_data)?;
    
    let js_code = r#"
        api.applyTransaction({ add: [rowData] });
    "#;
    
    let function = Function::new_with_args("api, rowData", js_code);
    Reflect::apply(&function, &JsValue::NULL, &Array::of2(grid_api, &js_row))?;
    
    Ok(())
}

/// Update a row in the grid
pub fn update_row(grid_api: &JsValue, row_data: impl Serialize) -> Result<(), JsValue> {
    let js_row = to_value(&row_data)?;
    
    let js_code = r#"
        api.applyTransaction({ update: [rowData] });
    "#;
    
    let function = Function::new_with_args("api, rowData", js_code);
    Reflect::apply(&function, &JsValue::NULL, &Array::of2(grid_api, &js_row))?;
    
    Ok(())
}

/// Remove a row from the grid
pub fn remove_row(grid_api: &JsValue, row_data: impl Serialize) -> Result<(), JsValue> {
    let js_row = to_value(&row_data)?;
    
    let js_code = r#"
        api.applyTransaction({ remove: [rowData] });
    "#;
    
    let function = Function::new_with_args("api, rowData", js_code);
    Reflect::apply(&function, &JsValue::NULL, &Array::of2(grid_api, &js_row))?;
    
    Ok(())
}

/// Get selected rows from the grid
pub fn get_selected_rows<T: for<'de> Deserialize<'de>>(grid_api: &JsValue) -> Result<Vec<T>, JsValue> {
    let js_code = r#"
        return api.getSelectedRows();
    "#;
    
    let function = Function::new_with_args("api", js_code);
    let result = Reflect::apply(&function, &JsValue::NULL, &Array::of1(grid_api))?;
    
    if let Ok(array) = result.dyn_into::<Array>() {
        let mut rows = Vec::with_capacity(array.length() as usize);
        
        for i in 0..array.length() {
            let item = array.get(i);
            if let Ok(row) = from_value::<T>(item) {
                rows.push(row);
            }
        }
        
        Ok(rows)
    } else {
        Ok(Vec::new())
    }
}

/// Set filter model
pub fn set_filter_model(grid_api: &JsValue, filter_model: impl Serialize) -> Result<(), JsValue> {
    let js_filter = to_value(&filter_model)?;
    
    let js_code = r#"
        api.setFilterModel(filterModel);
    "#;
    
    let function = Function::new_with_args("api, filterModel", js_code);
    Reflect::apply(&function, &JsValue::NULL, &Array::of2(grid_api, &js_filter))?;
    
    Ok(())
}

/// Set sort model
pub fn set_sort_model(grid_api: &JsValue, sort_model: impl Serialize) -> Result<(), JsValue> {
    let js_sort = to_value(&sort_model)?;
    
    let js_code = r#"
        api.setSortModel(sortModel);
    "#;
    
    let function = Function::new_with_args("api, sortModel", js_code);
    Reflect::apply(&function, &JsValue::NULL, &Array::of2(grid_api, &js_sort))?;
    
    Ok(())
}

/// Export to CSV
pub fn export_to_csv(grid_api: &JsValue, filename: &str) -> Result<(), JsValue> {
    let js_code = format!(r#"
        const params = {{
            fileName: '{}'
        }};
        api.exportDataAsCsv(params);
    "#, filename);
    
    let function = Function::new_with_args("api", &js_code);
    Reflect::apply(&function, &JsValue::NULL, &Array::of1(grid_api))?;
    
    Ok(())
}

/// Set pagination page
pub fn set_pagination_page(grid_api: &JsValue, page: usize) -> Result<(), JsValue> {
    // Note: AG-Grid pages are 0-based
    let js_code = format!(r#"
        api.paginationGoToPage({});
    "#, page - 1);
    
    let function = Function::new_with_args("api", &js_code);
    Reflect::apply(&function, &JsValue::NULL, &Array::of1(grid_api))?;
    
    Ok(())
}

/// Set pagination page size
pub fn set_pagination_page_size(grid_api: &JsValue, page_size: usize) -> Result<(), JsValue> {
    let js_code = format!(r#"
        api.paginationSetPageSize({});
    "#, page_size);
    
    let function = Function::new_with_args("api", &js_code);
    Reflect::apply(&function, &JsValue::NULL, &Array::of1(grid_api))?;
    
    Ok(())
}