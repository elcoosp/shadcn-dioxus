---
title: Select
description: A dropdown for selecting one value from a list. Supports labels, separators, and disabled items.
component: true
---

## Usage

```bash
rust
use ui::{Select, SelectTrigger, SelectValue, SelectItem, SelectLabel, SelectSeparator};

<Select>
    <SelectTrigger>
        <SelectValue placeholder="Select..." />
    </SelectTrigger>
    <SelectContent>
        <SelectLabel>Fruit</SelectLabel>
        <SelectItem value="apple">Apple</SelectItem>
        <SelectSeparator />
        <SelectLabel>Vegetables</SelectLabel>
        <SelectItem value="banana">Banana</SelectItem>
        <SelectItem value="cherry" disabled>Cherry (unavailable)</SelectItem>
    </SelectContent>
</Select>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| default_value | String |  | Currently selected value. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{Select, SelectTrigger, SelectValue, SelectItem, SelectLabel, SelectSeparator};

<Select>
    <SelectTrigger>
        <SelectValue placeholder="Select..." />
    </SelectTrigger>
    <SelectContent>
        <SelectLabel>Fruit</SelectLabel>
        <SelectItem value="apple">Apple</SelectItem>
        <SelectSeparator />
        <SelectLabel>Vegetables</SelectLabel>
        <SelectItem value="banana">Banana</SelectItem>
        <SelectItem value="cherry" disabled>Cherry (unavailable)</SelectItem>
    </SelectContent>
</Select>

```
