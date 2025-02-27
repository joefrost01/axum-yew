use crate::models::theme::{Theme, ThemeContext};
use yew::prelude::*;

#[function_component(ThemeToggle)]
pub fn theme_toggle() -> Html {
    let theme_ctx = use_context::<ThemeContext>().expect("No theme context found");
    let theme = theme_ctx.theme;

    let onclick = {
        let theme_ctx = theme_ctx.clone();
        Callback::from(move |_| {
            let new_theme = theme_ctx.theme.toggle();
            theme_ctx.set_theme.emit(new_theme);
        })
    };

    let icon_class = match theme {
        Theme::Light => "fas fa-moon",
        Theme::Dark => "fas fa-sun",
    };

    let text = match theme {
        Theme::Light => "Dark Mode",
        Theme::Dark => "Light Mode",
    };

    html! {
        <button class="theme-toggle" {onclick}>
            <i class={icon_class}></i>
            <span class="ml-2 hidden md:inline-block">{text}</span>
        </button>
    }
}