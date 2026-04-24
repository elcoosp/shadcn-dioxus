use crate::cn;
use crate::collapsible::CollapsibleContext;
use dioxus::prelude::*;

const CONTENT_BASE: &str = "data-[state=closed]:animate-collapsible-up data-[state=open]:animate-collapsible-down overflow-hidden transition-all";

#[derive(Clone, PartialEq, Props)]
pub struct CollapsibleContentProps {
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CollapsibleContent(props: CollapsibleContentProps) -> Element {
    let ctx = use_context::<CollapsibleContext>();
    let open = ctx.open;

    let data_state = if open() { "open" } else { "closed" };
    let classes = cn(CONTENT_BASE, &props.class);

    rsx! {
        div {
            "data-slot": "collapsible-content",
            "data-state": data_state,
            class: "{classes}",
            ..props.attributes,
            {props.children}
        }
    }
}
