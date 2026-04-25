---
title: Radio Group
description: A group of radio buttons where only one can be selected at a time. Provides accessible labels for each option.
component: true
---

## Usage

```bash
rust
use ui::{RadioGroup, RadioGroupItem, RadioGroupLabel};

<RadioGroup name="plan" default_value={Some("free".to_string())}>
    <RadioGroupItem value="free">Free</RadioGroupItem>
    <RadioGroupItem value="pro">Pro</RadioGroupItem>
    <RadioGroupLabel>"Choose your plan:"</RadioGroupLabel>
</RadioGroup>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| default_value | Option<String> | None | Pre-selected value. |
| name | String | radiogroup | Accessible group name. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{RadioGroup, RadioGroupItem, RadioGroupLabel};

<RadioGroup name="plan" default_value={Some("free".to_string())}>
    <RadioGroupItem value="free">Free</RadioGroupItem>
    <RadioGroupItem value="pro">Pro</RadioGroupItem>
    <RadioGroupLabel>"Choose your plan:"</RadioGroupLabel>
</RadioGroup>

```
