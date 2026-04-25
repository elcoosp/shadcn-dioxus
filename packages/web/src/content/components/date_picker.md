---
title: Date Picker
description: A date selection input that opens a calendar in a popover when clicked. Supports pre-selected dates and change callbacks.
component: true
---

## Usage

```bash
rust
use ui::DatePicker;

fn App() -> Element {
    rsx! {
        DatePicker placeholder="Select a date" on_change={|date| println!("{:?}", date)} />
    }
}

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| default_date | Option<(i32, u32, u32)> | None | Pre-selected date. |
| placeholder | String | Pick a date | Placeholder text when no date is selected. |
| on_change | Option<Callback<Option<(i32, u32, u32)>> | None | Callback when a date is picked. |
| disabled | bool | false | Disable the picker trigger. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::DatePicker;

fn App() -> Element {
    rsx! {
        DatePicker placeholder="Select a date" on_change={|date| println!("{:?}", date)} />
    }
}

```
