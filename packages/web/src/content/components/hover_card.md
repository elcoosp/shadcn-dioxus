---
title: Hover Card
description: A floating card with rich content that appears when hovering over a trigger element.
component: true
---

## Usage

```bash
rust
use ui::{HoverCard, HoverCardContent, HoverCardTrigger};

<HoverCard>
    <HoverCardTrigger>
        <span class="font-semibold underline">Hover me</span>
    </HoverCardTrigger>
    <HoverCardContent side="top" class="w-64">
        <p class="font-medium">This card appeared on hover.</p>
        <p class="text-sm text-muted-foreground">It stays in view while hovered.</p>
    </HoverCardContent>
</HoverCard>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| delay_duration | u64 | 0 | Hover delay in milliseconds. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{HoverCard, HoverCardContent, HoverCardTrigger};

<HoverCard>
    <HoverCardTrigger>
        <span class="font-semibold underline">Hover me</span>
    </HoverCardTrigger>
    <HoverCardContent side="top" class="w-64">
        <p class="font-medium">This card appeared on hover.</p>
        <p class="text-sm text-muted-foreground">It stays in view while hovered.</p>
    </HoverCardContent>
</HoverCard>

```
