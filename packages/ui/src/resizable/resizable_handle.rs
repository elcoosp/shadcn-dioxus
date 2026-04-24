use crate::cn;
use dioxus::prelude::*;

const HANDLE_BASE: &str = "relative flex w-px items-center justify-center bg-border after:absolute after:inset-y-0 after:left-1/2 after:w-1 after:-translate-x-1/2 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring focus-visible:ring-offset-1 data-[orientation=vertical]:h-px data-[orientation=vertical]:w-full data-[orientation=vertical]:after:left-0 data-[orientation=vertical]:after:h-1 data-[orientation=vertical]:after:w-full data-[orientation=vertical]:after:-translate-y-1/2 data-[orientation=vertical]:after:translate-x-0 [&[data-panel-group-direction=vertical]>div]:rotate-90";

#[derive(Clone, PartialEq, Props)]
pub struct ResizableHandleProps {
    #[props(default = "vertical".to_string())]
    pub orientation: String,
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub disabled: bool,
}

#[component]
pub fn ResizableHandle(props: ResizableHandleProps) -> Element {
    let classes = cn(HANDLE_BASE, &props.class);
    let is_vertical = props.orientation == "vertical";

    rsx! {
        div {
            "data-slot": "resizable-handle",
            "data-orientation": "{props.orientation}",
            class: "{classes}",
            tabindex: "0",
            "aria-orientation": if is_vertical { "vertical" } else { "horizontal" },
            // Drag logic would be implemented here with onmousedown/onmousemove
        }
    }
}
