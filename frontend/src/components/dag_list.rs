use crate::components::search_filter::SearchFilter;
use crate::components::table::{SortDirection, Table, TableHead, TableBody};
use crate::models::dag::{DAG, DAGsQuery, DAGsResponse};
use crate::utils::api::{fetch_dags, format_datetime, toggle_dag_paused};
use web_sys::MouseEvent;
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

    // Data fetch when query changes
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
            new_query.page = Some(1); // Reset to page 1 when changing limit
            query.set(new_query);
        })
    };

    let on_toggle_paused = {
        let dags_response = dags_response.clone();

        Callback::from(move |(dag_id, paused): (String, bool)| {
            // In a real app, we'd call the API here
            let _ = toggle_dag_paused(&dag_id, paused);

            // Update the local state for immediate feedback
            let mut response = (*dags_response).clone();
            response.dags.iter_mut().for_each(|dag| {
                if dag.dag_id == dag_id {
                    dag.paused = paused;
                }
            });
            dags_response.set(response);
        })
    };

    let on_sort = {
        let query = query.clone();

        Callback::from(move |(key, direction): (String, SortDirection)| {
            let sort_order = match direction {
                SortDirection::Ascending => "asc",
                SortDirection::Descending => "desc",
            };

            let mut new_query = (*query).clone();
            new_query.sort_by = Some(key);
            new_query.sort_order = Some(sort_order.to_string());
            query.set(new_query);
        })
    };

    let on_dag_click = {
        Callback::from(move |dag: DAG| {
            // Navigation would go here in a real app
            log::info!("Clicked on DAG: {}", dag.dag_id);
        })
    };

    let on_page_change = {
        let query = query.clone();

        Callback::from(move |page: usize| {
            let mut new_query = (*query).clone();
            new_query.page = Some(page);
            query.set(new_query);
        })
    };

    // Get data for rendering outside of the closure
    let current_page = query.page.unwrap_or(1);
    let query_limit = query.limit.unwrap_or(10);
    let total_count = dags_response.total_count;
    let dags = dags_response.dags.clone();

    // Calculate pagination
    let total_pages = (total_count as f64 / query_limit as f64).ceil() as usize;
    let has_previous = current_page > 1;
    let has_next = current_page < total_pages;

    // Determine current sort direction for table props
    let (sort_key, sort_direction) = {
        let sort_by = query.sort_by.clone().unwrap_or_else(|| "dag_id".to_string());
        let sort_order = query.sort_order.clone().unwrap_or_else(|| "asc".to_string());

        let direction = if sort_order == "asc" {
            Some(SortDirection::Ascending)
        } else {
            Some(SortDirection::Descending)
        };

        (Some(sort_by), direction)
    };

    // Helper callback to handle sorting for a given column.
    let handle_sort = {
        let sort_key = sort_key.clone();
        let sort_direction = sort_direction.clone();
        let on_sort = on_sort.clone();
        Callback::from(move |column: String| {
            let new_direction = if sort_key.as_ref().map(|k| k == &column).unwrap_or(false) {
                if sort_direction == Some(SortDirection::Ascending) {
                    SortDirection::Descending
                } else {
                    SortDirection::Ascending
                }
            } else {
                SortDirection::Ascending
            };
            on_sort.emit((column, new_direction));
        })
    };

    let render_sort_icon = |column: &str| -> Html {
        if sort_key.as_ref().map(|k| k == column).unwrap_or(false) {
            if sort_direction == Some(SortDirection::Ascending) {
                html! { <i class="fas fa-sort-up ml-1"></i> }
            } else {
                html! { <i class="fas fa-sort-down ml-1"></i> }
            }
        } else {
            html! { <i class="fas fa-sort ml-1"></i> }
        }
    };

    let render_pagination = {
        let on_page_change = on_page_change.clone();
        let current_page = current_page;
        let total_pages = total_pages;
        let has_previous = has_previous;
        let has_next = has_next;
        let query_limit = query_limit;
        let total_count = total_count;

        move || {
            if total_pages <= 1 {
                return html! {};
            }

            let page_items = (1..=total_pages)
                .map(|page| {
                    let on_click = {
                        let on_page_change = on_page_change.clone();

                        Callback::from(move |e: MouseEvent| {
                            e.prevent_default();
                            on_page_change.emit(page);
                        })
                    };

                    let is_current = page == current_page;
                    let class = if is_current {
                        "z-10 bg-blue-50 dark:bg-blue-900 border-blue-500 dark:border-blue-500 text-blue-600 dark:text-blue-300 relative inline-flex items-center px-4 py-2 border text-sm font-medium"
                    } else {
                        "bg-white dark:bg-gray-800 border-gray-300 dark:border-gray-600 text-gray-500 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-700 relative inline-flex items-center px-4 py-2 border text-sm font-medium"
                    };

                    html! {
                        <a href="#" {class} onclick={on_click}>{page}</a>
                    }
                })
                .collect::<Html>();

            let prev_onclick = {
                let on_page_change = on_page_change.clone();

                Callback::from(move |e: MouseEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    if has_previous {
                        on_page_change.emit(current_page - 1);
                    }
                })
            };

            let next_onclick = {
                let on_page_change = on_page_change.clone();

                Callback::from(move |e: MouseEvent| {
                    e.prevent_default();
                    e.stop_propagation();
                    if has_next {
                        on_page_change.emit(current_page + 1);
                    }
                })
            };

            html! {
                <div class="bg-white dark:bg-gray-900 px-4 py-3 flex items-center justify-between border-t border-gray-200 dark:border-gray-700 sm:px-6">
                    <div class="flex-1 flex justify-between sm:hidden">
                        <a
                            href="#"
                            class={format!("relative inline-flex items-center px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 {}", if !has_previous { "opacity-50 cursor-not-allowed" } else { "" })}
                            onclick={prev_onclick.clone()}
                        >
                            {"Previous"}
                        </a>
                        <a
                            href="#"
                            class={format!("ml-3 relative inline-flex items-center px-4 py-2 border border-gray-300 dark:border-gray-600 text-sm font-medium rounded-md text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 {}", if !has_next { "opacity-50 cursor-not-allowed" } else { "" })}
                            onclick={next_onclick.clone()}
                        >
                            {"Next"}
                        </a>
                    </div>
                    <div class="hidden sm:flex-1 sm:flex sm:items-center sm:justify-between">
                        <div class="flex items-center space-x-4">
                            <p class="text-sm text-gray-700 dark:text-gray-300">
                                {"Showing "}
                                <span class="font-medium">{((current_page - 1) * query_limit) + 1}</span>
                                {" to "}
                                <span class="font-medium">{std::cmp::min(current_page * query_limit, total_count)}</span>
                                {" of "}
                                <span class="font-medium">{total_count}</span>
                                {" results"}
                            </p>

                        </div>
                        <div>
                            <nav class="relative z-0 inline-flex rounded-md shadow-sm -space-x-px" aria-label="Pagination">
                                <a
                                    href="#"
                                    class={format!("relative inline-flex items-center px-2 py-2 rounded-l-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-sm font-medium text-gray-500 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-700 {}", if !has_previous { "opacity-50 cursor-not-allowed" } else { "" })}
                                    onclick={prev_onclick}
                                >
                                    <span class="sr-only">{"Previous"}</span>
                                    <i class="fas fa-chevron-left"></i>
                                </a>
                                {page_items}
                                <a
                                    href="#"
                                    class={format!("relative inline-flex items-center px-2 py-2 rounded-r-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-sm font-medium text-gray-500 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-700 {}", if !has_next { "opacity-50 cursor-not-allowed" } else { "" })}
                                    onclick={next_onclick}
                                >
                                    <span class="sr-only">{"Next"}</span>
                                    <i class="fas fa-chevron-right"></i>
                                </a>
                            </nav>
                        </div>
                    </div>
                </div>
            }
        }
    };

    // Clone for use in the html macro
    let query_for_props = (*query).clone();

    html! {
        <div>
            <SearchFilter
                query={query_for_props}
                on_search={on_search}
                on_rows_change={Some(on_limit_change)}
                current_limit={query_limit}
            />

            {
                if let Some(error_message) = &*error {
                    html! {
                        <div class="bg-red-50 dark:bg-red-900 border border-red-200 dark:border-red-800 text-red-700 dark:text-red-200 px-4 py-3 rounded">
                            <p>{format!("Error loading DAGs: {}", error_message)}</p>
                        </div>
                    }
                } else {
                    html! {
                        <div class="mb-4 bg-white dark:bg-gray-900 rounded-lg shadow dark:shadow-gray-800">
                            <Table>
                                <TableHead>
                                    <tr>
                                        // DAG ID (sortable)
                                        <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider w-1/4">
                                            <div
                                                onclick={
                                                    let handle_sort = handle_sort.clone();
                                                    Callback::from(move |_| {
                                                        handle_sort.emit("dag_id".to_string());
                                                    })
                                                }
                                                style="cursor: pointer; display: flex; align-items: center;"
                                            >
                                                <span>{"DAG ID"}</span>
                                                { render_sort_icon("dag_id") }
                                            </div>
                                        </th>
                                        // Owner (sortable)
                                        <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider w-1/12">
                                            <div
                                                onclick={
                                                    let handle_sort = handle_sort.clone();
                                                    Callback::from(move |_| {
                                                        handle_sort.emit("owner".to_string());
                                                    })
                                                }
                                                style="cursor: pointer; display: flex; align-items: center;"
                                            >
                                                <span>{"Owner"}</span>
                                                { render_sort_icon("owner") }
                                            </div>
                                        </th>
                                        // Tags (not sortable)
                                        <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider w-1/8">
                                            {"Tags"}
                                        </th>
                                        // Schedule (sortable)
                                        <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider w-1/8">
                                            <div
                                                onclick={
                                                    let handle_sort = handle_sort.clone();
                                                    Callback::from(move |_| {
                                                        handle_sort.emit("schedule_interval".to_string());
                                                    })
                                                }
                                                style="cursor: pointer; display: flex; align-items: center;"
                                            >
                                                <span>{"Schedule"}</span>
                                                { render_sort_icon("schedule_interval") }
                                            </div>
                                        </th>
                                        // Last Run (sortable)
                                        <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider w-1/6">
                                            <div
                                                onclick={
                                                    let handle_sort = handle_sort.clone();
                                                    Callback::from(move |_| {
                                                        handle_sort.emit("last_run".to_string());
                                                    })
                                                }
                                                style="cursor: pointer; display: flex; align-items: center;"
                                            >
                                                <span>{"Last Run"}</span>
                                                { render_sort_icon("last_run") }
                                            </div>
                                        </th>
                                        // Runs (sortable) – assuming sorting by total runs count
                                        <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider w-1/6">
                                            <div
                                                onclick={
                                                    let handle_sort = handle_sort.clone();
                                                    Callback::from(move |_| {
                                                        handle_sort.emit("runs_count".to_string());
                                                    })
                                                }
                                                style="cursor: pointer; display: flex; align-items: center;"
                                            >
                                                <span>{"Runs"}</span>
                                                { render_sort_icon("runs_count") }
                                            </div>
                                        </th>
                                        // Actions (not sortable)
                                        <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider w-1/12">
                                            {"Actions"}
                                        </th>
                                    </tr>
                                </TableHead>
                                <TableBody
                                    loading={*loading && fetch_data.loading}
                                    empty={dags.is_empty()}
                                    no_data_message="No DAGs found matching your criteria."
                                    col_span={7}
                                >
                                    {
                                        dags.iter().enumerate().map(|(index, dag)| {
                                            let is_striped = index % 2 == 1;
                                            let status_class = match dag.status() {
                                                "paused" => "status-paused",
                                                "running" => "status-running",
                                                "failed" => "status-failed",
                                                "success" => "status-success",
                                                _ => "",
                                            };

                                            let on_toggle_paused_cb = {
                                                let on_toggle_paused = on_toggle_paused.clone();
                                                let dag_id = dag.dag_id.clone();
                                                let currently_paused = dag.paused;

                                                Callback::from(move |e: MouseEvent| {
                                                    e.prevent_default();
                                                    e.stop_propagation();
                                                    on_toggle_paused.emit((dag_id.clone(), !currently_paused));
                                                })
                                            };

                                            let toggle_text = if dag.paused { "Unpause" } else { "Pause" };
                                            let toggle_icon = if dag.paused { "fa-play" } else { "fa-pause" };

                                            let row_class = if is_striped {
                                                "bg-gray-50 dark:bg-gray-800 dark:text-gray-200"
                                            } else {
                                                "bg-white dark:bg-gray-900 dark:text-gray-200"
                                            };

                                            html! {
                                                <tr class={format!("hover:bg-gray-100 dark:hover:bg-gray-700 {}", row_class)}>
                                                    <td class="px-6 py-2">
                                                        <div>
                                                            <div class="font-medium text-gray-900 dark:text-gray-100 flex items-center">
                                                                <span class={format!("status-circle {} mr-2", status_class)}></span>
                                                                <a href={format!("/dag/{}/graph", dag.dag_id)} class="hover:text-blue-600 dark:hover:text-blue-400">
                                                                    {&dag.dag_id}
                                                                </a>
                                                            </div>
                                                            <div class="text-sm text-gray-500 dark:text-gray-400">
                                                                {dag.description.clone().unwrap_or_else(|| "No description".to_string())}
                                                            </div>
                                                        </div>
                                                    </td>
                                                    <td class="px-6 py-2">
                                                        <span>{&dag.owner}</span>
                                                    </td>
                                                    <td class="px-6 py-2">
                                                        <div class="flex flex-wrap">
                                                            {
                                                                dag.tags.iter().map(|tag| {
                                                                    html! {
                                                                        <span class="tag mr-1 mb-1">{tag}</span>
                                                                    }
                                                                }).collect::<Html>()
                                                            }
                                                        </div>
                                                    </td>
                                                    <td class="px-6 py-2">
                                                        <span>{&dag.schedule_interval}</span>
                                                    </td>
                                                    <td class="px-6 py-2">
                                                        <span>{format_datetime(dag.last_run)}</span>
                                                    </td>
                                                    <td class="px-6 py-2">
                                                        <div class="text-sm">
                                                            <span class="mr-1">{format!("{}", dag.runs_count)}</span>
                                                            <span class="text-green-600 mr-1">{format!("✓{}", dag.success_count)}</span>
                                                            <span class="text-red-600 mr-1">{format!("✗{}", dag.failed_count)}</span>
                                                            {
                                                                if dag.running_count > 0 {
                                                                    html! {
                                                                        <span class="text-blue-600">{format!("⟳{}", dag.running_count)}</span>
                                                                    }
                                                                } else {
                                                                    html! {}
                                                                }
                                                            }
                                                        </div>
                                                    </td>
                                                    <td class="px-6 py-2">
                                                        <div class="flex space-x-2">
                                                            <a href={format!("/dag/{}/graph", dag.dag_id)} class="text-gray-500 hover:text-blue-600 dark:text-gray-400 dark:hover:text-blue-400" title="View DAG Graph">
                                                                <i class="fas fa-project-diagram"></i>
                                                            </a>
                                                            <button
                                                                class="text-gray-500 hover:text-blue-600 dark:text-gray-400 dark:hover:text-blue-400"
                                                                onclick={on_toggle_paused_cb}
                                                                title={toggle_text}
                                                            >
                                                                <i class={format!("fas {}", toggle_icon)}></i>
                                                            </button>
                                                            <button
                                                                class="text-gray-500 hover:text-blue-600 dark:text-gray-400 dark:hover:text-blue-400"
                                                                title="Trigger DAG"
                                                            >
                                                                <i class="fas fa-play"></i>
                                                            </button>
                                                        </div>
                                                    </td>
                                                </tr>
                                            }
                                        }).collect::<Html>()
                                    }
                                </TableBody>
                            </Table>
                        </div>
                    }
                }
            }

            { render_pagination() }
        </div>
    }
}
