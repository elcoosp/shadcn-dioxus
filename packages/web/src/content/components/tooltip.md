---
title: Tooltip
description: A small informational overlay that appears on hover or focus near a trigger element.
component: true
---

## Usage

```bash
rust
use ui::{Tooltip, TooltipTrigger, TooltipContent};

<Tooltip>
    <TooltipTrigger>
        <button>Hover me</button>
    </TooltipTrigger>
    <TooltipContent side="top">
        <p>This is a tooltip.</p>
    </TooltipContent>
</Tooltip>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| delay_duration | u64 | 0 | Delay before showing the tooltip. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{Tooltip, TooltipTrigger, TooltipContent};

<Tooltip>
    <TooltipTrigger>
        <button>Hover me</button>
    </TooltipTrigger>
    <TooltipContent side="top">
        <p>This is a tooltip.</p>
    </TooltipContent>
</Tooltip>

```
