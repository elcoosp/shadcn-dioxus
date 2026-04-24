use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ComboboxOption {
    pub value: String,
    pub label: String,
}

#[derive(Clone, PartialEq, Props)]
pub struct ComboboxProps {
    pub options: Vec<ComboboxOption>,
    #[props(default = "Select an option...".to_string())]
    pub placeholder: String,
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

const INPUT_CLASS: &str = "border-input shadow-xs flex h-9 w-full min-w-0 rounded-md border bg-transparent px-3 py-2 text-sm outline-none transition-[color,box-shadow] disabled:cursor-not-allowed disabled:opacity-50 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]";

const CONTENT_CLASS: &str = "bg-popover text-popover-foreground z-50 min-w-[8rem] overflow-hidden rounded-md border p-1 shadow-md absolute left-0 top-full mt-1 w-full max-h-[400px]";

const ITEM_CLASS: &str = "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors focus:bg-accent focus:text-accent-foreground data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50";

const EMPTY_CLASS: &str = "py-6 text-center text-sm text-muted-foreground";

#[component]
pub fn Combobox(props: ComboboxProps) -> Element {
    let mut search = use_signal(|| String::new());

    let filtered = use_memo(move || {
        let s = search();
        if s.is_empty() {
            props.options.clone()
        } else {
            props.options
                .iter()
                .filter(|o| o.label.to_lowercase().contains(&s.to_lowercase()))
                .cloned()
                .collect::<Vec<_>>()
        }
    });

    let input_class = format!("{} {}", INPUT_CLASS, props.class);
    let has_results = !filtered().is_empty();
    let has_search = !search().is_empty();

    rsx! {
        div {
            "data-slot": "combobox",
            class: "{input_class}",
            ..props.attributes,
            input {
                r#type: "text",
                "data-slot": "combobox-input",
                placeholder: "{props.placeholder}",
                disabled: props.disabled,
                value: "{search}",
                oninput: move |e| search.set(e.value()),
            }
            if has_results {
                rsx! {
                    div {
                        "data-slot": "combobox-content",
                        "role": "listbox",
                        class: CONTENT_CLASS,
                        div { class: "p-1 overflow-y-auto max-h-[300px]",
                            for option in filtered().iter() {
                                rsx! {
                                    div {
                                        key: "{option.value}",
                                        "data-slot": "combobox-item",
                                        role: "option",
                                        "data-value": "{option.value}",
                                        tabindex: "0",
                                        class: ITEM_CLASS,
                                        onclick: move |_| {
                                            search.set(String::new())
                                        },
                                        "{option.label}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if has_search && !has_results {
                rsx! {
                    div { class: EMPTY_CLASS, "No results found." }
                }
            }
        }
    }
}
