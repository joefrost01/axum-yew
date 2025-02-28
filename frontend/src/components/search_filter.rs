use crate::models::dag::DAGsQuery;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq, Clone)]
pub struct SearchFilterProps {
    pub query: DAGsQuery,
    pub on_search: Callback<DAGsQuery>,
    #[prop_or_default]
    pub on_rows_change: Option<Callback<usize>>,
    #[prop_or(Some(10))]
    pub current_limit: Option<usize>,
}

#[function_component(SearchFilter)]
pub fn search_filter(props: &SearchFilterProps) -> Html {
    let query = props.query.clone();
    let on_search = props.on_search.clone();

    let search_value = use_state(|| query.search.clone().unwrap_or_default());
    let status_value = use_state(|| query.status.clone().unwrap_or_default());
    let tags_value = use_state(|| query.tags.clone().unwrap_or_default());

    // Initialize rows_limit from local storage if available, otherwise use props.current_limit.
    let rows_limit = {
        let default = props.current_limit.unwrap_or(10);
        use_state(move || {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(value)) = storage.get_item("table_rows_limit") {
                        if let Ok(parsed) = value.parse::<usize>() {
                            return parsed;
                        }
                    }
                }
            }
            default
        })
    };

    // Notify the parent about the stored value when the component mounts.
    {
        let rows_limit = rows_limit.clone();
        let on_rows_change = props.on_rows_change.clone();
        use_effect_with((), move |_| {
            if let Some(callback) = on_rows_change {
                callback.emit(*rows_limit);
            }
            || ()
        });
    }

    let on_search_input = {
        let search_value = search_value.clone();
        let on_search = on_search.clone();
        let query = query.clone();

        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap();
            let input = target.dyn_into::<HtmlInputElement>().unwrap();
            let value = input.value();
            search_value.set(value.clone());

            let mut new_query = query.clone();
            if value.is_empty() {
                new_query.search = None;
            } else {
                new_query.search = Some(value);
            }

            on_search.emit(new_query);
        })
    };

    let on_status_change = {
        let status_value = status_value.clone();
        let on_search = on_search.clone();
        let query = query.clone();

        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            let select = target.dyn_into::<web_sys::HtmlSelectElement>().unwrap();
            let value = select.value();
            status_value.set(value.clone());

            let mut new_query = query.clone();
            if value == "all" {
                new_query.status = None;
            } else {
                new_query.status = Some(value);
            }

            on_search.emit(new_query);
        })
    };

    let on_tags_input = {
        let tags_value = tags_value.clone();
        let on_search = on_search.clone();
        let query = query.clone();

        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap();
            let input = target.dyn_into::<HtmlInputElement>().unwrap();
            let value = input.value();
            tags_value.set(value.clone());

            let mut new_query = query.clone();
            if value.is_empty() {
                new_query.tags = None;
            } else {
                new_query.tags = Some(value);
            }

            on_search.emit(new_query);
        })
    };

    html! {
        <div class="bg-white dark:bg-gray-800 shadow rounded-lg p-4 mb-4">
            <div class="grid grid-cols-1 md:grid-cols-12 gap-4">
                <div class="md:col-span-5">
                    <label for="search" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                        {"Search DAGs"}
                    </label>
                    <div class="relative rounded-md shadow-sm">
                        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                            <i class="fas fa-search text-gray-400"></i>
                        </div>
                        <input
                            type="text"
                            id="search"
                            class="focus:ring-blue-500 focus:border-blue-500 block w-full pl-10 pr-12 py-2 border-2 border-gray-300 dark:border-gray-500 bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-md"
                            placeholder="Search by DAG ID or owner"
                            value={(*search_value).clone()}
                            oninput={on_search_input}
                        />
                    </div>
                </div>

                <div class="md:col-span-2">
                    <label for="status" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                        {"Status"}
                    </label>
                    <select
                        id="status"
                        class="focus:ring-blue-500 focus:border-blue-500 block w-full py-2 pl-3 pr-10 border-2 border-gray-300 dark:border-gray-500 bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-md"
                        onchange={on_status_change}
                        value={(*status_value).clone()}
                    >
                        <option value="all">{"All"}</option>
                        <option value="active">{"Active"}</option>
                        <option value="paused">{"Paused"}</option>
                        <option value="success">{"Success"}</option>
                        <option value="failed">{"Failed"}</option>
                        <option value="running">{"Running"}</option>
                    </select>
                </div>

                <div class="md:col-span-4">
                    <label for="tags" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                        {"Tags (comma separated)"}
                    </label>
                    <div class="relative rounded-md shadow-sm">
                        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                            <i class="fas fa-tags text-gray-400"></i>
                        </div>
                        <input
                            type="text"
                            id="tags"
                            class="focus:ring-blue-500 focus:border-blue-500 block w-full pl-10 py-2 border-2 border-gray-300 dark:border-gray-500 bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-md"
                            placeholder="e.g. production,etl"
                            value={(*tags_value).clone()}
                            oninput={on_tags_input}
                        />
                    </div>
                </div>

                {
                    if let Some(on_rows_change) = &props.on_rows_change {
                        let current = *rows_limit;
                        html! {
                            <div class="md:col-span-1">
                                <label for="rows-select" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                    {"Rows"}
                                </label>
                                <select
                                    id="rows-select"
                                    class="focus:ring-blue-500 focus:border-blue-500 block w-full py-2 pl-3 pr-4 border-2 border-gray-300 dark:border-gray-500 bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-md"
                                    onchange={
                                        let on_rows_change = on_rows_change.clone();
                                        let rows_limit = rows_limit.clone();
                                        Callback::from(move |e: yew::events::Event| {
                                            let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
                                            let value = select.value();
                                            let limit = if value == "All" {
                                                1000 // A large number representing "All"
                                            } else {
                                                value.parse::<usize>().unwrap_or(10)
                                            };

                                            // Update the local state.
                                            rows_limit.set(limit);

                                            // Store the chosen limit in local storage.
                                            if let Some(window) = web_sys::window() {
                                                if let Ok(Some(storage)) = window.local_storage() {
                                                    let _ = storage.set_item("table_rows_limit", &limit.to_string());
                                                }
                                            }
                                            on_rows_change.emit(limit);
                                        })
                                    }
                                >
                                    <option value="10" selected={current == 10}>{"10"}</option>
                                    <option value="15" selected={current == 15}>{"15"}</option>
                                    <option value="20" selected={current == 20}>{"20"}</option>
                                    <option value="25" selected={current == 25}>{"25"}</option>
                                    <option value="50" selected={current == 50}>{"50"}</option>
                                    <option value="100" selected={current == 100}>{"100"}</option>
                                    <option value="1000" selected={current >= 1000}>{"All"}</option>
                                </select>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        </div>
    }
}
