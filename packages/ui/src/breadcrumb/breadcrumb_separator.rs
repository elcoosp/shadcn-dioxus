use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct BreadcrumbSeparatorProps {
    #[props(into, default)]
    pub class: String,
}

#[component]
pub fn BreadcrumbSeparator(props: BreadcrumbSeparatorProps) -> Element {
    rsx! {
        span {
            class: "text-muted-foreground mx-1 {props.class}",
            role: "presentation",
            "aria-hidden": "true",
            "data-slot": "breadcrumb-separator",
            // simple text separator instead of SVG icon
            "/"
        }
    }
}
