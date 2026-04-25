use super::super::Sidebar;
use crate::Route;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SidebarLayoutProps {
    children: Element,
}

#[component]
pub fn SidebarLayout(props: SidebarLayoutProps) -> Element {
    let route = use_route::<Route>();
    let active_slug = match route {
        Route::ComponentDoc { name } => name,
        _ => String::new(),
    };

    rsx! {
        div { class: "flex w-full min-h-screen",
            // Sidebar handles its own mobile/desktop visibility
            Sidebar { active_slug }
            // Main content – add left padding on mobile to not overlap the fixed toggle button
            div { class: "flex-1 min-h-screen w-full overflow-x-hidden pt-12 md:pt-0",
                div { class: "w-full max-w-3xl mx-auto py-4 sm:py-6 px-4 sm:px-6 lg:py-8",
                    {props.children}
                }
            }
        }
    }
}
