---
title: Breadcrumb
description: Displays the path to the current resource using a hierarchy of links.
component: true
---

<ComponentPreview name="breadcrumb-default"/>

## Usage

```rust
use ui::{Breadcrumb, BreadcrumbList, BreadcrumbItem, BreadcrumbLink, BreadcrumbPage, BreadcrumbSeparator};

rsx! {
    Breadcrumb {
        BreadcrumbList {
            BreadcrumbItem { BreadcrumbLink { href: "/", "Home" } }
            BreadcrumbSeparator {}
            BreadcrumbItem { BreadcrumbPage { "Components" } }
        }
    }
}
