use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ScrollAreaProps {
    #[props(into, default)]
    pub class: String,
    #[props(default = "5px".to_string())]
    pub scrollbar_width: String,
    #[props(default = "5px".to_string())]
    pub scrollbar_height: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn ScrollArea(props: ScrollAreaProps) -> Element {
    rsx! {
        div {
            "data-slot": "scroll-area",
            class: cn("relative overflow-hidden", &props.class),
            style: "--scroll-area-width: {props.scrollbar_width}; --scroll-area-height: {props.scrollbar_height};",
            ..props.attributes,
            div {
                class: "h-full w-full rounded-[inherit]",
                style: "overflow: scroll;",
                {props.children}
            }
        }
    }
}
