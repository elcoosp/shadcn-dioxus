---
title: Combobox
description: A searchable dropdown that filters options as the user types. Displays matching items in a list below the input.
component: true
---

## Usage

```bash
rust
use ui::Combobox;
use ui::combobox::ComboboxOption;

fn Example() -> Element {
    let options = vec![
        ComboboxOption { value: "apple".into(), label: "Apple".into() },
        ComboboxOption { value: "banana".into(), label: "Banana".into() },
        ComboboxOption { value: "cherry".into(), label: "Cherry".into() },
    ];
    rsx! {
        ComboBox options={options} placeholder="Select a fruit..." />
    }
}

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| options | Vec<ComboboxOption> |  | The list of selectable options. |
| placeholder | String | Select an option... | Placeholder text shown when the input is empty. |
| disabled | bool | false | Disable the combobox. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::Combobox;
use ui::combobox::ComboboxOption;

fn Example() -> Element {
    let options = vec![
        ComboboxOption { value: "apple".into(), label: "Apple".into() },
        ComboboxOption { value: "banana".into(), label: "Banana".into() },
        ComboboxOption { value: "cherry".into(), label: "Cherry".into() },
    ];
    rsx! {
        ComboBox options={options} placeholder="Select a fruit..." />
    }
}

```
