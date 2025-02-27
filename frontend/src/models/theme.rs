use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn toggle(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }

    pub fn css_class(&self) -> &'static str {
        match self {
            Theme::Light => "light-theme",
            Theme::Dark => "dark-theme",
        }
    }
}

// Theme context for global access
#[derive(Clone, Debug, PartialEq)]
pub struct ThemeContext {
    pub theme: Theme,
    pub set_theme: Callback<Theme>,
}

impl ThemeContext {
    pub fn new(theme: Theme, set_theme: Callback<Theme>) -> Self {
        Self { theme, set_theme }
    }
}