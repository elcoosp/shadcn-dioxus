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
            class: "font-normal text-foreground {props.class}",
            role: "link",
            "aria-current": "page",
            "data-slot": "breadcrumb-page",
            {props.children}
        }
    }
}
