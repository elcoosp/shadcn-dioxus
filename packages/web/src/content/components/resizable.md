---
title: Resizable
description: Resizable panels with draggable divider handles for adjusting their relative sizes.
component: true
---

## Usage

```bash
rust
use ui::{Resizable, ResizablePanel, ResizableHandle};

<Resizable direction={Direction::Horizontal}>
    <ResizablePanel default_size={0.5} />
    <ResizableHandle />
    <ResizablePanel default_size={0.5} />
</Resizable>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| direction | Direction | Direction::Horizontal | Resize direction. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{Resizable, ResizablePanel, ResizableHandle};

<Resizable direction={Direction::Horizontal}>
    <ResizablePanel default_size={0.5} />
    <ResizableHandle />
    <ResizablePanel default_size={0.5} />
</Resizable>

```
