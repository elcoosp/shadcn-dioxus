use crate::docs::component_exists;
use dioxus::prelude::*;

#[component]
pub fn ComponentDoc(name: String) -> Element {
    // Temporarily replace all doc rendering with a simple string to isolate the crash
    rsx! {
        div {
            class: "container mx-auto flex-1 py-12 text-center",
            h1 { class: "text-4xl font-bold mb-4", "Component Doc" }
            p { class: "text-muted-foreground mb-6",
                "This is a temporary placeholder. The crash is not in the breadcrumb component."
            }
        }
    }
}
