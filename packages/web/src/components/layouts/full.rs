use crate::components::Hero;
use dioxus::prelude::*;
use ui::{Button, ButtonVariant};

#[derive(Props, Clone, PartialEq)]
pub struct FullLayoutProps {
    children: Element,
}

#[component]
pub fn FullLayout(props: FullLayoutProps) -> Element {
    rsx! {
        Hero {}
        div {
            class: "container-wrapper scroll-mt-24",
            id: "examples",
            div {
                class: "container mx-auto flex items-center justify-between gap-2 sm:gap-4 py-3 sm:py-4 px-4",
                div {
                    class: "flex items-center [&>a:first-child]:text-primary flex-1 overflow-x-auto",
                    div {
                        class: "flex items-center space-x-1 sm:space-x-2",
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ui::ButtonSize::Sm,
                            href: "/",
                            "Examples"
                        }
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ui::ButtonSize::Sm,
                            disabled: true,
                            href: "/examples/dashboard",
                            "Dashboard"
                        }
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ui::ButtonSize::Sm,
                            disabled: true,
                            href: "/examples/tasks",
                            "Tasks"
                        }
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ui::ButtonSize::Sm,
                            href: "/examples/playground",
                            "Playground"
                        }
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ui::ButtonSize::Sm,
                            disabled: true,
                            href: "/examples/authentication",
                            "Authentication"
                        }
                    }
                }
            }
        }
        {props.children}
    }
}
