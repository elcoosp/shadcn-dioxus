use crate::cn;
use crate::navigation_menu::NavigationMenuContext;
use dioxus::prelude::*;
use super::navigation_menu_item::NavigationMenuItemContext;

const CONTENT_BASE: &str = "left-0 top-full flex justify-center data-[motion^=from-]:animate-in data-[motion^=to-]:animate-out data-[motion^=from-]:fade-in data-[motion^=to-]:fade-out data-[motion=from-end]:slide-in-from-right-52 data-[motion=from-start]:slide-in-from-left-52 data-[motion=to-end]:slide-out-to-right-52 data-[motion=to-start]:slide-out-to-left-52 md:absolute md:w-auto";

const INNER_BASE: &str = "bg-popover text-popover-foreground data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-90 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 relative z-50 grid w-64 origin-(--radix-navigation-menu-content-transform-origin) gap-1 rounded-md border p-2 shadow-lg md:w-[var(--radix-navigation-menu-viewport-width)]";

#[derive(Clone, PartialEq, Props)]
pub struct NavigationMenuContentProps {
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub force_mount: bool,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn NavigationMenuContent(props: NavigationMenuContentProps) -> Element {
    let ctx = use_context::<NavigationMenuContext>();
    let item_ctx = use_context::<NavigationMenuItemContext>();
    let open_item = ctx.open_item;
    let set_open_item = ctx.set_open_item;

    let is_open = use_memo(move || open_item() == Some(item_ctx.id));

    if !props.force_mount && !is_open() {
        return rsx! {};
    }

    let handle_mouse_leave = move |_| {
        set_open_item.call(None);
    };

    let data_state = if is_open() { "open" } else { "closed" };

    rsx! {
        div {
            "data-slot": "navigation-menu-content",
            "data-state": data_state,
            id: "{item_ctx.content_id}",
            "aria-labelledby": "{item_ctx.trigger_id}",
            role: "menu",
            class: cn(&cn(CONTENT_BASE, "relative"), &props.class),
            onmouseleave: handle_mouse_leave,
            ..props.attributes,
            div {
                class: INNER_BASE,
            }
        }
    }
}
