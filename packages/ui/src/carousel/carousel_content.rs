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

    let overflow_class = match orientation {
        crate::carousel::CarouselOrientation::Horizontal => "overflow-x",
        crate::carousel::CarouselOrientation::Vertical => "overflow-y",
    };

    rsx! {
        div {
            "data-slot": "carousel-content",
            class: "flex {overflow_class}-hidden {props.class}",
            ..props.attributes,
            {props.children}
        }
    }
}
