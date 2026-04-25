---
title: Command Palette
description: A floating panel for searching commands and actions, navigable with arrow keys. Groups items under headings with optional keyboard shortcut labels.
component: true
---

## Usage

```bash
rust
use ui::{Command, CommandInput, CommandList, CommandGroup, CommandItem, CommandEmpty, CommandSeparator, CommandShortcut};

<Command class="w-[500px] rounded-lg border shadow-md">
    <CommandInput placeholder="Type a command or search..." />
    <CommandList max_height={300}>
        <CommandGroup heading="Suggestions">
            CommandItem value="calendar".to_string() "Calendar" CommandShortcut { "\u2318K" } />
            CommandItem value="search-files".to_string() "Search Files" CommandShortcut { "\u2318P" } />
            CommandItem value="settings".to_string() "Settings" CommandShortcut { "\u2318," } />
        </CommandGroup>
        <CommandSeparator />
        <CommandGroup heading="Actions">
            CommandItem value="new-file".to_string() "New File" CommandShortcut { "\u2318N" } />
            CommandItem value="new-project".to_string() "New Project" />
        </CommandGroup>
        <CommandSeparator />
        <CommandGroup heading="Navigation">
            CommandItem value="go-home".to_string() "Go to Home" CommandShortcut { "\u2318H" } />
            CommandItem value="go-inbox".to_string() "Go to Inbox" CommandShortcut { "\u2318I" } />
        </CommandGroup>
        <CommandSeparator />
        <CommandGroup heading="Help">
            CommandItem value="documentation".to_string() "Documentation" CommandShortcut { "F1" } />
            CommandItem value="support".to_string() "Support" disabled: true />
        </CommandGroup>
        <CommandEmpty { "No commands found." } />
    </CommandList>
</Command>

```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| default_value | String |  | Pre-filled search value. |
| on_select | Option<Callback<String>> | None | Callback when an item is selected. |
| class | String |  | Additional CSS classes to apply to the root element. |
```

## Examples

```bash
rust
use ui::{Command, CommandInput, CommandList, CommandGroup, CommandItem, CommandEmpty, CommandSeparator, CommandShortcut};

<Command class="w-[500px] rounded-lg border shadow-md">
    <CommandInput placeholder="Type a command or search..." />
    <CommandList max_height={300}>
        <CommandGroup heading="Suggestions">
            CommandItem value="calendar".to_string() "Calendar" CommandShortcut { "\u2318K" } />
            CommandItem value="search-files".to_string() "Search Files" CommandShortcut { "\u2318P" } />
            CommandItem value="settings".to_string() "Settings" CommandShortcut { "\u2318," } />
        </CommandGroup>
        <CommandSeparator />
        <CommandGroup heading="Actions">
            CommandItem value="new-file".to_string() "New File" CommandShortcut { "\u2318N" } />
            CommandItem value="new-project".to_string() "New Project" />
        </CommandGroup>
        <CommandSeparator />
        <CommandGroup heading="Navigation">
            CommandItem value="go-home".to_string() "Go to Home" CommandShortcut { "\u2318H" } />
            CommandItem value="go-inbox".to_string() "Go to Inbox" CommandShortcut { "\u2318I" } />
        </CommandGroup>
        <CommandSeparator />
        <CommandGroup heading="Help">
            CommandItem value="documentation".to_string() "Documentation" CommandShortcut { "F1" } />
            CommandItem value="support".to_string() "Support" disabled: true />
        </CommandGroup>
        <CommandEmpty { "No commands found." } />
    </CommandList>
</Command>

```
