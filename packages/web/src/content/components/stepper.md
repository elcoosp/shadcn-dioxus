---
title: Stepper
description: A multi-step form wizard with labeled steps, a progress indicator, and previous/next navigation.
component: true
---

## Usage

```bash
rust
use ui::{Stepper, StepperItem, StepperTitle, StepperDescription, StepperSeparator, StepperFooter, StepperPrevious, StepperNext, StepperIndicator};

<Stepper default_step={0}>
    <StepperItem step={0}>
        <StepperTitle>Account</StepperTitle>
        <StepperDescription>Create your account.</StepperDescription>
    </StepperItem>
    <StepperSeparator />
    <StepperItem step={1}>
        <StepperTitle>Profile</StepperTitle>
        <StepperDescription>Set up your profile.</StepperDescription>
    </StepperItem>
    <StepperSeparator />
    <StepperItem step={2}>
        <StepperTitle>Review</StepperTitle>
        <StepperDescription>Confirm everything.</StepperDescription>
    </StepperItem>
    <StepperFooter>
        <StepperPrevious />
        <StepperIndicator />
        <StepperNext />
    </StepperFooter>
</Stepper>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| default_step | usize | 0 | Starting step. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{Stepper, StepperItem, StepperTitle, StepperDescription, StepperSeparator, StepperFooter, StepperPrevious, StepperNext, StepperIndicator};

<Stepper default_step={0}>
    <StepperItem step={0}>
        <StepperTitle>Account</StepperTitle>
        <StepperDescription>Create your account.</StepperDescription>
    </StepperItem>
    <StepperSeparator />
    <StepperItem step={1}>
        <StepperTitle>Profile</StepperTitle>
        <StepperDescription>Set up your profile.</StepperDescription>
    </StepperItem>
    <StepperSeparator />
    <StepperItem step={2}>
        <StepperTitle>Review</StepperTitle>
        <StepperDescription>Confirm everything.</StepperDescription>
    </StepperItem>
    <StepperFooter>
        <StepperPrevious />
        <StepperIndicator />
        <StepperNext />
    </StepperFooter>
</Stepper>

```
