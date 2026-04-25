use super::super::Sidebar;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn SidebarLayout() -> Element {
    let route = use_route::<Route>();
    let route_clone = route.clone();
    let active_slug = match route_clone {
        Route::ComponentDoc { name } => name,
        _ => String::new(),
    };
    let route_key = format!("sidebar-{:?}", route);

    rsx! {
        div { class: "flex flex-1 w-full",
            Sidebar { active_slug }
            div { class: "flex-1",
                div { class: "w-full max-w-2xl mx-auto py-6 px-4 md:px-0 lg:py-8",
                    // Wrap Outlet in a div with a route-dependent key to force remount
                    div { key: "{route_key}",
                        Outlet::<Route> {}
                    }
                }
            }
        }
    }
}
