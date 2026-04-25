---
title: Popover
description: A floating panel anchored to a trigger element. Supports four sides and optional force-mount rendering.
component: true
---

## Usage

```bash
rust
use ui::{Popover, PopoverTrigger, PopoverContent};

<Popover>
    <PopoverTrigger>Hover me</PopoverTrigger>
    <PopoverContent side="top">
        <div class="p-4 border rounded-md bg-background shadow-md">
            <p>This popover appears above the trigger.</p>
        </div>
    </PopoverContent>
</Popover>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| side | String | "bottom" | Position relative to the trigger. |
| force_mount | bool | false | Render even when hidden. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{Popover, PopoverTrigger, PopoverContent};

<Popover>
    <PopoverTrigger>Hover me</PopoverTrigger>
    <PopoverContent side="top">
        <div class="p-4 border rounded-md bg-background shadow-md">
            <p>This popover appears above the trigger.</p>
        </div>
    </PopoverContent>
</Popover>

```
