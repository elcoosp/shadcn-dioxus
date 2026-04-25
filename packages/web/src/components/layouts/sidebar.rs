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
            // Sidebar: hidden on mobile, shown on md+
            div { class: "hidden md:block shrink-0 w-64",
                div { class: "sticky top-0 h-screen overflow-y-auto border-r border-border bg-background",
                    Sidebar { active_slug }
                }
            }
            // Main content: full width on mobile, adjusted for sidebar on desktop
            div { class: "flex-1 min-h-screen w-full overflow-x-hidden",
                div { class: "w-full max-w-3xl mx-auto py-4 sm:py-6 px-4 sm:px-6 lg:py-8",
                    {props.children}
                }
            }
        }
    }
}
