use crate::components::search_filter::SearchFilter;
use crate::models::dag::{DAG, DAGsQuery, DAGsResponse};
use crate::utils::aggrid::{ColumnDef, GridPaginationEvent, SortDirection};
use crate::utils::aggrid::AgGrid;
use crate::utils::api::{fetch_dags, toggle_dag_paused};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use js_sys::Object;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[function_component(DagList)]
pub fn dag_list() -> Html {
    let query = use_state(|| DAGsQuery {
        page: Some(1),
        limit: Some(10),
        sort_by: Some("dag_id".to_string()),
        sort_order: Some("asc".to_string()),
        ..Default::default()
    });

    let dags_response = use_state(|| DAGsResponse {
        dags: vec![],
        total_count: 0,
    });

    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let grid_api = use_state(|| None::<JsValue>);

    // Async fetch for DAGs
    let fetch_data = {
        let query = query.clone();
        let dags_response = dags_response.clone();
        let loading = loading.clone();
        let error = error.clone();
        use_async(async move {
            loading.set(true);
            error.set(None);

            match fetch_dags(&query).await {
                Ok(response) => {
                    dags_response.set(response);
                    loading.set(false);
                    Ok(())
                }
                Err(err) => {
                    error.set(Some(err.clone()));
                    loading.set(false);
                    Err(err)
                }
            }
        })
    };

    // Initial data fetch
    {
        let fetch_data = fetch_data.clone();
        use_effect_with((), move |_| {
            fetch_data.run();
            || {}
        });
    }

    // Refetch data when the query changes
    {
        let fetch_data = fetch_data.clone();
        let query_deps = (*query).clone();
        use_effect_with(query_deps, move |_| {
            fetch_data.run();
            || {}
        });
    }

    let on_search = {
        let query = query.clone();
        Callback::from(move |new_query: DAGsQuery| {
            query.set(new_query);
        })
    };

    // Handler for changing the number of rows per page
    let on_limit_change = {
        let query = query.clone();
        Callback::from(move |limit: usize| {
            let mut new_query = (*query).clone();
            new_query.limit = Some(limit);
            new_query.page = Some(1); // Reset to page 1 when limit changes
            query.set(new_query);
        })
    };

    let on_toggle_paused = {
        let dags_response = dags_response.clone();
        let fetch_data = fetch_data.clone();
        Callback::from(move |(dag_id, paused): (String, bool)| {
            let _ = toggle_dag_paused(&dag_id, paused);
            let mut response = (*dags_response).clone();
            response.dags.iter_mut().for_each(|dag| {
                if dag.dag_id == dag_id {
                    dag.paused = paused;
                }
            });
            dags_response.set(response);
            let fetch_data_clone = fetch_data.clone();
            let handle = gloo_timers::callback::Timeout::new(500, move || {
                fetch_data_clone.run();
            });
            handle.forget();
        })
    };

    // Unused sort handler for future use
    let _on_sort = {
        let query = query.clone();
        Callback::from(move |(field, direction): (String, SortDirection)| {
            let sort_order = match direction {
                SortDirection::Ascending => "asc",
                SortDirection::Descending => "desc",
            };
            let mut new_query = (*query).clone();
            new_query.sort_by = Some(field);
            new_query.sort_order = Some(sort_order.to_string());
            query.set(new_query);
        })
    };

    let on_pagination_changed = {
        let query = query.clone();
        Callback::from(move |evt: GridPaginationEvent| {
            let mut new_query = (*query).clone();
            if new_query.page != Some(evt.current_page) {
                new_query.page = Some(evt.current_page);
                query.set(new_query);
            }
        })
    };

    let on_grid_ready = {
        let grid_api = grid_api.clone();
        Callback::from(move |api: JsValue| {
            grid_api.set(Some(api));
        })
    };

    let on_cell_clicked = {
        Callback::from(move |evt: crate::utils::aggrid::GridCellClickEvent| {
            if evt.column == "dag_id" {
                if let Ok(data) = serde_wasm_bindgen::from_value::<DAG>(evt.data) {
                    log::info!("Navigating to DAG: {}", data.dag_id);
                    // Navigation logic would go here
                }
            }
        })
    };

    // Define AG-Grid column definitions
    let column_defs = vec![
        ColumnDef::new("dag_id", "DAG ID")
            .with_width(200)
            .sortable(true)
            .filter(true)
            .with_cell_class("text-blue-600 dark:text-blue-400 underline cursor-pointer")
            .with_extra_prop("cellRenderer", r#"
                function(params) {
                    return `<div class="dag-id">
                        <div class="flex items-center">
                            <span class="status-circle ${params.data.paused ? 'status-paused' : (params.data.running_count > 0 ? 'status-running' : (params.data.failed_count > 0 ? 'status-failed' : (params.data.success_count > 0 ? 'status-success' : '')))} mr-2"></span>
                            <a href="/dag/${params.data.dag_id}/graph" class="hover:text-blue-800 dark:hover:text-blue-300">
                                ${params.data.dag_id}
                            </a>
                        </div>
                        <div class="text-sm text-gray-500 dark:text-gray-400">
                            ${params.data.description || 'No description'}
                        </div>
                    </div>`;
                }
            "#.to_string()),
        ColumnDef::new("owner", "Owner")
            .with_width(100)
            .sortable(true)
            .filter(true),
        ColumnDef::new("tags", "Tags")
            .with_width(150)
            .filter(true)
            .with_extra_prop("cellRenderer", r#"
                function(params) {
                    if (!params.value || params.value.length === 0) return '';
                    return `<div class="flex flex-wrap">
                        ${params.value.map(tag => `<span class="tag mr-1 mb-1">${tag}</span>`).join('')}
                    </div>`;
                }
            "#.to_string()),
        ColumnDef::new("schedule_interval", "Schedule")
            .with_width(120)
            .sortable(true)
            .filter(true),
        ColumnDef::new("last_run", "Last Run")
            .with_width(150)
            .sortable(true)
            .with_extra_prop("cellRenderer", r#"
                function(params) {
                    if (!params.value) return 'Never';
                    const date = new Date(params.value);
                    return date.toLocaleString();
                }
            "#.to_string()),
        ColumnDef::new("runs_count", "Runs")
            .with_width(120)
            .sortable(true)
            .with_extra_prop("cellRenderer", r#"
                function(params) {
                    return `<div class="text-sm">
                        <span class="mr-1">${params.data.runs_count}</span>
                        <span class="text-green-600 mr-1">✓${params.data.success_count}</span>
                        <span class="text-red-600 mr-1">✗${params.data.failed_count}</span>
                        ${params.data.running_count > 0 ? `<span class="text-blue-600">⟳${params.data.running_count}</span>` : ''}
                    </div>`;
                }
            "#.to_string()),
        ColumnDef::new("actions", "Actions")
            .with_width(100)
            .with_extra_prop("cellRenderer", r#"
                function(params) {
                    const toggleText = params.data.paused ? 'Unpause' : 'Pause';
                    const toggleIcon = params.data.paused ? 'fa-play' : 'fa-pause';
                    return `<div class="flex space-x-2">
                        <a href="/dag/${params.data.dag_id}/graph" class="text-gray-500 hover:text-blue-600 dark:text-gray-400 dark:hover:text-blue-400" title="View DAG Graph">
                            <i class="fas fa-project-diagram"></i>
                        </a>
                        <button
                            class="text-gray-500 hover:text-blue-600 dark:text-gray-400 dark:hover:text-blue-400"
                            onclick="window.togglePaused('${params.data.dag_id}', ${!params.data.paused})"
                            title="${toggleText}"
                        >
                            <i class="fas ${toggleIcon}"></i>
                        </button>
                        <button
                            class="text-gray-500 hover:text-blue-600 dark:text-gray-400 dark:hover:text-blue-400"
                            title="Trigger DAG"
                        >
                            <i class="fas fa-play"></i>
                        </button>
                    </div>`;
                }
            "#.to_string()),
    ];

    // Set up a global function for toggling pause status in the grid
    {
        let on_toggle_paused_clone = on_toggle_paused.clone();
        use_effect_with((), move |_| {
            let window = web_sys::window().expect("no global window exists");
            let on_toggle_paused = on_toggle_paused_clone.clone();
            let closure = Closure::wrap(Box::new(move |dag_id: String, paused: bool| {
                on_toggle_paused.emit((dag_id, paused));
            }) as Box<dyn FnMut(String, bool)>);
            js_sys::Reflect::set(
                &window,
                &JsValue::from_str("togglePaused"),
                closure.as_ref().unchecked_ref(),
            ).expect("Failed to set togglePaused function");
            closure.forget();
            move || {}
        });
    }

    // Convert the fetched DAGs to JsValue for the grid
    let row_data = dags_response.dags.iter()
        .map(|dag| {
            let val = to_value(dag).unwrap_or(JsValue::NULL);
            web_sys::console::log_1(&format!("Row data: {:?}", dag).into());
            val
        })
        .collect::<Vec<JsValue>>();

    web_sys::console::log_1(&format!("Total rows: {}", row_data.len()).into());

    // Effect to update the grid when row data or grid API changes
    {
        let grid_api_dep = grid_api.clone();
        let row_data_dep = row_data.clone();
        use_effect_with((grid_api_dep, row_data_dep), move |(grid_api_value, row_data_value)| {
            if let Some(api) = grid_api_value.as_ref() {
                let js_array = js_sys::Array::new();
                for item in row_data_value.iter() {
                    js_array.push(item);
                }
                let update_grid_js = r#"
                    try {
                        api.setRowData(newData);
                        console.log("Grid updated with " + newData.length + " rows");
                    } catch (e) {
                        console.error("Error updating grid:", e);
                    }
                "#;
                let update_fn = js_sys::Function::new_with_args("api, newData", update_grid_js);
                let _ = js_sys::Reflect::apply(&update_fn, &JsValue::NULL, &js_sys::Array::of2(api, &js_array));
            }
            || ()
        });
    }

    let query_for_props = (*query).clone();

    // Custom grid options
    let mut custom_options = std::collections::HashMap::new();
    custom_options.insert("rowHeight".to_string(), JsValue::from_f64(65.0));
    custom_options.insert("headerHeight".to_string(), JsValue::from_f64(40.0));
    custom_options.insert("animateRows".to_string(), JsValue::from_bool(true));
    custom_options.insert("domLayout".to_string(), JsValue::from_str("autoHeight"));
    custom_options.insert("defaultColDef".to_string(), {
        let default_col = Object::new();
        js_sys::Reflect::set(&default_col, &JsValue::from_str("resizable"), &JsValue::from_bool(true)).unwrap();
        js_sys::Reflect::set(&default_col, &JsValue::from_str("sortable"), &JsValue::from_bool(true)).unwrap();
        js_sys::Reflect::set(&default_col, &JsValue::from_str("filter"), &JsValue::from_bool(true)).unwrap();
        default_col.into()
    });

    html! {
        <div>
            <SearchFilter
                query={query_for_props}
                on_search={on_search}
                on_rows_change={Some(on_limit_change)}
                current_limit={query.limit}
            />

            {
                if let Some(error_message) = &*error {
                    html! {
                        <div class="bg-red-50 dark:bg-red-900 border border-red-200 dark:border-red-800 text-red-700 dark:text-red-200 px-4 py-3 rounded">
                            <p>{format!("Error loading DAGs: {}", error_message)}</p>
                        </div>
                    }
                } else {
                    html! {}
                }
            }

            <div class="mb-4 bg-white dark:bg-gray-900 rounded-lg shadow dark:shadow-gray-800" style="height: 650px; width: 100%;">
                <AgGrid
                    id="dags-table"
                    class="w-full h-full"
                    row_data={row_data}
                    column_defs={column_defs}
                    height="600px"
                    pagination={true}
                    page_size={query.limit}
                    row_selection={false}
                    dark_mode={true}
                    custom_options={Some(custom_options)}
                    on_cell_clicked={Some(on_cell_clicked)}
                    on_pagination_changed={Some(on_pagination_changed)}
                    on_grid_ready={Some(on_grid_ready)}
                />
            </div>
        </div>
    }
}
