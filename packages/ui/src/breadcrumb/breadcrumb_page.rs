use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct BreadcrumbPageProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn BreadcrumbPage(props: BreadcrumbPageProps) -> Element {
    rsx! {
        span {
            "data-slot": "breadcrumb-page",
            role: "link",
            "aria-current": "page",
            class: cn("font-normal text-foreground", &props.class),
            {props.children}
        }
    }
}
