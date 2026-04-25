---
title: Table
description: A semantic HTML table with header, body, footer, and caption for tabular data.
component: true
---

## Usage

```bash
rust
use ui::{Table, TableHeader, TableBody, TableRow, TableHead, TableCell, TableFooter, TableCaption};

<Table class="w-full caption-top">
    <TableCaption>Monthly Sales</TableCaption>
    <TableHeader>
        <TableRow class="border-b">
            <TableHead class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Month</TableHead>
            <TableHead class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Sales</TableHead>
        </TableRow>
    </TableHeader>
    <TableBody>
        <TableRow>
            <TableCell>Jan</TableCell>
            <TableCell class="font-medium">$1,200</TableCell>
        </TableRow>
        <TableRow>
            <TableCell>Feb</TableCell>
            <TableCell class="font-medium">$900</TableCell>
        </TableRow>
    </TableBody>
    <TableFooter>
        <TableRow>
            <TableCell>Total</TableCell>
            <TableCell>$2,100</TableCell>
        </TableRow>
    </TableFooter>
</Table>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{Table, TableHeader, TableBody, TableRow, TableHead, TableCell, TableFooter, TableCaption};

<Table class="w-full caption-top">
    <TableCaption>Monthly Sales</TableCaption>
    <TableHeader>
        <TableRow class="border-b">
            <TableHead class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Month</TableHead>
            <TableHead class="h-10 px-2 text-left align-middle font-medium text-muted-foreground">Sales</TableHead>
        </TableRow>
    </TableHeader>
    <TableBody>
        <TableRow>
            <TableCell>Jan</TableCell>
            <TableCell class="font-medium">$1,200</TableCell>
        </TableRow>
        <TableRow>
            <TableCell>Feb</TableCell>
            <TableCell class="font-medium">$900</TableCell>
        </TableRow>
    </TableBody>
    <TableFooter>
        <TableRow>
            <TableCell>Total</TableCell>
            <TableCell>$2,100</TableCell>
        </TableRow>
    </TableFooter>
</Table>

```
