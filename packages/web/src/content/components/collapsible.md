---
title: Collapsible
description: A container whose content can be shown or hidden with a trigger. Supports animated expand/collapse transitions.
component: true
---

## Usage

```bash
rust
use ui::{Collapsible, CollapsibleTrigger, CollapsibleContent};

<Collapsible>
    <CollapsibleTrigger>Click me</CollapsibleTrigger>
    <CollapsibleContent>
        <div class="p-4 border rounded-md bg-muted">
            This content is collapsible and animates smoothly.
        </div>
    </CollapsibleContent>
</Collapsible>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| default_open | bool | false | Whether the content is visible on mount. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{Collapsible, CollapsibleTrigger, CollapsibleContent};

<Collapsible>
    <CollapsibleTrigger>Click me</CollapsibleTrigger>
    <CollapsibleContent>
        <div class="p-4 border rounded-md bg-muted">
            This content is collapsible and animates smoothly.
        </div>
    </CollapsibleContent>
</Collapsible>

```
