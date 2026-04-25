use crate::cn;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct TableCellProps {
    #[props(into, default)]
    pub class: String,
    /// HTML colspan attribute
    #[props(default = 1)]
    pub colspan: u32,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn TableCell(props: TableCellProps) -> Element {
    rsx! {
        td {
            "data-slot": "table-cell",
            colspan: "{props.colspan}",
            class: cn("p-2 align-middle [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]", &props.class),
            ..props.attributes,
            {props.children}
        }
    }
}
