use dioxus::prelude::*;
use super::toast::{ToastData, ToastVariant};
use std::sync::atomic::{AtomicUsize, Ordering};

static TOAST_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

static mut TOASTS: Option<Vec<Signal<ToastData>>> = None;

pub fn get_toasts() -> &'static Vec<Signal<ToastData>> {
    unsafe {
        if TOASTS.is_none() {
            TOASTS = Some(Vec::new());
        }
        TOASTS.as_ref().unwrap()
    }
}

pub fn get_toasts_mut() -> &'static mut Vec<Signal<ToastData>> {
    unsafe {
        if TOASTS.is_none() {
            TOASTS = Some(Vec::new());
        }
        TOASTS.as_mut().unwrap()
    }
}

/// Add a toast notification
pub fn add_toast(title: String, description: Option<String>, variant: ToastVariant, duration: u64) {
    let id = TOAST_ID_COUNTER.fetch_add(1, Ordering::Relaxed);

    let signal = Signal::new(ToastData {
        id,
        title,
        description,
        variant,
        action: None,
        duration,
    });

    get_toasts_mut().push(signal);
}

/// Remove a toast by ID
pub fn remove_toast(id: usize) {
    let toasts = get_toasts_mut();
    toasts.retain(|t| t.with(|data| data.id != id));
}

/// Clear all toasts
pub fn clear_toasts() {
    get_toasts_mut().clear();
}

#[derive(Clone, PartialEq, Props)]
pub struct ToastProviderProps {
    pub children: Element,
}

#[component]
pub fn ToastProvider(props: ToastProviderProps) -> Element {
    use_hook(|| {
        let _ = get_toasts();
    });

    rsx! {
        {props.children}
        super::toast_viewport::ToastViewport {}
    }
}
