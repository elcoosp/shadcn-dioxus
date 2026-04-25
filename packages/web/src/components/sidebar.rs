use crate::Route;
use super::{SidebarLinkv2, SidebarNav};
use dioxus::prelude::*;
use ui::Button;

#[component]
pub fn Sidebar(#[props(into, default)] active_slug: String) -> Element {
    let mut mobile_open = use_signal(|| false);

    rsx! {
        // Mobile toggle button
        div { class: "md:hidden fixed top-3 left-3 z-50",
            Button {
                variant: ui::ButtonVariant::Outline,
                size: ui::ButtonSize::IconSm,
                onclick: move |_| mobile_open.set(!mobile_open()),
                span { class: "sr-only", "Toggle sidebar" }
                // Hamburger icon
                svg {
                    class: "h-4 w-4",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke: "currentColor",
                    stroke_width: "2",
                    path { stroke_linecap: "round", stroke_linejoin: "round", d: "M4 6h16M4 12h16M4 18h16" }
                }
            }
        }

        // Desktop sidebar (always visible on md+)
        aside { class: "hidden md:block w-64 shrink-0 border-r border-border",
            div { class: "sticky top-[--header-height] h-[calc(100vh-var(--header-height))] overflow-y-auto py-6 px-4",
                div {
                    class: "pb-4",
                    h4 { class: "text-sm font-semibold", "Sections" }
                }
                div {
                    class: "flex flex-col gap-1",
                    SidebarLinkv2 {
                        to: Route::InstallationView {  }.into(),
                        is_active: active_slug == "installation",
                        "Installation"
                    }
                    SidebarLinkv2 {
                        to: Route::ComponentView {  }.into(),
                        is_active: active_slug == "components",
                        div { "Components" }
                    }
                    SidebarLinkv2 {
                        to: Route::ThemingView {  }.into(),
                        is_active: active_slug == "theming",
                        div { "Theming" }
                    }
                }

                div { class: "py-4",
                    h4 { class: "text-sm font-semibold", "Components" }
                }
                SidebarNav { active_slug }
            }
        }

        // Mobile overlay
        if mobile_open() {
            div {
                class: "fixed inset-0 z-40 bg-black/50 md:hidden",
                onclick: move |_| mobile_open.set(false),
            }
            div {
                class: "fixed inset-y-0 left-0 z-50 w-64 bg-background border-r border-border overflow-y-auto p-4 md:hidden",
                onclick: move |evt| evt.stop_propagation(),
                div { class: "pb-4", h4 { class: "text-sm font-semibold", "Sections" } }
                div {
                    class: "flex flex-col gap-1",
                    SidebarLinkv2 {
                        to: Route::InstallationView {  }.into(),
                        is_active: active_slug == "installation",
                        "Installation"
                    }
                    SidebarLinkv2 {
                        to: Route::ComponentView {  }.into(),
                        is_active: active_slug == "components",
                        div { "Components" }
                    }
                    SidebarLinkv2 {
                        to: Route::ThemingView {  }.into(),
                        is_active: active_slug == "theming",
                        div { "Theming" }
                    }
                }
                div { class: "py-4", h4 { class: "text-sm font-semibold", "Components" } }
                SidebarNav { active_slug large_text: true }
            }
        }
    }
}
