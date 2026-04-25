use crate::command::CommandContext;
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
    let ctx = use_context::<CommandContext>();
    let value = (ctx.value)();
    let selected_id = (ctx.selected_id)();
    // Show only when user has typed something and nothing is highlighted/selected
    if value.is_empty() || selected_id.is_some() {
        return rsx! {};
    }
    rsx! {
        div {
            "data-slot": "command-empty",
            class: "py-6 text-center text-sm {props.class}",
            ..props.attributes,
            {props.children}
        }
    }
}
