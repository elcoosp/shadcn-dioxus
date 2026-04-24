use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CommandEmptyProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CommandEmpty(props: CommandEmptyProps) -> Element {
    rsx! {
        div {
            "data-slot": "command-empty",
            class: "py-6 text-center text-sm {props.class}",
            ..props.attributes,
            {props.children}
        }
    }
}
