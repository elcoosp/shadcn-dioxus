use crate::cn;
use crate::data_table::DataTableContext;
use crate::native_select::{NativeSelect, NativeSelectOption};
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct DataTablePaginationProps {
    #[props(default = vec![10, 20, 30, 40, 50])]
    pub page_size_options: Vec<usize>,
    #[props(into, default)]
    pub class: String,
}

#[component]
pub fn DataTablePagination(props: DataTablePaginationProps) -> Element {
    let ctx = use_context::<DataTableContext>();
    let page = ctx.page;
    let page_size = ctx.page_size;
    let total_filtered = ctx.total_filtered;

    let total_pages = use_memo(move || {
        let ps = page_size();
        let total = total_filtered();
        if ps > 0 && total > 0 {
            (total + ps - 1) / ps
        } else {
            1
        }
    });

    let current_page = page();
    let ps = page_size();
    let total = total_filtered();
    let start = if total > 0 { current_page * ps + 1 } else { 0 };
    let end = std::cmp::min((current_page + 1) * ps, total);

    let can_prev = current_page > 0;
    let can_next = current_page < total_pages() - 1;

    let classes = cn(
        "flex flex-col-reverse gap-3 sm:flex-row sm:items-center sm:justify-between px-2",
        &props.class,
    );

    rsx! {
        div { class: "{classes}",
            div { class: "text-sm text-muted-foreground",
                if total > 0 {
                    "Showing {start} to {end} of {total} entries"
                } else {
                    "0 entries"
                }
            }
            div { class: "flex items-center gap-6 lg:gap-8",
                div { class: "flex items-center gap-2",
                    label { class: "text-sm font-medium", "Rows per page" }
                    NativeSelect {
                        class: "h-8 w-[70px]",
                        onchange: move |e| {
                            if let Ok(val) = e.value().parse::<usize>() {
                                page_size.set(val);
                                page.set(0);
                            }
                        },
                        {
                            props.page_size_options.iter().map(|size| {
                                let s = *size;
                                let selected = s == ps;
                                rsx! {
                                    NativeSelectOption {
                                        value: "{s}",
                                        selected,
                                        "{s}"
                                    }
                                }
                            }).collect::<Vec<_>>()
                        }
                    }
                }
                div { class: "flex items-center gap-1",
                    button {
                        r#type: "button",
                        class: "inline-flex h-8 w-8 items-center justify-center rounded-md border border-input bg-transparent text-sm font-medium disabled:pointer-events-none disabled:opacity-50",
                        disabled: !can_prev,
                        onclick: move |_| page.set(page() - 1),
                        "‹"
                    }
                    span { class: "text-sm font-medium",
                        "Page {current_page} of {total_pages}"
                    }
                    button {
                        r#type: "button",
                        class: "inline-flex h-8 w-8 items-center justify-center rounded-md border border-input bg-transparent text-sm font-medium disabled:pointer-events-none disabled:opacity-50",
                        disabled: !can_next,
                        onclick: move |_| page.set(page() + 1),
                        "›"
                    }
                }
            }
        }
    }
}
