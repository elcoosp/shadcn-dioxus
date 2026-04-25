use dioxus::prelude::*;
use ui::*;

#[component]
pub fn Showcase() -> Element {
    rsx! {
        div { class: "container mx-auto py-12 px-4 max-w-5xl",
            h1 { class: "text-4xl font-bold tracking-tight mb-2", "New Components" }
            p { class: "text-muted-foreground mb-12 max-w-2xl text-lg",
                "Recently added components to the shadcn-dioxus library."
            }

            {section_heading("Calendar")}
            Calendar { default_year: 2025, default_month: 5 }

            {section_heading("Command")}
            Command { class: "w-[500px] rounded-lg border shadow-md",
                CommandInput { placeholder: "Type a command or search..." }
                CommandList { max_height: 300,
                    CommandGroup { heading: "Suggestions",
                        CommandItem { value: "calendar".to_string(),
                            "Calendar"
                            CommandShortcut { "⌘K" }
                        }
                        CommandItem { value: "search-files".to_string(),
                            "Search Files"
                            CommandShortcut { "⌘P" }
                        }
                        CommandItem { value: "settings".to_string(),
                            "Settings"
                            CommandShortcut { "⌘," }
                        }
                    }
                    CommandSeparator {}
                    CommandGroup { heading: "Actions",
                        CommandItem { value: "new-file".to_string(),
                            "New File"
                            CommandShortcut { "⌘N" }
                        }
                        CommandItem { value: "new-team".to_string(),
                            "New Team"
                        }
                        CommandItem { value: "new-project".to_string(),
                            "New Project"
                        }
                    }
                    CommandSeparator {}
                    CommandGroup { heading: "Navigation",
                        CommandItem { value: "go-home".to_string(),
                            "Go to Home"
                            CommandShortcut { "⌘H" }
                        }
                        CommandItem { value: "go-inbox".to_string(),
                            "Go to Inbox"
                            CommandShortcut { "⌘I" }
                        }
                        CommandItem { value: "go-notifications".to_string(),
                            "Go to Notifications"
                            CommandShortcut { "⌘N" }
                        }
                    }
                    CommandSeparator {}
                    CommandGroup { heading: "Help",
                        CommandItem { value: "documentation".to_string(),
                            "Documentation"
                            CommandShortcut { "F1" }
                        }
                        CommandItem { value: "api-reference".to_string(),
                            "API Reference"
                        }
                        CommandItem { value: "support".to_string(),
                            disabled: true,
                            "Support"
                        }
                    }
                    CommandEmpty { "No commands found." }
                }
            }

            {section_heading("Date Picker")}
            DatePicker { placeholder: "Pick a date" }

            {section_heading("Stepper")}
            Stepper { default_step: 0,
                StepperItem { step: 0,
                    StepperTitle { "Account" }
                    StepperDescription { "Create your account to get started." }
                }
                StepperSeparator {}
                StepperItem { step: 1,
                    StepperTitle { "Profile" }
                    StepperDescription { "Set up your profile information." }
                }
                StepperSeparator {}
                StepperItem { step: 2,
                    StepperTitle { "Review" }
                    StepperDescription { "Review your information before submitting." }
                }
                StepperFooter {
                    StepperPrevious {}
                    StepperIndicator {}
                    StepperNext {}
                }
            }

            {section_heading("Coming Soon")}
            div { class: "grid grid-cols-1 gap-6 sm:grid-cols-2",
                Card {
                    CardHeader {
                        CardTitle { "Data Table" }
                        CardDescription { "Full-featured table with sorting, filtering, and row selection. (module compilation pending)" }
                    }
                    CardContent {
                        p { class: "text-muted-foreground text-sm",
                            "Requires fixes to the data_table module before it can be publicly exported."
                        }
                    }
                }
                Card {
                    CardHeader {
                        CardTitle { "ComboBox" }
                        CardDescription { "Selectable combobox input with search filtering. (module compilation pending)" }
                    }
                    CardContent {
                        p { class: "text-muted-foreground text-sm",
                            "Requires fixes to the combobox module before it can be publicly exported."
                        }
                    }
                }
            }
        }
    }
}

fn section_heading(title: &'static str) -> Element {
    rsx! {
        div { class: "mb-12",
            h2 { class: "text-2xl font-semibold tracking-tight mb-6 border-b border-border pb-2", "{title}" }
        }
    }
}
