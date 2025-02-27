use crate::models::dag::DAGsQuery;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq, Clone)]
pub struct SearchFilterProps {
    pub query: DAGsQuery,
    pub on_search: Callback<DAGsQuery>,
}

#[function_component(SearchFilter)]
pub fn search_filter(props: &SearchFilterProps) -> Html {
    let query = props.query.clone();
    let on_search = props.on_search.clone();

    let search_value = use_state(|| query.search.clone().unwrap_or_default());
    let status_value = use_state(|| query.status.clone().unwrap_or_default());
    let tags_value = use_state(|| query.tags.clone().unwrap_or_default());

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
        <div class="bg-white shadow rounded-lg p-4 mb-4">
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div>
                    <label for="search" class="block text-sm font-medium text-gray-700 mb-1">{"Search DAGs"}</label>
                    <div class="relative rounded-md shadow-sm">
                        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                            <i class="fas fa-search text-gray-400"></i>
                        </div>
                        <input
                            type="text"
                            id="search"
                            class="focus:ring-blue-500 focus:border-blue-500 block w-full pl-10 pr-12 py-2 border-gray-300 rounded-md"
                            placeholder="Search by DAG ID or owner"
                            value={(*search_value).clone()}
                            oninput={on_search_input}
                        />
                    </div>
                </div>

                <div>
                    <label for="status" class="block text-sm font-medium text-gray-700 mb-1">{"Status"}</label>
                    <select
                        id="status"
                        class="focus:ring-blue-500 focus:border-blue-500 block w-full py-2 pl-3 pr-10 border-gray-300 rounded-md"
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

                <div>
                    <label for="tags" class="block text-sm font-medium text-gray-700 mb-1">{"Tags (comma separated)"}</label>
                    <div class="relative rounded-md shadow-sm">
                        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                            <i class="fas fa-tags text-gray-400"></i>
                        </div>
                        <input
                            type="text"
                            id="tags"
                            class="focus:ring-blue-500 focus:border-blue-500 block w-full pl-10 py-2 border-gray-300 rounded-md"
                            placeholder="e.g. production,etl"
                            value={(*tags_value).clone()}
                            oninput={on_tags_input}
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}