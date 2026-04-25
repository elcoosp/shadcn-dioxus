use crate::cn;
use crate::navigation_menu::NavigationMenuContext;
use dioxus::prelude::*;

const INDICATOR_BASE: &str = "data-[state=visible]:animate-in data-[state=hidden]:animate-out data-[state=hidden]:fade-out data-[state=visible]:fade-in data-[state=hidden]:zoom-out-100 data-[state=visible]:zoom-in-100 top-full z-[1] flex h-1.5 items-end justify-center overflow-hidden data-[state=hidden]:opacity-0";

const ARROW_BASE: &str = "relative top-[60%] size-2 rotate-45 rounded-tl-sm bg-border shadow-md";

#[derive(Clone, PartialEq, Props)]
pub struct NavigationMenuIndicatorProps {
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub force_mount: bool,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn NavigationMenuIndicator(props: NavigationMenuIndicatorProps) -> Element {
    let ctx = use_context::<NavigationMenuContext>();
    let open_item = ctx.open_item;

    let is_visible = open_item().is_some();

    if !props.force_mount && !is_visible {
        return rsx! {};
    }

    let data_state = if is_visible { "visible" } else { "hidden" };

    rsx! {
        div {
            "data-slot": "navigation-menu-indicator",
            "data-state": data_state,
            "aria-hidden": "true",
            class: cn(INDICATOR_BASE, &props.class),
            ..props.attributes,
            div {
                class: ARROW_BASE,
            }
        }
    }
}
