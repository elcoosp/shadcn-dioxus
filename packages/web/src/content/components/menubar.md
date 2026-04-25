---
title: Menubar
description: A horizontal menu bar at the top of the page for grouping related actions under dropdown sub-menus.
component: true
---

## Usage

```bash
rust
use ui::Menubar;

<Menubar class="w-full">
    <MenubarMenu>
        <MenubarTrigger>File</MenubarTrigger>
        <MenubarContent>
            <MenubarItem>Edit</MenubarItem>
            <MenubarSeparator />
            <MenubarItem>Save</MenubarItem>
        </MenubarContent>
    </MenubarMenu>
</Menubar>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::Menubar;

<Menubar class="w-full">
    <MenubarMenu>
        <MenubarTrigger>File</MenubarTrigger>
        <MenubarContent>
            <MenubarItem>Edit</MenubarItem>
            <MenubarSeparator />
            <MenubarItem>Save</MenubarItem>
        </MenubarContent>
    </MenubarMenu>
</Menubar>

```
