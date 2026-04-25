use crate::carousel::CarouselContext;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CarouselContentProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CarouselContent(props: CarouselContentProps) -> Element {
    let ctx = use_context::<CarouselContext>();
    let orientation = ctx.orientation;
    let current_index = ctx.current_index;

    let is_horizontal = orientation == crate::carousel::CarouselOrientation::Horizontal;

    // Compute translate percentage
    let translate = use_memo(move || {
        if is_horizontal {
            format!("translateX(-{}%)", current_index() * 100)
        } else {
            format!("translateY(-{}%)", current_index() * 100)
        }
    });

    let flex_class = if is_horizontal { "flex flex-row" } else { "flex flex-col" };

    rsx! {
        div {
            "data-slot": "carousel-content",
            class: "overflow-hidden {props.class}",
            ..props.attributes,
            div {
                class: "{flex_class} transition-transform duration-500",
                style: "transform: {translate};",
                {props.children}
            }
        }
    }
}
