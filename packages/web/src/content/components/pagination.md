---
title: Pagination
description: Navigation controls for browsing through pages of data. Includes previous/next buttons and a page indicator.
component: true
---

## Usage

```bash
rust
use ui::{Pagination, PaginationContent, PaginationItem, PaginationPrevious, PaginationNext};

<Pagination page={1} total_pages={5}>
    <PaginationContent>
        <PaginationPrevious />
        <PaginationItem>{1}</PaginationItem>
        <PaginationNext />
    </PaginationContent>
</Pagination>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| page | usize | 1 | Current page number. |
| total_pages | usize | 5 | Total number of pages. |
| on_change | Option<Callback<usize>> | None | Callback when the page changes. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{Pagination, PaginationContent, PaginationItem, PaginationPrevious, PaginationNext};

<Pagination page={1} total_pages={5}>
    <PaginationContent>
        <PaginationPrevious />
        <PaginationItem>{1}</PaginationItem>
        <PaginationNext />
    </PaginationContent>
</Pagination>

```
