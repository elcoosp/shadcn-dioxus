---
title: Input OTP
description: A one-time password input that automatically advances focus to the next slot. Commonly used for verification codes.
component: true
---

## Usage

```bash
rust
use ui::InputOtp;

<InputOtp max_length={6} placeholder="Enter OTP" />

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| max_length | usize | 6 | Maximum number of characters. |
| value | String |  | Current OTP string. |
| on_change | Option<Callback<String>> | None | Callback when the OTP changes. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::InputOtp;

<InputOtp max_length={6} placeholder="Enter OTP" />

```
