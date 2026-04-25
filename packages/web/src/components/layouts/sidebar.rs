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
            // Sidebar: full height, sticky at top
            div { class: "hidden md:block shrink-0",
                div { class: "sticky top-0 h-screen overflow-y-auto border-r border-border",
                    Sidebar { active_slug }
                }
            }
            // Main content: flex-1, also full height
            div { class: "flex-1 min-h-screen",
                div { class: "w-full max-w-3xl mx-auto py-6 px-4 md:px-6 lg:py-8",
                    {props.children}
                }
            }
        }
    }
}
