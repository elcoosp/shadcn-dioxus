---
title: Navigation Menu
description: A horizontal or vertical navigation menu with items that can have dropdown sub-menus with links.
component: true
---

## Usage

```bash
rust
use ui::{NavigationMenu, NavigationMenuList, NavigationMenuItem, NavigationMenuLink, NavigationMenuTrigger};

<NavigationMenu orientation={NavigationMenuOrientation::Horizontal}>
    <NavigationMenuList>
        <NavigationMenuItem>
            <NavigationMenuTrigger>Getting Started</NavigationMenuTrigger>
        </NavigationMenuItem>
        <NavigationMenuItem>
            <NavigationMenuLink href="/docs">Docs</NavigationMenuLink>
        </NavigationMenuItem>
    </NavigationMenuList>
</NavigationMenu>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| orientation | NavigationMenuOrientation | NavigationMenuOrientation::Horizontal | Layout direction. |
| skip_delay_show | bool | false | Skip hover delay for dropdowns. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{NavigationMenu, NavigationMenuList, NavigationMenuItem, NavigationMenuLink, NavigationMenuTrigger};

<NavigationMenu orientation={NavigationMenuOrientation::Horizontal}>
    <NavigationMenuList>
        <NavigationMenuItem>
            <NavigationMenuTrigger>Getting Started</NavigationMenuTrigger>
        </NavigationMenuItem>
        <NavigationMenuItem>
            <NavigationMenuLink href="/docs">Docs</NavigationMenuLink>
        </NavigationMenuItem>
    </NavigationMenuList>
</NavigationMenu>

```
