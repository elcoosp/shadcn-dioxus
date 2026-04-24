use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct BreadcrumbItemProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn BreadcrumbItem(props: BreadcrumbItemProps) -> Element {
    rsx! {
        li {
            "data-slot": "breadcrumb-item",
            class: cn("inline-flex items-center gap-1.5", &props.class),
            {props.children}
        }
    }
}
