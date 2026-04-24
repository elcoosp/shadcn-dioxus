use crate::carousel::CarouselContext;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CarouselItemProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CarouselItem(props: CarouselItemProps) -> Element {
    let ctx = use_context::<CarouselContext>();
    let current_index = ctx.current_index;

    // We need to track our own index; use a hook-based counter
    let index = use_hook(|| {
        // This will be set properly by the parent via key
        0usize
    });

    let is_active = use_memo(move || {
        current_index() == index
    });

    let min_class = match ctx.orientation {
        crate::carousel::CarouselOrientation::Horizontal => "min-w-0 shrink-0",
        crate::carousel::CarouselOrientation::Vertical => "min-h-0 shrink-0",
    };

    rsx! {
        div {
            "data-slot": "carousel-item",
            role: "group",
            "aria-roledescription": "slide",
            "data-active": is_active(),
            class: "{min_class} {props.class}",
            ..props.attributes,
            {props.children}
        }
    }
}
