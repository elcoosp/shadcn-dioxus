---
title: Drawer
description: A panel that slides in from the specified edge of the viewport. Supports all four sides and includes a close button.
component: true
---

## Usage

```bash
rust
use ui::{Drawer, DrawerContent, DrawerHeader, DrawerTitle, DrawerDescription, DrawerFooter, DrawerClose, DrawerTrigger};

<Drawer>
    <DrawerTrigger>Open Drawer</DrawerTrigger>
    <DrawerContent side={DrawerSide::Right}>
        <DrawerHeader>
            <DrawerTitle>Title</DrawerTitle>
            <DrawerDescription>Description</DrawerDescription>
        </DrawerHeader>
        <DrawerFooter>
            <DrawerClose>Close</DrawerClose>
        </DrawerFooter>
    </DrawerContent>
</Drawer>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| side | DrawerSide | DrawerSide::Right | Which edge the drawer slides from. |
| show_close_button | bool | true | Show the close button. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{Drawer, DrawerContent, DrawerHeader, DrawerTitle, DrawerDescription, DrawerFooter, DrawerClose, DrawerTrigger};

<Drawer>
    <DrawerTrigger>Open Drawer</DrawerTrigger>
    <DrawerContent side={DrawerSide::Right}>
        <DrawerHeader>
            <DrawerTitle>Title</DrawerTitle>
            <DrawerDescription>Description</DrawerDescription>
        </DrawerHeader>
        <DrawerFooter>
            <DrawerClose>Close</DrawerClose>
        </DrawerFooter>
    </DrawerContent>
</Drawer>

```
