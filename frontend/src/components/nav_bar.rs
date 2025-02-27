use crate::components::theme_toggle::ThemeToggle;
use yew::prelude::*;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
        <nav class="airflow-navbar py-2 shadow-sm">
            <div class="container mx-auto px-4">
                <div class="flex justify-between items-center">
                    <div class="flex items-center">
                        <a href="/" class="flex items-center text-xl font-bold text-blue-600 mr-10">
                            <img src="/assets/logo.png" alt="Cyclonetix Logo" class="navbar-logo" />
                            {"Cyclonetix"}
                        </a>
                        <div class="hidden md:flex space-x-6">
                            <a href="/" class="text-gray-700 hover:text-blue-600 px-3 py-2 rounded-md font-medium">
                                <i class="fas fa-home mr-1"></i>
                                {"Home"}
                            </a>
                            <a href="/" class="text-blue-600 border-b-2 border-blue-600 px-3 py-2 rounded-md font-medium">
                                <i class="fas fa-project-diagram mr-1"></i>
                                {"DAGs"}
                            </a>
                            <a href="#" class="text-gray-700 hover:text-blue-600 px-3 py-2 rounded-md font-medium">
                                <i class="fas fa-tasks mr-1"></i>
                                {"Tasks"}
                            </a>
                            <a href="#" class="text-gray-700 hover:text-blue-600 px-3 py-2 rounded-md font-medium">
                                <i class="fas fa-users mr-1"></i>
                                {"Users"}
                            </a>
                            <a href="#" class="text-gray-700 hover:text-blue-600 px-3 py-2 rounded-md font-medium">
                                <i class="fas fa-cog mr-1"></i>
                                {"Admin"}
                            </a>
                        </div>
                    </div>
                    <div class="flex items-center space-x-4">
                        <ThemeToggle />
                        <a href="#" class="text-gray-700 hover:text-blue-600">
                            <i class="fas fa-bell text-lg"></i>
                        </a>
                        <a href="#" class="text-gray-700 hover:text-blue-600">
                            <i class="fas fa-question-circle text-lg"></i>
                        </a>
                        <div class="flex items-center">
                            <span class="bg-blue-600 text-white rounded-full h-8 w-8 flex items-center justify-center">
                                <i class="fas fa-user"></i>
                            </span>
                            <span class="ml-2 text-gray-700 hidden md:inline-block">{"Prefs"}</span>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}