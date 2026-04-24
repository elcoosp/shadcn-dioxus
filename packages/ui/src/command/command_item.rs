use crate::cn;
use crate::command::CommandContext;
use dioxus::prelude::*;

const ITEM_BASE: &str = "relative flex cursor-default gap-2 select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none data-[disabled=true]:pointer-events-none data-[selected=true]:bg-accent data-[selected=true]:text-accent-foreground data-[disabled=true]:opacity-50";

#[derive(Clone, PartialEq, Props)]
pub struct CommandItemProps {
    #[props(into)]
    pub value: String,
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub on_select: Option<Callback<()>>,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CommandItem(props: CommandItemProps) -> Element {
    let ctx = use_context::<CommandContext>();
    let selected_id = ctx.selected_id;
    let set_selected_id = ctx.set_selected_id;
    let value = props.value.clone();
    let value_for_memo = props.value.clone();

    let is_selected = use_memo(move || {
        selected_id().as_deref() == Some(&value_for_memo)
    });

    let classes = cn(ITEM_BASE, &props.class);

    rsx! {
        div {
            "data-slot": "command-item",
            role: "option",
            "aria-selected": is_selected(),
            "data-disabled": props.disabled,
            "data-value": "{value}",
            class: "{classes}",
            onclick: move |_| {
                if !props.disabled {
                    set_selected_id.call(Some(value.clone()));
                    if let Some(cb) = &props.on_select {
                        cb.call(());
                    }
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}
