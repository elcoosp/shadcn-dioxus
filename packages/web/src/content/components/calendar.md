---
title: Calendar
description: A monthly calendar grid that allows the user to navigate months and select a date. Supports pre-selected dates and change callbacks.
component: true
---

## Usage

```bash
rust
use ui::Calendar;

<Calendar default_year={2025} default_month={5} />

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| default_year | i32 | 2024 | The year the calendar initially displays. |
| default_month | u32 | 0 | The month the calendar initially displays (0 = January). |
| default_date | Option<(i32, u32, u32)> | None | A date that is pre-selected when the calendar mounts. |
| on_change | Option<Callback<Option<(i32, u32, u32)>> | None | Callback fired when the user selects a date. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::Calendar;

<Calendar default_year={2025} default_month={5} />

```
