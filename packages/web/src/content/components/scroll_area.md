---
title: Scroll Area
description: A scrollable container with styled scrollbars for content that overflows.
component: true
---

## Usage

```bash
rust
use ui::ScrollArea;

<ScrollArea class="h-48 w-64 overflow-y-auto rounded-md border">
    <div class="p-4">
        <p>Scroll down for more content...</p>
        <p>More content...</p>
        <p>Even more content...</p>
    </div>
</ScrollArea>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| scrollbar_width | String | "5px" | Scrollbar width. |
| scrollbar_height | String | "5px" | Scrollbar height. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::ScrollArea;

<ScrollArea class="h-48 w-64 overflow-y-auto rounded-md border">
    <div class="p-4">
        <p>Scroll down for more content...</p>
        <p>More content...</p>
        <p>Even more content...</p>
    </div>
</ScrollArea>

```
