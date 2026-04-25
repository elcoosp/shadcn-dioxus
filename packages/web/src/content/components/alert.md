---
title: Alert
description: Displays a callout for important messages to the user. Built with composable sub-components for title, description, and actions.
component: true
---

## Usage

```bash
rust
use ui::{Alert, AlertTitle, AlertDescription};

<Alert>
    <AlertTitle>Heads up!</AlertTitle>
    <AlertDescription>
        Your session will expire in 5 minutes.
    </AlertDescription>
</Alert>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{Alert, AlertTitle, AlertDescription};

<Alert>
    <AlertTitle>Heads up!</AlertTitle>
    <AlertDescription>
        Your session will expire in 5 minutes.
    </AlertDescription>
</Alert>

```
