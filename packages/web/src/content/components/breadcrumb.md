---
title: Breadcrumb
description: Displays the path to the current page using a horizontal trail of links, providing spatial context in navigation.
component: true
---

## Usage

```bash
rust
use ui::{Breadcrumb, BreadcrumbList, BreadcrumbItem, BreadcrumbLink, BreadcrumbPage, BreadcrumbSeparator};

<BreadcrumbList>
    <BreadcrumbItem>
        <BreadcrumbLink href="/">Home</BreadcrumbLink>
    </BreadcrumbItem>
    <BreadcrumbSeparator />
    <BreadcrumbItem>
        <BreadcrumbLink href="/docs">Documentation</BreadcrumbLink>
    </BreadcrumbItem>
    <BreadcrumbItem>
        <BreadcrumbPage>Components</BreadcrumbPage>
    </BreadcrumbItem>
</BreadcrumbList>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{Breadcrumb, BreadcrumbList, BreadcrumbItem, BreadcrumbLink, BreadcrumbPage, BreadcrumbSeparator};

<BreadcrumbList>
    <BreadcrumbItem>
        <BreadcrumbLink href="/">Home</BreadcrumbLink>
    </BreadcrumbItem>
    <BreadcrumbSeparator />
    <BreadcrumbItem>
        <BreadcrumbLink href="/docs">Documentation</BreadcrumbLink>
    </BreadcrumbItem>
    <BreadcrumbItem>
        <BreadcrumbPage>Components</BreadcrumbPage>
    </BreadcrumbItem>
</BreadcrumbList>

```
