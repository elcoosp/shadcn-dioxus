---
title: Chart
description: A simple bar chart component that renders SVG bars from a list of data points with labels, values, and colors.
component: true
---

## Usage

```bash
rust
use ui::Chart;
use ui::chart::ChartBar;

fn Example() -> Element {
    let bars = vec![
        ChartBar { label: "Jan".into(), value: 65.0, color: "#3b82f6".into() },
        ChartBar { label: "Feb".into(), value: 45.0, color: "#2563eb".into() },
        ChartBar { label: "Mar".into(), value: 80.0, color: "#16a34a".into() },
    ];
    rsx! {
        Chart height={200.0} width={400.0} bars={bars} />
    }
}

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| height | f64 | 200.0 | Chart height in CSS pixels. |
| width | f64 | 400.0 | Chart width in CSS pixels. |
| bar_gap | f64 | 4.0 | Gap between bars in pixels. |
| bars | Vec<ChartBar> |  | The data points to render as bars. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::Chart;
use ui::chart::ChartBar;

fn Example() -> Element {
    let bars = vec![
        ChartBar { label: "Jan".into(), value: 65.0, color: "#3b82f6".into() },
        ChartBar { label: "Feb".into(), value: 45.0, color: "#2563eb".into() },
        ChartBar { label: "Mar".into(), value: 80.0, color: "#16a34a".into() },
    ];
    rsx! {
        Chart height={200.0} width={400.0} bars={bars} />
    }
}

```
