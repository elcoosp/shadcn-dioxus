use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CarouselItemProps {
    pub index: usize,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CarouselItem(props: CarouselItemProps) -> Element {
    // Ensure item fills full width/height, with minimum size 0 to prevent overflow
    rsx! {
        div {
            "data-slot": "carousel-item",
            role: "group",
            "aria-roledescription": "slide",
            class: "min-w-0 min-h-0 shrink-0 {props.class}",
            style: "flex: 0 0 100%;",
            ..props.attributes,
            {props.children}
        }
    }
}
