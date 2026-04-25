---
title: Slider
description: A range input for selecting a numeric value within defined bounds with step increments.
component: true
---

## Usage

```bash
rust
use ui::Slider;

fn App() -> Element {
    let mut value = use_signal(|| 0.0);
    rsx! {
        Slider min={0.0} max={100.0} step={1.0} value={value} on_change={|v| value.set(v)} />
        <p class="text-sm text-muted-foreground">Value: {value}</p>
    }
}

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| min | f64 | 0.0 | Minimum value. |
| max | f64 | 100.0 | Maximum value. |
| step | f64 | 1.0 | Step increment. |
| value | f64 | 0.0 | Current value. |
| on_change | Option<Callback<f64>> | None | Callback when value changes. |
| disabled | bool | false | Disable the slider. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::Slider;

fn App() -> Element {
    let mut value = use_signal(|| 0.0);
    rsx! {
        Slider min={0.0} max={100.0} step={1.0} value={value} on_change={|v| value.set(v)} />
        <p class="text-sm text-muted-foreground">Value: {value}</p>
    }
}

```
