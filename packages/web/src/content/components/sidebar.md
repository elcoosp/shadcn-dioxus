---
title: Sidebar
description: A collapsible side panel for app navigation with groups, labels, search input, and a rail toggle.
component: true
---

## Usage

```bash
rust
use ui::{Sidebar, SidebarContent, SidebarHeader, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarMenu, SidebarMenuItem, SidebarInput, SidebarTrigger, SidebarRail, SidebarSeparator, SidebarInset};
use ui::sidebar_context::SidebarVariant;

fn App() -> Element {
    rsx! {
        <SidebarProvider variant={SidebarVariant::Sidebar}>
            <div class="flex h-screen">
                <Sidebar collapsible={true}>
                    <SidebarHeader>Title</SidebarHeader>
                    <SidebarContent>...</SidebarContent>
                    <SidebarFooter>...</SidebarFooter>
                </Sidebar>
                <main class="flex-1">
                    <SidebarInset />
                </main>
            </div>
        </SidebarProvider>
    }
}

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| collapsible | bool | true | Whether the sidebar can collapse. |
| variant | SidebarVariant | SidebarVariant::Sidebar | Visual variant. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{Sidebar, SidebarContent, SidebarHeader, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarMenu, SidebarMenuItem, SidebarInput, SidebarTrigger, SidebarRail, SidebarSeparator, SidebarInset};
use ui::sidebar_context::SidebarVariant;

fn App() -> Element {
    rsx! {
        <SidebarProvider variant={SidebarVariant::Sidebar}>
            <div class="flex h-screen">
                <Sidebar collapsible={true}>
                    <SidebarHeader>Title</SidebarHeader>
                    <SidebarContent>...</SidebarContent>
                    <SidebarFooter>...</SidebarFooter>
                </Sidebar>
                <main class="flex-1">
                    <SidebarInset />
                </main>
            </div>
        </SidebarProvider>
    }
}

```
