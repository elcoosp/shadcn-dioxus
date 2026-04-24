use crate::date_picker::{DataTableContext, SortDirection};
use dioxus::prelude::*;
use lucide_dioxus::{Check, ChevronDown, Minus};

#[component]
pub fn DataTableHeader() -> Element {
    let ctx = use_context::<DataTableContext>();
    let visible_column_ids = ctx.visible_column_ids;
    let sort_column = ctx.sort_column;
    let sort_direction = ctx.sort_direction;
    let selected_rows = ctx.selected_rows;
    let processed_rows = ctx.processed_rows;

    let all_selected = use_memo(move || {
        let sel = selected_rows();
        let proc = processed_rows();
        !proc.is_empty() && proc.iter().all(|(idx, _)| sel.contains(idx))
    });
    let some_selected = use_memo(move || {
        let sel = selected_rows();
        let proc = processed_rows();
        proc.iter().any(|(idx, _)| sel.contains(idx))
    });

    rsx! {
        crate::table::TableHeader {
            crate::table::TableRow {
                class: "border-b hover:bg-muted/50",
                crate::table::TableHead { class: "w-[40px]", "" }
                {
                    let cols = visible_column_ids();
                    let cur_sort = sort_column();
                    let cur_dir = sort_direction();
                    cols.iter().map(|col_id| {
                        let id = col_id.clone();
                        let is_sorted = cur_sort == id;
                        let dir = if is_sorted { cur_dir } else { SortDirection::None };
                        // Find the header text from columns
                        let header_text = {
                            let all_cols = ctx.columns();
                            all_cols
                                .iter()
                                .find(|c| c.id == col_id)
                                .map(|c| c.header.clone())
                                .unwrap_or_default()
                        };
                        let is_sortable = {
                            let all_cols = ctx.columns();
                            all_cols
                                .iter()
                                .find(|c| c.id == col_id)
                                .map(|c| c.sortable)
                                .unwrap_or(false)
                        };
                        rsx! {
                            crate::table::TableHead {
                                key: "{id}",
                                class: if is_sortable {
                                    "h-9 cursor-pointer select-none hover:bg-muted/50 data-[state=sorted]:bg-muted/100"
                                } else {
                                    "h-9"
                                },
                                onclick: move |_| {
                                    let current = sort_column();
                                    let current_dir = sort_direction();
                                    if current == id {
                                        match current_dir {
                                            SortDirection::Asc => sort_direction.set(SortDirection::Desc),
                                            SortDirection::Desc => {
                                                sort_column.set(String::new());
                                                sort_direction.set(SortDirection::None);
                                            }
                                            SortDirection::None => sort_direction.set(SortDirection::Asc),
                                        }
                                    } else {
                                        sort_column.set(id);
                                        sort_direction.set(SortDirection::Asc);
                                    }
                                },
                                span { class: "flex items-center gap-1",
                                    "{header_text}"
                                    match dir {
                                        SortDirection::Asc => rsx! {
                                            ChevronDown { class: "ml-1 h-4 w-4 text-foreground/70 -rotate-90" }
                                        },
                                        SortDirection::Desc => rsx! {
                                            ChevronDown { class: "ml-1 h-4 w-4 text-foreground/70" }
                                        },
                                        SortDirection::None => rsx! {},
                                    }
                                }
                            }
                        }
                    }).collect::<Vec<_>>()
                }
            }
        }
    }
}
