use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct ResizablePanelProps {
    #[props(default = 1.0)]
    pub default_size: f64,
    #[props(default = false)]
    pub collapsible: bool,
    #[props(default = 0.0)]
    pub min_size: f64,
    #[props(default = 100.0)]
    pub max_size: f64,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn ResizablePanel(props: ResizablePanelProps) -> Element {
    let style = format!(
        "flex-basis: {}%; min-width: 0; min-height: 0; overflow: hidden;",
        props.default_size * 100.0
    );

    rsx! {
        div {
            "data-slot": "resizable-panel",
            style: "{style}",
            class: "{props.class}",
            {props.children}
        }
    }
}
