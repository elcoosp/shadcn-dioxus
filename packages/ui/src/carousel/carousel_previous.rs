use crate::cn;
use crate::carousel::CarouselContext;
use dioxus::prelude::*;
use lucide_dioxus::ChevronLeft;

const BUTTON_BASE: &str = "absolute z-10 inline-flex h-8 w-8 items-center justify-center rounded-full border border-input bg-background/80 shadow-sm transition-colors hover:bg-accent hover:text-accent-foreground disabled:pointer-events-none disabled:opacity-50";

#[derive(Clone, PartialEq, Props)]
pub struct CarouselPreviousProps {
    #[props(into, default)]
    pub class: String,
    #[props(default = false)]
    pub disabled: bool,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn CarouselPrevious(props: CarouselPreviousProps) -> Element {
    let ctx = use_context::<CarouselContext>();
    let current_index = ctx.current_index;
    let set_index = ctx.set_index;
    let total = ctx.total;

    let is_first = use_memo(move || current_index() == 0);
    let is_disabled = props.disabled || is_first() || total() == 0;

    let position_class = match ctx.orientation {
        crate::carousel::CarouselOrientation::Horizontal => "left-2 top-1/2 -translate-y-1/2",
        crate::carousel::CarouselOrientation::Vertical => "left-1/2 top-2 -translate-x-1/2 rotate-90",
    };

    let classes = cn(&format!("{} {}", BUTTON_BASE, position_class), &props.class);

    rsx! {
        button {
            r#type: "button",
            "data-slot": "carousel-previous",
            disabled: is_disabled,
            class: "{classes}",
            onclick: move |_| {
                let t = total();
                if t > 0 {
                    let prev = if current_index() == 0 { t - 1 } else { current_index() - 1 };
                    set_index.call(prev);
                }
            },
            ..props.attributes,
            ChevronLeft { class: "h-4 w-4" }
            span { class: "sr-only", "Previous slide" }
        }
    }
}
