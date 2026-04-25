---
title: Toggle Group
description: A group of toggle buttons where one or more can be active at once. Supports single and multiple selection modes.
component: true
---

## Usage

```bash
rust
use ui::{ToggleGroup, ToggleGroupItem};

<ToggleGroup group_type={ToggleGroupType::Single}>
    <ToggleGroupItem value="bold" class="data-[state=on]:bg-accent text-accent-foreground shadow">B</ToggleGroupItem>
    <ToggleGroupItem value="italic" class="data-[state=off]:opacity-50">I</ToggleGroupItem>
    <ToggleGroupItem value="underline" class="data-[state=off]:opacity-50">U</ToggleGroupItem>
</ToggleGroup>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| group_type | ToggleGroupType | ToggleGroupType::Single | Selection mode. |
| default_value | Vec<String> | [] | Active values. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{ToggleGroup, ToggleGroupItem};

<ToggleGroup group_type={ToggleGroupType::Single}>
    <ToggleGroupItem value="bold" class="data-[state=on]:bg-accent text-accent-foreground shadow">B</ToggleGroupItem>
    <ToggleGroupItem value="italic" class="data-[state=off]:opacity-50">I</ToggleGroupItem>
    <ToggleGroupItem value="underline" class="data-[state=off]:opacity-50">U</ToggleGroupItem>
</ToggleGroup>

```
