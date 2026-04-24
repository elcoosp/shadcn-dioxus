use crate::cn;
use dioxus::prelude::*;
use lucide_dioxus::ChevronRight;

#[derive(Clone, PartialEq, Props)]
pub struct BreadcrumbSeparatorProps {
    #[props(into, default)]
    pub class: String,
}

#[component]
pub fn BreadcrumbSeparator(props: BreadcrumbSeparatorProps) -> Element {
    rsx! {
        span {
            "data-slot": "breadcrumb-separator",
            role: "presentation",
            "aria-hidden": "true",
            class: cn("[&>svg]:size-3.5", &props.class),
            ChevronRight {}
        }
    }
}
