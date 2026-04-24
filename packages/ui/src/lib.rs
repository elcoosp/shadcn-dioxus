//! This crate contains all shared UI for the workspace.
mod utils;
pub use utils::{cn, RenderFn};
mod button;
pub use button::*;
pub mod button_group;
pub use button_group as ButtonGroup;
pub mod spinner;
pub use spinner::*;
pub mod separator;
pub use separator::Separator;
pub mod item;
pub use item::*;
pub mod empty;
pub use empty::*;
pub mod card;
pub use card::*;
pub mod avatar;
pub use avatar::*;
pub mod skeleton;
pub use skeleton::*;
pub mod badge;
pub use badge::*;
pub mod input;
pub use input::*;
pub mod label;
pub use label::*;
pub mod progress;
pub use progress::*;
pub mod kbd;
pub use kbd::*;
pub mod checkbox;
pub use checkbox::*;
pub mod textarea;
pub use textarea::*;
pub mod alert;
pub use alert::*;
pub mod switch;
pub use switch::*;
pub mod field;
pub use field::*;
pub mod toggle;
pub use toggle::*;
pub mod aspect_ratio;
pub use aspect_ratio::*;
pub mod native_select;
pub use native_select::*;
pub mod input_group;
pub use input_group::*;
pub mod dialog;
pub use dialog::*;
pub mod portal;
pub use portal::*;
pub mod sheet;
pub use sheet::*;
mod navbar;
pub use navbar::*;
mod hero;
pub use hero::*;
mod popover;
pub use popover::*;
mod tooltip;
pub use tooltip::*;
mod dropdown_menu;
pub use dropdown_menu::*;
mod context_menu;
pub use context_menu::*;
mod hover_card;
pub use hover_card::*;
mod navigation_menu;
pub use navigation_menu::*;
mod menubar;
pub use menubar::*;
mod tabs;
pub use tabs::*;
mod collapsible;
pub use collapsible::*;
mod accordion;
pub use accordion::*;
mod toggle_group;
pub use toggle_group::*;
mod radio_group;
pub use radio_group::*;
mod table;
pub use table::*;
mod breadcrumb;
pub use breadcrumb::*;
mod alert_dialog;
pub use alert_dialog::*;
mod scroll_area;
pub use scroll_area::*;
mod select;
pub use select::*;
mod slider;
pub use slider::*;
mod input_otp;
pub use input_otp::*;
mod resizable;
pub use resizable::*;
mod pagination;
pub use pagination::*;
mod drawer;
pub use drawer::*;
mod calendar;
pub use calendar::*;
mod toast;
pub use toast::{ToastProvider, add_toast, remove_toast, clear_toasts};
mod command;
pub use command::*;
mod carousel;
pub use carousel::*;
mod sidebar;
pub use sidebar::*;
mod stepper;
pub use stepper::*;

#[cfg(test)]
mod tests {
    use tailwind_fuse::tw_merge;
    #[test]
    fn test_ring_arbitrary_value_merge() {
        let result = tw_merge!("ring-[3px]", "ring-0");
        assert_eq!(result, "ring-0", "Arbitrary ring value should be overridden");
    }
    #[test]
    fn test_ring_arbitrary_value_with_variant() {
        let result = tw_merge!("focus-visible:ring-[3px]", "focus-visible:ring-0");
        assert_eq!(result, "focus-visible:ring-0", "Variant with arbitrary ring should be overridden");
    }
    #[test]
    fn test_ring_width_and_inset_separate() {
        let result = tw_merge!("ring-1", "ring-inset");
        assert_eq!(result, "ring-1 ring-inset", "ring-width and ring-inset should not conflict");
    }
}
