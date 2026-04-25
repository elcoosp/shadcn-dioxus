---
title: Dropdown Menu
description: A dropdown that appears below a trigger button. Supports labels, separators, and shortcut labels.
component: true
---

## Usage

```bash
rust
use ui::{DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuShortcut};

<DropdownMenu>
    <DropdownMenuTrigger>
        <button>Open Menu</button>
    </DropdownMenuTrigger>
    <DropdownMenuContent align="end">
        <DropdownMenuLabel>Actions</DropdownMenuLabel>
        <DropdownMenuItem onclick={|| tracing.info("Edit")}>Edit</DropdownMenuItem>
        <DropdownMenuSeparator />
        <DropdownMenuItem onclick={|| tracing.info("Delete")}>Delete</DropdownMenuItem>
    </DropdownMenuContent>
</DropdownMenu>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuShortcut};

<DropdownMenu>
    <DropdownMenuTrigger>
        <button>Open Menu</button>
    </DropdownMenuTrigger>
    <DropdownMenuContent align="end">
        <DropdownMenuLabel>Actions</DropdownMenuLabel>
        <DropdownMenuItem onclick={|| tracing.info("Edit")}>Edit</DropdownMenuItem>
        <DropdownMenuSeparator />
        <DropdownMenuItem onclick={|| tracing.info("Delete")}>Delete</DropdownMenuItem>
    </DropdownMenuContent>
</DropdownMenu>

```
