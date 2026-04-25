use crate::carousel::CarouselContext;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CarouselIndicatorsProps {
    #[props(into, default)]
    pub class: String,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CarouselIndicators(props: CarouselIndicatorsProps) -> Element {
    let ctx = use_context::<CarouselContext>();
    let current_index = ctx.current_index;
    let total = ctx.total;
    let set_index = ctx.set_index;

    let position_class = match ctx.orientation {
        crate::carousel::CarouselOrientation::Horizontal => "bottom-2 left-1/2 -translate-x-1/2 flex",
        crate::carousel::CarouselOrientation::Vertical => "right-2 top-1/2 -translate-y-1/2 flex-col",
    };

    rsx! {
        div {
            "data-slot": "carousel-indicators",
            class: "absolute z-10 {position_class} gap-1 {props.class}",
            ..props.attributes,
            {
                let t = total();
                let curr = current_index();
                (0..t).map(move |i| {
                    let si = set_index.clone();
                    rsx! {
                        button {
                            key: "{i}",
                            r#type: "button",
                            role: "tab",
                            "aria-selected": i == curr,
                            "aria-label": "Slide {i + 1}",
                            class: if i == curr {
                                "inline-block h-2 w-2 rounded-full bg-primary"
                            } else {
                                "inline-block h-2 w-2 rounded-full bg-primary/30"
                            },
                            onclick: move |_| si.call(i),
                        }
                    }
                })
            }
        }
    }
}
