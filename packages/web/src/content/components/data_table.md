---
title: Data Table
description: A full-featured data table with column sorting, text filtering, row selection with checkboxes, and client-side pagination.
component: true
---

## Usage

```bash
rust
use ui::{DataTable, DataTableColumn};

fn Example() -> Element {
    let columns = vec![
        DataTableColumn::new("name", "Name").sortable(true),
        DataTableColumn::new("email", "Email").sortable(true),
        DataTableColumn::new("role", "Role").sortable(true),
    ];
    let rows = vec![
        vec!["Alice", "alice@example.com", "Engineer"],
        vec!["Bob", "bob@example.com", "Designer"],
        vec!["Charlie", "charlie@example.com", "Manager"],
    ];
    rsx! {
        DataTable columns={columns} rows={rows} show_toolbar={true} show_pagination={true} />
    }
}

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| columns | Vec<DataTableColumn> |  | Column definitions. |
| rows | Vec<Vec<String>> |  | Row data. |
| page_size | usize | 10 | Rows per page. |
| show_toolbar | bool | true | Show the filter toolbar. |
| show_pagination | bool | true | Show pagination controls. |
| on_selection_change | Option<Callback<Vec<usize>>> | None | Callback when selected rows change. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{DataTable, DataTableColumn};

fn Example() -> Element {
    let columns = vec![
        DataTableColumn::new("name", "Name").sortable(true),
        DataTableColumn::new("email", "Email").sortable(true),
        DataTableColumn::new("role", "Role").sortable(true),
    ];
    let rows = vec![
        vec!["Alice", "alice@example.com", "Engineer"],
        vec!["Bob", "bob@example.com", "Designer"],
        vec!["Charlie", "charlie@example.com", "Manager"],
    ];
    rsx! {
        DataTable columns={columns} rows={rows} show_toolbar={true} show_pagination={true} />
    }
}

```
