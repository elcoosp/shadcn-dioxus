use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct DataTableColumn {
    pub id: String,
    pub header: String,
    pub sortable: bool,
}

impl DataTableColumn {
    pub fn new(id: impl Into<String>, header: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            header: header.into(),
            sortable: true,
        }
    }

    pub fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum SortDirection {
    #[default]
    None,
    Asc,
    Desc,
}

#[derive(Clone, PartialEq)]
pub struct DataTableContext {
    pub columns: Signal<Vec<DataTableColumn>>,
    pub rows: Signal<Vec<Vec<String>>>,
    pub sort_column: Signal<String>,
    pub sort_direction: Signal<SortDirection>,
    pub selected_rows: Signal<Vec<usize>>,
    pub column_visibility: Signal<Vec<String>>,
    pub filter_text: Signal<String>,
    pub page: Signal<usize>,
    pub page_size: Signal<usize>,
    pub visible_column_ids: Signal<Vec<String>>,
    pub processed_rows: Signal<Vec<(usize, Vec<String>)>>,
    pub total_filtered: Signal<usize>,
}

#[derive(Clone, PartialEq, Props)]
pub struct DataTableProps {
    pub columns: Vec<DataTableColumn>,
    pub rows: Vec<Vec<String>>,
    #[props(default = 10)]
    pub page_size: usize,
    #[props(default = true)]
    pub show_toolbar: bool,
    #[props(default = true)]
    pub show_pagination: bool,
    #[props(default)]
    pub on_selection_change: Option<Callback<Vec<usize>>>,
    #[props(into, default)]
    pub class: String,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn DataTable(props: DataTableProps) -> Element {
    let mut columns = use_signal(|| props.columns.clone());
    let mut rows = use_signal(|| props.rows.clone());
    let mut sort_column = use_signal(|| String::new());
    let mut sort_direction = use_signal(|| SortDirection::None);
    let mut selected_rows = use_signal(|| Vec::<usize>::new());
    let mut column_visibility = use_signal(|| {
        props.columns.iter().map(|c| c.id.clone()).collect::<Vec<_>>()
    });
    let mut filter_text = use_signal(|| String::new());
    let mut page = use_signal(|| 0usize);
    let page_size = use_signal(|| props.page_size);

    let initial_processed: Vec<(usize, Vec<String>)> = props
        .rows
        .iter()
        .enumerate()
        .map(|(i, r)| (i, r.clone()))
        .collect();

    let mut visible_column_ids = use_signal(|| {
        props.columns.iter().map(|c| c.id.clone()).collect::<Vec<_>>()
    });
    let mut processed_rows = use_signal(|| initial_processed);
    let mut total_filtered = use_signal(|| props.rows.len());

    use_effect(move || { columns.set(props.columns.clone()); });
    use_effect(move || { rows.set(props.rows.clone()); });

    use_effect(move || {
        if let Some(cb) = &props.on_selection_change {
            cb.call(selected_rows());
        }
    });

    use_effect(move || {
        let cols = columns();
        let vis = column_visibility();
        let mut vis_ids: Vec<String> =
            cols.iter().filter(|c| vis.contains(&c.id)).map(|c| c.id.clone()).collect();
        visible_column_ids.set(vis_ids.clone());

        let all_rows = rows();
        let filter = filter_text();
        let sort_col = sort_column();
        let sort_dir = sort_direction();

        let sort_idx = vis_ids.iter().position(|id| id == &sort_col);

        let mut result: Vec<(usize, Vec<String>)> =
            all_rows.into_iter().enumerate().collect();

        if !filter.is_empty() {
            let lower_filter = filter.to_lowercase();
            result.retain(|(_, row)| {
                row.iter()
                    .any(|cell| cell.to_lowercase().contains(&lower_filter))
            });
        }

        if let Some(idx) = sort_idx {
            let dir = sort_dir;
            result.sort_by(|a, b| {
                let a_val = a.1.get(idx).map(|s| s.as_str()).unwrap_or("");
                let b_val = b.1.get(idx).map(|s| s.as_str()).unwrap_or("");
                match dir {
                    SortDirection::Asc => a_val.cmp(b_val),
                    SortDirection::Desc => b_val.cmp(a_val),
                    SortDirection::None => std::cmp::Ordering::Equal,
                }
            });
        }

        let total = result.len();
        total_filtered.set(total);
        processed_rows.set(result);

        let ps = page_size();
        if ps > 0 && total > 0 {
            let max_page = (total - 1) / ps;
            if page() > max_page {
                page.set(max_page);
            }
        }
    });

    use_effect(move || {
        let _ = filter_text();
        let _ = sort_column();
        let _ = sort_direction();
        page.set(0);
    });

    use_context_provider(|| DataTableContext {
        columns,
        rows,
        sort_column,
        sort_direction,
        selected_rows,
        column_visibility,
        filter_text,
        page,
        page_size,
        visible_column_ids,
        processed_rows,
        total_filtered,
    });

    rsx! {
        div {
            "data-slot": "data-table",
            class: "flex flex-col gap-4 {props.class}",
            ..props.attributes,
            if props.show_toolbar {
                super::DataTableToolbar {}
            }
            div { class: "rounded-md border",
                crate::Table {
                    super::DataTableHeader {}
                    super::DataTableBody {}
                }
            }
            if props.show_pagination {
                super::DataTablePagination {}
            }
        }
    }
}
