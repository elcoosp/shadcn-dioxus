use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct CarouselContext {
    pub current_index: Signal<usize>,
    pub total: Signal<usize>,
    pub set_index: Callback<usize>,
    pub orientation: CarouselOrientation,
    pub auto_play: bool,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum CarouselOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl CarouselOrientation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct CarouselProps {
    #[props(default)]
    pub orientation: CarouselOrientation,
    #[props(default = false)]
    pub auto_play: bool,
    #[props(default = 3000)]
    pub auto_play_interval_ms: u64,
    /// Total number of slides
    pub total: usize,
    /// Override default height for vertical orientation (default: 20rem)
    #[props(default = "20rem".to_string())]
    pub height: String,
    #[props(into, default)]
    pub class: String,
    pub children: Element,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Carousel(props: CarouselProps) -> Element {
    let mut current_index = use_signal(|| 0);
    let total = use_signal(|| props.total);
    let auto_play = props.auto_play;
    let auto_play_interval_ms = props.auto_play_interval_ms;

    let set_index = use_callback(move |idx: usize| {
        if total() > 0 {
            current_index.set(idx % total());
        }
    });

    use_context_provider(|| CarouselContext {
        current_index,
        total,
        set_index,
        orientation: props.orientation,
        auto_play,
    });

    // Autoplay cleanup
    let mut cancel_autoplay = use_signal(|| false);
    use_drop(move || cancel_autoplay.set(true));

    use_effect(move || {
        if !auto_play { return; }
        let cancel = cancel_autoplay.clone();
        spawn(async move {
            loop {
                futures_timer::Delay::new(std::time::Duration::from_millis(auto_play_interval_ms)).await;
                if cancel() { break; }
                let t = total();
                if t > 0 {
                    let next = (current_index() + 1) % t;
                    current_index.set(next);
                }
            }
        });
    });

    let orientation_class = match props.orientation {
        CarouselOrientation::Horizontal => "relative overflow-hidden".to_string(),
        CarouselOrientation::Vertical => format!("relative overflow-hidden flex flex-col h-[{}]", props.height),
    };

    rsx! {
        div {
            "data-slot": "carousel",
            "data-orientation": props.orientation.as_str(),
            role: "region",
            "aria-roledescription": "carousel",
            class: "{orientation_class} {props.class}",
            ..props.attributes,
            {props.children}
        }
    }
}
