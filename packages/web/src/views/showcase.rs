use dioxus::prelude::*;
use lucide_dioxus::Search;
use ui::*;

#[component]
pub fn Showcase() -> Element {
    rsx! {
        div { class: "container mx-auto py-12 px-4 max-w-5xl",
            h1 { class: "text-4xl font-bold tracking-tight mb-2", "Component Showcase" }
            p { class: "text-muted-foreground mb-12 max-w-2xl text-lg",
                "Visual reference for every component in the shadcn-dioxus library."
            }
            {section_heading("Buttons & Actions")}
            div { class: "flex flex-wrap items-center gap-3",
                Button { "Default" }
                Button { variant: ButtonVariant::Secondary, "Secondary" }
                Button { variant: ButtonVariant::Destructive, "Destructive" }
                Button { variant: ButtonVariant::Outline, "Outline" }
                Button { variant: ButtonVariant::Ghost, "Ghost" }
                Button { variant: ButtonVariant::Link, "Link" }
            }
            div { class: "flex flex-wrap items-center gap-2",
                Button { size: ButtonSize::Sm, "Small" }
                Button { "Default" }
                Button { size: ButtonSize::Lg, "Large" }
                Button { size: ButtonSize::Icon, Button { variant: ButtonVariant::Outline, "..." } }
            }

            {section_heading("Badges")}
            div { class: "flex flex-wrap items-center gap-2",
                Badge { "Default" }
                Badge { variant: BadgeVariant::Secondary, "Secondary" }
                Badge { variant: BadgeVariant::Destructive, "Destructive" }
                Badge { variant: BadgeVariant::Outline, "Outline" }
            }

            {section_heading("Inputs")}
            div { class: "grid grid-cols-1 gap-4 max-w-md",
                div { class: "space-y-2",
                    Label { "for": "s-input", "Input" }
                    Input { id: "s-input", placeholder: "Type here..." }
                }
                div { class: "space-y-2",
                    Label { "for": "s-textarea", "Textarea" }
                    Textarea { id: "s-textarea", placeholder: "Your message..." }
                }
                div { class: "flex items-center space-x-2",
                    Checkbox { default_checked: CheckboxState::Checked, id: "s-check" }
                    Label { "for": "s-check", "Accept terms" }
                }
                div { class: "flex items-center space-x-2",
                    Switch { id: "s-switch" }
                    Label { "for": "s-switch", "Dark mode" }
                }
                div { class: "space-y-2",
                    Label { "for": "s-select", "Native Select" }
                    NativeSelect { id: "s-select",
                        NativeSelectOption { value: "", "Pick a fruit" }
                        NativeSelectOption { value: "apple", "Apple" }
                        NativeSelectOption { value: "banana", "Banana" }
                        NativeSelectOption { value: "cherry", selected: true, "Cherry" }
                    }
                }
                div { class: "space-y-2",
                    Label { "for": "s-slider", "Slider" }
                    Slider { id: "s-slider", min: 0.0, max: 100.0, value: 42.0, class: "w-full" }
                }
                InputGroup {
                    InputGroupInput { placeholder: "Search..." }
                    InputGroupAddon { Search {} }
                }
            }

            {section_heading("Data Display")}
            Card { class: "w-[350px]",
                CardHeader {
                    CardTitle { "Card Title" }
                    CardDescription { "Card description goes here." }
                }
                CardContent { p { "Card content goes here." } }
                CardFooter { Button { "Action" } }
            }
            div { class: "flex items-center space-x-3",
                Avatar { AvatarImage { src: "https://github.com/shadcn.png", alt: "shadcn" } }
                AvatarFallback { "CN" }
            }
            div { class: "space-y-4 max-w-md",
                Progress { value: 60.0, class: "w-full" }
                Progress { class: "w-full" }
            }
            div { class: "space-y-2 max-w-md",
                Skeleton { class: "h-12 w-12 rounded-full" }
                div { class: "space-y-2",
                    Skeleton { class: "h-4 w-[250px]" }
                    Skeleton { class: "h-4 w-[200px]" }
                }
            }
            Spinner {}
            div { class: "my-6 h-px w-full bg-border" }
            AspectRatio { ratio: 16.0 / 9.0, class: "w-[300px]",
                div { class: "bg-muted rounded-md h-full w-full flex items-center justify-center text-sm", "16:9" }
            }
            div { class: "flex items-center gap-1",
                Kbd { "Ctrl" }
                Kbd { "K" }
            }
            Table {
                TableHeader { TableRow { TableHead { "Name" } TableHead { "Email" } TableHead { "Role" } } }
                TableBody {
                    TableRow { TableCell { "Alice" } TableCell { "alice@example.com" } TableCell { "Admin" } }
                    TableRow { TableCell { "Bob" } TableCell { "bob@example.com" } TableCell { "User" } }
                }
            }
            ItemGroup {
                Item {
                    ItemMedia { Avatar { AvatarFallback { "S" } } }
                    ItemContent {
                        ItemTitle { "Component Name" }
                        ItemDescription { "Description here." }
                    }
                    ItemActions { Button { size: ButtonSize::Icon, variant: ButtonVariant::Ghost, "..." } }
                }
                ItemSeparator {}
                Item {
                    ItemMedia { Avatar { AvatarFallback { "B" } } }
                    ItemContent { ItemTitle { "Another Item" } }
                }
            }
            Empty {
                EmptyHeader {
                    EmptyTitle { "No items found" }
                    EmptyDescription { "Try adjusting your filters." }
                }
                EmptyContent { Button { "Clear filters" } }
            }
            Field {
                FieldLabel { "for": "s-field", "Email" }
                Input { id: "s-field", placeholder: "Enter email" }
                FieldDescription { "We'll never share your email." }
            }

            {section_heading("Navigation")}
            Tabs { default_value: "tab1".to_string(),
                TabsList {
                    TabsTrigger { value: "tab1".to_string(), "Account" }
                    TabsTrigger { value: "tab2".to_string(), "Password" }
                }
                TabsContent { value: "tab1".to_string(), p { "Account settings here." } }
                TabsContent { value: "tab2".to_string(), p { "Password settings here." } }
            }
            Accordion {
                AccordionItem { value: "item-1".to_string(),
                    AccordionTrigger { "Is it accessible?" }
                    AccordionContent { "Yes. It follows WAI-ARIA guidelines." }
                }
                AccordionItem { value: "item-2".to_string(),
                    AccordionTrigger { "Is it styled?" }
                    AccordionContent { "Yes. It uses Tailwind CSS classes." }
                }
            }
            Collapsible { default_open: true,
                CollapsibleTrigger { "Click to toggle" }
                CollapsibleContent { "This content is collapsible." }
            }
            Breadcrumb {
                BreadcrumbList {
                    BreadcrumbItem { BreadcrumbLink { "Home" } }
                    BreadcrumbSeparator {}
                    BreadcrumbItem { BreadcrumbLink { "Products" } }
                    BreadcrumbSeparator {}
                    BreadcrumbItem { BreadcrumbPage { "Current" } }
                }
            }
            Pagination { page: 1, total_pages: 10,
                PaginationContent {
                    PaginationPrevious {}
                    PaginationItem { active: true, "1" }
                    PaginationEllipsis {}
                    PaginationItem { "10" }
                    PaginationEllipsis {}
                    PaginationNext {}
                }
            }
            Calendar { default_year: 2025, default_month: 6 }
            DatePicker { placeholder: "Pick a date" }

            {section_heading("Overlays")}
            Dialog {
                DialogTrigger { Button { variant: ButtonVariant::Outline, "Open Dialog" } }
                DialogPortal {
                    DialogOverlay {}
                    DialogContent { class: "sm:max-w-[425px]",
                        DialogHeader {
                            DialogTitle { "Edit Profile" }
                            DialogDescription { "Make changes to your profile here." }
                        }
                        div { class: "grid gap-4 py-4",
                            div { class: "grid grid-cols-4 items-center gap-4",
                                Label { "for": "s-name", class: "text-end", "Name" }
                                Input { id: "s-name", value: "Pedro Duarte", class: "col-span-3" }
                            }
                        }
                        DialogFooter {
                            Button { "Save changes" }
                        }
                    }
                }
            }
            Sheet {
                SheetTrigger { Button { variant: ButtonVariant::Outline, "Open Sheet" } }
                SheetPortal {
                    SheetOverlay {}
                    SheetContent { side: Side::Right,
                        SheetHeader {
                            SheetTitle { "Edit Profile" }
                            SheetDescription { "Make changes to your profile here." }
                        }
                        div { class: "grid gap-4 py-4",
                            div { class: "grid grid-cols-4 items-center gap-4",
                                Label { "for": "s-name2", class: "text-end", "Name" }
                                Input { id: "s-name2", value: "Pedro Duarte", class: "col-span-3" }
                            }
                        }
                    }
                }
            }
            Drawer {
                DrawerTrigger { Button { variant: ButtonVariant::Outline, "Open Drawer" } }
                DrawerOverlay {}
                DrawerContent { side: DrawerSide::Right,
                    DrawerHeader { DrawerTitle { "Drawer Title" } }
                    DrawerDescription { "Drawer description." }
                    DrawerFooter { "Footer content" }
                }
            }
            AlertDialog {
                AlertDialogTrigger { Button { variant: ButtonVariant::Destructive, "Delete Account" } }
                AlertDialogContent {
                    AlertDialogHeader { AlertDialogTitle { "Are you sure?" } }
                    AlertDialogDescription { "This action cannot be undone." }
                    AlertDialogFooter {
                        AlertDialogCancel { Button { variant: ButtonVariant::Outline, "Cancel" } }
                        AlertDialogAction { Button { variant: ButtonVariant::Destructive, "Delete" } }
                    }
                }
            }
            Popover {
                PopoverTrigger { Button { variant: ButtonVariant::Outline, "Open Popover" } }
                PopoverContent { "Popover content here." }
            }
            Tooltip {
                TooltipTrigger { button { class: "underline", "Hover me" } }
                TooltipContent { "Tooltip content here." }
            }
            HoverCard {
                HoverCardTrigger { a { href: "#", class: "underline", "Hover for details" } }
                HoverCardContent { "Additional details here." }
            }
            DropdownMenu {
                DropdownMenuTrigger { Button { variant: ButtonVariant::Outline, "Open Menu" } }
                DropdownMenuContent {
                    DropdownMenuItem { "Profile" }
                    DropdownMenuSeparator {}
                    DropdownMenuLabel { "Actions" }
                    DropdownMenuShortcut { "Ctrl+K" }
                    DropdownMenuItem { "Settings" }
                }
            }
            ContextMenu {
                ContextMenuTrigger {
                    div { class: "p-8 border rounded-md text-center cursor-context-menu", "Right-click me" }
                }
                ContextMenuContent {
                    ContextMenuItem { "Copy" }
                    ContextMenuSeparator {}
                    ContextMenuItem { "Paste" }
                    ContextMenuSeparator {}
ContextMenuItem { disabled: true, "Delete" }
                }
            }

            {section_heading("Layout")}
            ScrollArea { class: "h-32 w-64 border rounded-md p-4",
                div { class: "space-y-2",
                    p { "Scrollable content line 1" }
                    p { "Scrollable content line 2" }
                    p { "Scrollable content line 3" }
                    p { "Scrollable content line 4" }
                    p { "Scrollable content line 5" }
                    p { "Scrollable content line 6" }
                    p { "Scrollable content line 7" }
                    p { "Scrollable content line 8" }
                }
            }
            SidebarProvider { default_open: true,
                Sidebar { collapsible: true,
                    SidebarHeader { "My App" }
                    SidebarContent {
                        SidebarGroup { default_open: true,
                            SidebarGroupLabel { "Navigation" }
                            SidebarGroupContent {
                                SidebarMenu {
                                    SidebarMenuItem { "Dashboard" }
                                    SidebarMenuItem { "Settings" }
                                    SidebarMenuItem { "Documentation" }
                                }
                            }
                        }
                    }
                    SidebarFooter { "2025 shadcn-dioxus" }
                }
                div { class: "flex-1 p-8",
                    p { class: "text-muted-foreground", "Main content area" }
                }
            }

            {section_heading("Feedback")}
            Alert { variant: AlertVariant::Default,
                AlertTitle { "Heads up!" }
                AlertDescription { "You can add components to your app using the CLI." }
            }
            Alert { variant: AlertVariant::Destructive,
                AlertTitle { "Error" }
                AlertDescription { "Your session has expired. Please log in again." }
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
