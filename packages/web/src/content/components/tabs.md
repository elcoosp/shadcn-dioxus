---
title: Tabs
description: Tabbed interface with selectable triggers that reveal different content panels.
component: true
---

## Usage

```bash
rust
use ui::{Tabs, TabsList, TabsTrigger, TabsContent};

<Tabs default_value="tab1">
    <TabsList>
        <TabsTrigger value="tab1">Tab 1</TabsTrigger>
        <TabsTrigger value="tab2">Tab 2</TabsTrigger>
    </TabsList>
    <TabsContent value="tab1">
        <div class="p-4">Tab 1 content here.</div>
    </TabsContent>
    <TabsContent value="tab2">
        <div class="p-4">Tab 2 content here.</div>
    </TabsContent>
</Tabs>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| default_value | String |  | Active tab ID. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{Tabs, TabsList, TabsTrigger, TabsContent};

<Tabs default_value="tab1">
    <TabsList>
        <TabsTrigger value="tab1">Tab 1</TabsTrigger>
        <TabsTrigger value="tab2">Tab 2</TabsTrigger>
    </TabsList>
    <TabsContent value="tab1">
        <div class="p-4">Tab 1 content here.</div>
    </TabsContent>
    <TabsContent value="tab2">
        <div class="p-4">Tab 2 content here.</div>
    </TabsContent>
</Tabs>

```
