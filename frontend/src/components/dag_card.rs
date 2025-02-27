use crate::models::dag::DAG;
use crate::utils::api::format_datetime;
use web_sys::MouseEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DagCardProps {
    pub dag: DAG,
    pub on_toggle_paused: Callback<(String, bool)>,
}

#[function_component(DagCard)]
pub fn dag_card(props: &DagCardProps) -> Html {
    let dag = &props.dag;

    let on_toggle_paused = {
        let on_toggle_paused = props.on_toggle_paused.clone();
        let dag_id = dag.dag_id.clone();
        let currently_paused = dag.paused;

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_toggle_paused.emit((dag_id.clone(), !currently_paused));
        })
    };

    let status_class = match dag.status() {
        "paused" => "status-paused",
        "running" => "status-running",
        "failed" => "status-failed",
        "success" => "status-success",
        _ => "",
    };

    let status_text = match dag.status() {
        "paused" => "Paused",
        "running" => "Running",
        "failed" => "Failed",
        "success" => "Success",
        _ => "No Status",
    };

    let toggle_text = if dag.paused {
        "Unpause"
    } else {
        "Pause"
    };

    let toggle_icon = if dag.paused {
        "fa-play"
    } else {
        "fa-pause"
    };

    html! {
        <div class="airflow-card bg-white rounded-lg mb-4">
            <div class="p-4">
                <div class="flex justify-between items-start">
                    <div>
                        <h3 class="text-lg font-semibold text-gray-800 mb-1">
                            <span class={format!("status-circle {}", status_class)}></span>
                            {&dag.dag_id}
                        </h3>
                        <p class="text-sm text-gray-600 mb-2">
                            {dag.description.clone().unwrap_or_else(|| "No description".to_string())}
                        </p>
                        <div class="flex flex-wrap mb-2">
                            {
                                dag.tags.iter().map(|tag| {
                                    html! {
                                        <span class="tag">{tag}</span>
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    </div>
                    <div class="flex space-x-2">
                        <button
                            class="px-3 py-1 bg-gray-200 hover:bg-gray-300 rounded text-sm"
                            onclick={on_toggle_paused}
                        >
                            <i class={format!("fas {} mr-1", toggle_icon)}></i>
                            {toggle_text}
                        </button>
                        <button class="px-3 py-1 bg-gray-200 hover:bg-gray-300 rounded text-sm">
                            <i class="fas fa-play mr-1"></i>
                            {"Trigger"}
                        </button>
                    </div>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mt-4 text-sm">
                    <div>
                        <p class="text-gray-600">{"Owner"}</p>
                        <p class="font-medium">{&dag.owner}</p>
                    </div>
                    <div>
                        <p class="text-gray-600">{"Last Run"}</p>
                        <p class="font-medium">{format_datetime(dag.last_run)}</p>
                    </div>
                    <div>
                        <p class="text-gray-600">{"Next Run"}</p>
                        <p class="font-medium">{format_datetime(dag.next_run)}</p>
                    </div>
                    <div>
                        <p class="text-gray-600">{"Schedule"}</p>
                        <p class="font-medium">{&dag.schedule_interval}</p>
                    </div>
                    <div>
                        <p class="text-gray-600">{"Runs"}</p>
                        <p class="font-medium">
                            <span class="mr-2">{format!("Total: {}", dag.runs_count)}</span>
                            <span class="text-green-600 mr-2">{format!("Success: {}", dag.success_count)}</span>
                            <span class="text-red-600 mr-2">{format!("Failed: {}", dag.failed_count)}</span>
                            {
                                if dag.running_count > 0 {
                                    html! {
                                        <span class="text-blue-600">{format!("Running: {}", dag.running_count)}</span>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                        </p>
                    </div>
                    <div>
                        <p class="text-gray-600">{"Status"}</p>
                        <p class="font-medium">{status_text}</p>
                    </div>
                </div>

                <div class="mt-4 pt-3 border-t border-gray-200 flex justify-end space-x-2">
                    <a href={format!("/dag/{}/graph", dag.dag_id)} class="text-sm text-blue-600 hover:text-blue-800">
                        <i class="fas fa-eye mr-1"></i>
                        {"View Graph"}
                    </a>
                    <a href="#" class="text-sm text-blue-600 hover:text-blue-800">
                        <i class="fas fa-code mr-1"></i>
                        {"View Code"}
                    </a>
                    <a href="#" class="text-sm text-blue-600 hover:text-blue-800">
                        <i class="fas fa-history mr-1"></i>
                        {"View History"}
                    </a>
                </div>
            </div>
        </div>
    }
}