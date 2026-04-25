---
title: Context Menu
description: A menu that appears near the cursor on right-click. Supports items, separators, and dismissal via Escape or outside click.
component: true
---

## Usage

```bash
rust
use ui::{ContextMenu, ContextMenuTrigger, ContextMenuContent, ContextMenuItem, ContextMenuSeparator};

<ContextMenu>
    <ContextMenuTrigger>
        <div class="p-4 border rounded-md bg-background shadow-md">Right-click me</div>
    </ContextMenuTrigger>
    <ContextMenuContent>
        <ContextMenuItem onclick={|| tracing.info("Copy")} >Copy</ContextMenuItem>
        <ContextMenuSeparator />
        <ContextMenuItem onclick={|| tracing.info("Paste")} >Paste</ContextMenuItem>
        <ContextMenuSeparator />
        <ContextMenuItem onclick={|| tracing.info("Delete")} >Delete</ContextMenuItem>
    </ContextMenuContent>
</ContextMenu>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{ContextMenu, ContextMenuTrigger, ContextMenuContent, ContextMenuItem, ContextMenuSeparator};

<ContextMenu>
    <ContextMenuTrigger>
        <div class="p-4 border rounded-md bg-background shadow-md">Right-click me</div>
    </ContextMenuTrigger>
    <ContextMenuContent>
        <ContextMenuItem onclick={|| tracing.info("Copy")} >Copy</ContextMenuItem>
        <ContextMenuSeparator />
        <ContextMenuItem onclick={|| tracing.info("Paste")} >Paste</ContextMenuItem>
        <ContextMenuSeparator />
        <ContextMenuItem onclick={|| tracing.info("Delete")} >Delete</ContextMenuItem>
    </ContextMenuContent>
</ContextMenu>

```
