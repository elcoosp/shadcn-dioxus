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
    let flex_basis = "0 0 100%"; // Ensure full-width/height slide
    let min_class = "min-w-0 shrink-0";

    rsx! {
        div {
            "data-slot": "carousel-item",
            role: "group",
            "aria-roledescription": "slide",
            class: "{min_class} {props.class}",
            style: "flex: {flex_basis};",
            ..props.attributes,
            {props.children}
        }
    }
}
