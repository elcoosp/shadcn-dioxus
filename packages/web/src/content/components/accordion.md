---
title: Accordion
description: A vertically stacked set of interactive headings that reveal or hide associated content panels. Only one panel can be open at a time by default, or set `multiple` to allow simultaneous open panels.
component: true
---

## Usage

```bash
rust
use ui::{Accordion, AccordionItem, AccordionContent, AccordionTrigger};

<Accordion default_value={vec!["section-1".to_string()]} class="w-full">
    <AccordionItem value="section-1">
        <AccordionTrigger>Section 1</AccordionTrigger>
        <AccordionContent>Content for section 1 goes here.</AccordionContent>
    </AccordionItem>
    <AccordionItem value="section-2">
        <AccordionTrigger>Section 2</AccordionTrigger>
        <AccordionContent>Content for section 2 goes here.</AccordionContent>
    </AccordionItem>
    <AccordionItem value="section-3">
        <AccordionTrigger>Section 3</AccordionTrigger>
        <AccordionContent>Content for section 3 goes here.</AccordionContent>
    </AccordionItem>
</Accordion>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| multiple | bool | false | Whether multiple panels can be open simultaneously. |
| default_value | Vec<String> | [] | IDs of sections that are open by default. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{Accordion, AccordionItem, AccordionContent, AccordionTrigger};

<Accordion default_value={vec!["section-1".to_string()]} class="w-full">
    <AccordionItem value="section-1">
        <AccordionTrigger>Section 1</AccordionTrigger>
        <AccordionContent>Content for section 1 goes here.</AccordionContent>
    </AccordionItem>
    <AccordionItem value="section-2">
        <AccordionTrigger>Section 2</AccordionTrigger>
        <AccordionContent>Content for section 2 goes here.</AccordionContent>
    </AccordionItem>
    <AccordionItem value="section-3">
        <AccordionTrigger>Section 3</AccordionTrigger>
        <AccordionContent>Content for section 3 goes here.</AccordionContent>
    </AccordionItem>
</Accordion>

```
