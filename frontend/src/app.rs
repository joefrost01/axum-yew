use crate::components::dag_graph::DAGGraph;
use crate::components::nav_bar::NavBar;
use crate::components::dag_list::DagList;
use crate::models::theme::{Theme, ThemeContext};
use yew::prelude::*;
use yew_hooks::use_location;
use chrono::Datelike;
use gloo::storage::{LocalStorage, Storage};

const THEME_KEY: &str = "cyclonetix_theme";

#[function_component(App)]
pub fn app() -> Html {
    // Initialize theme from localStorage or default to light
    let theme = use_state(|| {
        LocalStorage::get(THEME_KEY).unwrap_or(Theme::Light)
    });

    // Set up theme context
    let theme_value = *theme;
    let theme_ctx = {
        let theme_clone = theme.clone();
        ThemeContext::new(
            theme_value,
            Callback::from(move |new_theme| {
                LocalStorage::set(THEME_KEY, new_theme).expect("Failed to store theme");
                theme_clone.set(new_theme);
            }),
        )
    };

    // Apply theme to body
    {
        let theme_class = theme.css_class();
        use_effect_with(theme_value, move |_| {
            let window = web_sys::window().expect("No window found");
            let document = window.document().expect("No document found");
            let body = document.body().expect("No body found");

            // Set the class directly on the body element
            // First, get existing classes
            let existing_classes = body.get_attribute("class").unwrap_or_default();

            // Remove theme classes
            let mut classes: Vec<&str> = existing_classes
                .split_whitespace()
                .filter(|c| *c != "light-theme" && *c != "dark-theme")
                .collect();

            // Add the current theme class
            classes.push(theme_class);

            // Update the class attribute
            let _ = body.set_attribute("class", &classes.join(" "));

            || {}
        });
    }

    // Use the location hook to get the current URL
    let location = use_location();
    let pathname = location.pathname.clone();
    
    // Parse the URL to decide which view to show
    let view = {
        if pathname.starts_with("/dag/") && pathname.contains("/graph") {
            let dag_id = pathname.strip_prefix("/dag/").unwrap_or("")
                .strip_suffix("/graph").unwrap_or("");
            if !dag_id.is_empty() {
                // URL-decode the DAG ID
                let decoded_dag_id = match js_sys::decode_uri_component(&dag_id) {
                    Ok(decoded) => decoded,
                    Err(_) => js_sys::JsString::from(dag_id.clone())
                }.as_string().unwrap_or_else(|| dag_id.to_string());
                
                html! {
                    <DAGGraph dag_id={decoded_dag_id} />
                }
            } else {
                html! { <DagList /> }
            }
        } else {
            html! { <DagList /> }
        }
    };

    html! {
        <ContextProvider<ThemeContext> context={theme_ctx}>
            <div class="flex flex-col min-h-screen">
                <NavBar />
                <main class="flex-grow w-full">
                    { view }
                </main>
                <footer class="border-t border-gray-200 py-2">
                    <div class="w-full px-2 text-xs text-gray-500 text-center">
                        {"Cyclonetix © "}{chrono::Utc::now().year()}{" - Powered by Yew and Axum"}
                    </div>
                </footer>
            </div>
        </ContextProvider<ThemeContext>>
    }
}
