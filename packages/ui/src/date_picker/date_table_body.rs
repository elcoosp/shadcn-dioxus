use crate::date_picker::DataTableContext;
use dioxus::prelude::*;
use lucide_dioxus::Check;

#[component]
pub fn DataTableBody() -> Element {
    let ctx = use_context::<DataTableContext>();
    let visible_column_ids = ctx.visible_column_ids;
    let processed_rows = ctx.processed_rows;
    let selected_rows = ctx.selected_rows;
    let columns = ctx.columns;

    let rows = processed_rows();

    if rows.is_empty() {
        return rsx! {
            crate::table::TableBody {
                crate::table::TableRow {
                    crate::table::TableCell {
                        col_span: visible_column_ids().len() + 1,
                        class: "h-24 text-center",
                        div { class: "text-muted-foreground", "No results." }
                    }
                }
            }
        };
    }

    rsx! {
        crate::table::TableBody {
            {
                rows.iter().map(|(original_idx, row_data)| {
                    let idx = *original_idx;
                    let is_selected = use_memo(move || selected_rows().contains(&idx));
                    rsx! {
                        crate::table::TableRow {
                            key: "{idx}",
                            class: "data-[state=selected]:bg-muted/50",
                            crate::table::TableCell { class: "w-[40px]",
                                button {
                                    r#type: "button",
                                    class: "flex h-4 w-4 items-center justify-center rounded border border-primary data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground",
                                    "data-state": if is_selected() { "checked" } else { "unchecked" },
                                    "aria-checked": is_selected(),
                                    onclick: move |_| {
                                        let mut sel = selected_rows();
                                        if sel.contains(&idx) {
                                            sel.retain(|x| x != &idx);
                                        } else {
                                            sel.push(idx);
                                        }
                                        selected_rows.set(sel);
                                    },
                                    if is_selected() {
                                        Check { class: "h-3.5 w-3.5" }
                                    }
                                }
                            }
                            {
                                let cols = visible_column_ids();
                                cols.iter().map(|col_id| {
                                    let cell_value = row_data
                                        .iter()
                                        .enumerate()
                                        .find(|(_, cell)| {
                                            let all_cols = columns();
                                            all_cols
                                                .iter()
                                                .position(|c| &c.id == col_id)
                                                .map(|pos| pos == Some(row_data.iter().position(|_| true).unwrap_or(0))
                                                .is_some()
                                        })
                                        .map(|(_, cell)| cell.as_str())
                                        .unwrap_or("");
                                    rsx! {
                                        crate::table::TableCell { key: "{col_id}", "{cell_value}" }
                                    }
                                }).collect::<Vec<_>>()
                            }
                        }
                    }
                }).collect::<Vec<_>>()
            }
        }
    }
}
PLACEHOLDER_EOF

# --- data_table_pagination.rs ---
cat > packages/ui/src/date_picker/data_table_pagination.rs << 'EOF'
use crate::cn;
use crate::date_picker::DataTableContext;
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
