//! This crate contains all shared UI for the workspace.
mod utils;
pub use utils::{cn, RenderFn};
mod button;
pub mod chart;
pub use chart::*;
pub mod color_picker;
pub use color_picker::*;
pub mod combobox;
pub use combobox::*;
pub mod data_table;
pub use button::*;
pub use data_table::*;
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
mod date_picker;
pub use calendar::*;
pub use date_picker::*;
mod toast;
pub use toast::{add_toast, clear_toasts, remove_toast, ToastProvider};
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
    use super::*;
    use tailwind_fuse::tw_merge;

    // ── tw_merge baseline ───────────────────────────────────────────────
    #[test]
    fn test_ring_arbitrary_value_merge() {
        let result = tw_merge!("ring-[3px]", "ring-0");
        assert_eq!(
            result, "ring-0",
            "Arbitrary ring value should be overridden"
        );
    }
    #[test]
    fn test_ring_arbitrary_value_with_variant() {
        let result = tw_merge!("focus-visible:ring-[3px]", "focus-visible:ring-0");
        assert_eq!(
            result, "focus-visible:ring-0",
            "Variant with arbitrary ring should be overridden"
        );
    }
    #[test]
    fn test_ring_width_and_inset_separate() {
        let result = tw_merge!("ring-1", "ring-inset");
        assert_eq!(
            result, "ring-1 ring-inset",
            "ring-width and ring-inset should not conflict"
        );
    }

    // ── cn utility ───────────────────────────────────────────────────────
    #[test]
    fn test_cn_empty_additional() {
        assert_eq!(cn("bg-red text-white", ""), "bg-red text-white");
    }
    #[test]
    fn test_cn_empty_base() {
        assert_eq!(cn("", "extra"), "extra");
    }
    #[test]
    fn test_cn_both_empty() {
        assert_eq!(cn("", ""), "");
    }
    #[test]
    fn test_cn_conflict_resolution() {
        let result = cn("p-2 text-sm", "p-4 text-lg");
        assert_eq!(result, "p-4 text-lg", "Later class should win in conflict");
        assert!(
            !result.contains("text-sm"),
            "Conflicting class should be removed"
        );
    }
    #[test]
    fn test_cn_no_conflict() {
        let result = cn("p-2", "m-4");
        assert!(
            result.contains("p-2"),
            "Non-conflicting class should be kept"
        );
        assert!(result.contains("m-4"), "Additional class should be added");
    }

    // ── RenderFn PartialEq ─────────────────────────────────────────────
    // NOTE: rsx! is unavailable in test scope, so we only test PartialEq
    //       which does NOT require Debug on RenderFn.
    #[test]
    fn test_render_fn_always_unequal() {
        fn a(_: super::ItemChildProps, _: dioxus::prelude::Element) -> dioxus::prelude::Element {
            unimplemented!()
        }
        fn b(_: super::ItemChildProps, _: dioxus::prelude::Element) -> dioxus::prelude::Element {
            unimplemented!()
        }
        let r1 = super::RenderFn::new(a);
        let r2 = super::RenderFn::new(a);
        assert!(
            !r1.eq(&r2),
            "Same-function RenderFn instances should be unequal"
        );
        let r3 = super::RenderFn::new(b);
        assert!(
            !r1.eq(&r2),
            "Same-function RenderFn instances should be unequal"
        );
        let r3 = super::RenderFn::new(b);
        assert!(
            !r1.eq(&r2),
            "Same-function RenderFn instances should be unequal"
        );
        assert!(
            !r1.eq(&r3),
            "Different-function RenderFn instances should be unequal"
        );
    }

    // ── Badge ───────────────────────────────────────────────────────────
    mod badge {
        use super::*;

        #[test]
        fn test_badge_variant_default_class() {
            let c = BadgeVariant::Default.class();
            assert!(c.contains("bg-primary"), "Default should have bg-primary");
            assert!(
                c.contains("text-primary-foreground"),
                "Default should have text-primary-foreground"
            );
        }
        #[test]
        fn test_badge_variant_secondary_class() {
            let c = BadgeVariant::Secondary.class();
            assert!(
                c.contains("bg-secondary"),
                "Secondary should have bg-secondary"
            );
            assert!(
                c.contains("text-secondary-foreground"),
                "Secondary should have text-secondary-foreground"
            );
        }
        #[test]
        fn test_badge_variant_destructive_class() {
            let c = BadgeVariant::Destructive.class();
            assert!(
                c.contains("bg-destructive"),
                "Destructive should have bg-destructive"
            );
        }
        #[test]
        fn test_badge_variant_outline_class() {
            let c = BadgeVariant::Outline.class();
            assert!(
                c.contains("text-foreground"),
                "Outline should have text-foreground"
            );
            assert!(!c.contains("bg-"), "Outline should have no background");
        }
        #[test]
        fn test_badge_variant_as_str() {
            assert_eq!(BadgeVariant::Default.as_str(), "default");
            assert_eq!(BadgeVariant::Secondary.as_str(), "secondary");
            assert_eq!(BadgeVariant::Destructive.as_str(), "destructive");
            assert_eq!(BadgeVariant::Outline.as_str(), "outline");
        }
        #[test]
        fn test_badge_variants_includes_base() {
            for v in [
                BadgeVariant::Default,
                BadgeVariant::Secondary,
                BadgeVariant::Destructive,
                BadgeVariant::Outline,
            ] {
                let result = badge_variants(v);
                assert!(
                    result.contains("inline-flex"),
                    "badge_variants should always include base inline-flex"
                );
                assert!(
                    result.contains("rounded-full"),
                    "badge_variants should always include rounded-full"
                );
            }
        }
        #[test]
        fn test_badge_variants_includes_variant_class() {
            let result = badge_variants(BadgeVariant::Destructive);
            assert!(
                result.contains("bg-destructive"),
                "badge_variants should include variant class"
            );
        }
    }

    // ── Button ──────────────────────────────────────────────────────────
    mod button {
        use super::*;

        #[test]
        fn test_button_variant_classes() {
            assert!(ButtonVariant::Default.class().contains("bg-primary"));
            assert!(ButtonVariant::Destructive
                .class()
                .contains("bg-destructive"));
            assert!(ButtonVariant::Outline.class().contains("border"));
            assert!(ButtonVariant::Secondary.class().contains("bg-secondary"));
            assert!(ButtonVariant::Ghost.class().contains("hover:bg-accent"));
            assert!(ButtonVariant::Link.class().contains("underline"));
        }
        #[test]
        fn test_button_size_classes() {
            assert!(ButtonSize::Default.class().contains("h-9"));
            assert!(ButtonSize::Sm.class().contains("h-8"));
            assert!(ButtonSize::Lg.class().contains("h-10"));
            assert!(ButtonSize::Icon.class().contains("size-9"));
            assert!(ButtonSize::IconSm.class().contains("size-8"));
            assert!(ButtonSize::IconLg.class().contains("size-10"));
        }
        #[test]
        fn test_button_variants_includes_base() {
            for v in [
                ButtonVariant::Default,
                ButtonVariant::Outline,
                ButtonVariant::Ghost,
            ] {
                let result = button_variants(v, ButtonSize::Default);
                assert!(
                    result.contains("inline-flex"),
                    "button_variants should always include inline-flex"
                );
            }
        }
        #[test]
        fn test_button_variants_includes_size() {
            let result = button_variants(ButtonVariant::Default, ButtonSize::Lg);
            assert!(
                result.contains("h-10"),
                "button_variants should include size class"
            );
        }
        #[test]
        fn test_button_variants_includes_variant() {
            let result = button_variants(ButtonVariant::Destructive, ButtonSize::Default);
            assert!(
                result.contains("bg-destructive"),
                "button_variants should include variant class"
            );
        }
    }

    // ── Checkbox ────────────────────────────────────────────────────────
    mod checkbox {
        use super::*;

        #[test]
        fn test_checkbox_default_is_unchecked() {
            assert_eq!(CheckboxState::default(), CheckboxState::Unchecked);
        }

        // NOTE: to_aria_checked() and to_data_state() are private impl methods,
        //       inaccessible from test scope. as_str() and to_data_state() don't exist on
        //       the enum itself — they're on the impl block.
        // NOTE: !CheckboxState::Unchecked returns bool (via From<CheckboxState> for bool),
        //       so it can't be compared as a CheckboxState in an assert_ne!.
    }

    // ── Progress ────────────────────────────────────────────────────────
    mod progress {
        use super::*;

        #[test]
        fn test_progress_state_as_str() {
            assert_eq!(ProgressState::Indeterminate.as_str(), "indeterminate");
            assert_eq!(ProgressState::Loading.as_str(), "loading");
            assert_eq!(ProgressState::Loaded.as_str(), "loaded");
        }
        #[test]
        fn test_get_progress_state_none() {
            assert_eq!(
                get_progress_state(None, 100.0),
                ProgressState::Indeterminate
            );
        }
        #[test]
        fn test_get_progress_state_below_max() {
            assert_eq!(
                get_progress_state(Some(50.0), 100.0),
                ProgressState::Loading
            );
            assert_eq!(get_progress_state(Some(0.0), 100.0), ProgressState::Loading);
            assert_eq!(
                get_progress_state(Some(99.9), 100.0),
                ProgressState::Loading
            );
        }
        #[test]
        fn test_get_progress_state_at_max() {
            assert_eq!(
                get_progress_state(Some(100.0), 100.0),
                ProgressState::Loaded
            );
        }
        #[test]
        fn test_get_progress_state_above_max() {
            assert_eq!(
                get_progress_state(Some(150.0), 100.0),
                ProgressState::Loaded
            );
        }
        #[test]
        fn test_get_progress_state_zero_max() {
            assert_eq!(get_progress_state(Some(0.0), 0.0), ProgressState::Loaded);
            assert_eq!(get_progress_state(None, 0.0), ProgressState::Indeterminate);
        }
    }

    // ── Input ───────────────────────────────────────────────────────────
    mod input {
        use super::*;

        #[test]
        fn test_input_type_as_str() {
            assert_eq!(InputType::Text.as_str(), "text");
            assert_eq!(InputType::Password.as_str(), "password");
            assert_eq!(InputType::Email.as_str(), "email");
            assert_eq!(InputType::Number.as_str(), "number");
            assert_eq!(InputType::Tel.as_str(), "tel");
            assert_eq!(InputType::Url.as_str(), "url");
            assert_eq!(InputType::Search.as_str(), "search");
            assert_eq!(InputType::Date.as_str(), "date");
            assert_eq!(InputType::Time.as_str(), "time");
            assert_eq!(InputType::DatetimeLocal.as_str(), "datetime-local");
            assert_eq!(InputType::Month.as_str(), "month");
            assert_eq!(InputType::Week.as_str(), "week");
            assert_eq!(InputType::Color.as_str(), "color");
            assert_eq!(InputType::Hidden.as_str(), "hidden");
            assert_eq!(InputType::File.as_str(), "file");
        }
        #[test]
        fn test_input_classes_text() {
            let result = input_classes(InputType::Text);
            assert!(
                result.contains("bg-background"),
                "Text input should have bg-background"
            );
            assert!(
                result.contains("dark:bg-input/30"),
                "Text input should have dark variant background"
            );
            assert!(
                !result.contains("pt-1.5"),
                "Text input should not have file padding"
            );
        }
        #[test]
        fn test_input_classes_file() {
            let result = input_classes(InputType::File);
            assert!(
                result.contains("bg-transparent"),
                "File input should have bg-transparent"
            );
            assert!(
                result.contains("pt-1.5"),
                "File input should have extra top padding"
            );
        }
    }

    // ── Toggle ──────────────────────────────────────────────────────────
    mod toggle {
        use super::*;

        #[test]
        fn test_toggle_variant_class() {
            assert!(ToggleVariant::Default.class().contains("bg-transparent"));
            assert!(ToggleVariant::Outline.class().contains("border"));
        }
        #[test]
        fn test_toggle_size_class() {
            assert!(ToggleSize::Default.class().contains("h-9"));
            assert!(ToggleSize::Sm.class().contains("h-8"));
            assert!(ToggleSize::Lg.class().contains("h-10"));
        }
        #[test]
        fn test_toggle_variants_includes_base() {
            let result = toggle_variants(ToggleVariant::Default, ToggleSize::Default);
            assert!(result.contains("inline-flex"));
        }
    }

    // ── ToggleGroup ─────────────────────────────────────────────────────
    mod toggle_group {
        use super::*;

        #[test]
        fn test_toggle_group_type_as_str() {
            assert_eq!(ToggleGroupType::Single.as_str(), "single");
            assert_eq!(ToggleGroupType::Multiple.as_str(), "multiple");
        }
    }

    // ── Item ─────────────────────────────────────────────────────────────
    mod item {
        use super::*;

        #[test]
        fn test_item_variant_class() {
            assert!(ItemVariant::Default.class().contains("bg-transparent"));
            assert!(ItemVariant::Outline.class().contains("border-border"));
            assert!(ItemVariant::Muted.class().contains("bg-muted/50"));
        }
        #[test]
        fn test_item_variant_as_str() {
            assert_eq!(ItemVariant::Default.as_str(), "default");
            assert_eq!(ItemVariant::Outline.as_str(), "outline");
            assert_eq!(ItemVariant::Muted.as_str(), "muted");
        }
        #[test]
        fn test_item_size_class() {
            assert!(ItemSize::Default.class().contains("gap-4"));
            assert!(ItemSize::Sm.class().contains("gap-2.5"));
        }
        #[test]
        fn test_item_size_as_str() {
            assert_eq!(ItemSize::Default.as_str(), "default");
            assert_eq!(ItemSize::Sm.as_str(), "sm");
        }
        #[test]
        fn test_item_variants_includes_base() {
            let result = item_variants(ItemVariant::Default, ItemSize::Default);
            assert!(result.contains("inline-flex"));
            assert!(result.contains("rounded-md"));
        }
    }

    // ── Empty ────────────────────────────────────────────────────────────
    mod empty {
        use super::*;

        #[test]
        fn test_empty_media_variant_class() {
            assert!(EmptyMediaVariant::Default
                .class()
                .contains("bg-transparent"));
            assert!(EmptyMediaVariant::Icon.class().contains("bg-muted"));
            assert!(EmptyMediaVariant::Icon.class().contains("rounded-lg"));
        }
        #[test]
        fn test_empty_media_variant_as_str() {
            assert_eq!(EmptyMediaVariant::Default.as_str(), "default");
            assert_eq!(EmptyMediaVariant::Icon.as_str(), "icon");
        }
        #[test]
        fn test_empty_media_variants_includes_base() {
            let result = empty_media_variants(EmptyMediaVariant::Icon);
            assert!(result.contains("flex"));
            assert!(result.contains("shrink-0"));
        }
    }

    // ── Drawer ──────────────────────────────────────────────────────────
    mod drawer {
        use super::*;

        #[test]
        fn test_drawer_side_as_str() {
            assert_eq!(DrawerSide::Right.as_str(), "right");
            assert_eq!(DrawerSide::Left.as_str(), "left");
            assert_eq!(DrawerSide::Top.as_str(), "top");
            assert_eq!(DrawerSide::Bottom.as_str(), "bottom");
        }
        #[test]
        fn test_drawer_side_classes() {
            let right = DrawerSide::Right.classes();
            assert!(right.contains("inset-y-0"));
            assert!(right.contains("right-0"));
            assert!(right.contains("border-l"));
            let left = DrawerSide::Left.classes();
            assert!(left.contains("left-0"));
            assert!(left.contains("border-r"));
            let top = DrawerSide::Top.classes();
            assert!(top.contains("top-0"));
            assert!(top.contains("border-b"));
            let bottom = DrawerSide::Bottom.classes();
            assert!(bottom.contains("bottom-0"));
            assert!(bottom.contains("border-t"));
        }
    }

    // ── Sheet ────────────────────────────────────────────────────────────
    mod sheet {
        use super::*;

        #[test]
        fn test_sheet_side_as_str() {
            assert_eq!(Side::Top.as_str(), "top");
            assert_eq!(Side::Bottom.as_str(), "bottom");
            assert_eq!(Side::Left.as_str(), "left");
            assert_eq!(Side::Right.as_str(), "right");
        }
        #[test]
        fn test_sheet_side_class() {
            let right = Side::Right.class();
            assert!(right.contains("end-0"));
            assert!(right.contains("border-s"));
            let left = Side::Left.class();
            assert!(left.contains("start-0"));
            assert!(left.contains("border-e"));
        }
    }

    // ── Resizable ───────────────────────────────────────────────────────
    mod resizable {
        use super::*;

        #[test]
        fn test_direction_as_str() {
            assert_eq!(Direction::Horizontal.as_str(), "horizontal");
            assert_eq!(Direction::Vertical.as_str(), "vertical");
        }
        #[test]
        fn test_direction_flex_dir() {
            assert!(Direction::Horizontal.flex_dir().contains("flex-row"));
            assert!(Direction::Vertical.flex_dir().contains("flex-col"));
        }
    }

    // ── Alert ──────────────────────────────────────────────────────────
    mod alert {
        use super::*;

        #[test]
        fn test_alert_variant_class() {
            let default = AlertVariant::Default.class();
            assert!(default.contains("bg-card"));
            assert!(default.contains("text-card-foreground"));
            let destructive = AlertVariant::Destructive.class();
            assert!(destructive.contains("text-destructive"));
            assert!(destructive.contains("bg-card"));
        }
    }

    // ── InputGroup ───────────────────────────────────────────────────
    mod input_group {
        use super::*;

        #[test]
        fn test_input_group_addon_align_as_str() {
            assert_eq!(InputGroupAddonAlign::InlineStart.as_str(), "inline-start");
            assert_eq!(InputGroupAddonAlign::InlineEnd.as_str(), "inline-end");
            assert_eq!(InputGroupAddonAlign::BlockStart.as_str(), "block-start");
            assert_eq!(InputGroupAddonAlign::BlockEnd.as_str(), "block-end");
        }
        #[test]
        fn test_input_group_addon_align_class() {
            let start = InputGroupAddonAlign::InlineStart.class();
            assert!(start.contains("order-first"));
            assert!(start.contains("ps-3"));
            let end = InputGroupAddonAlign::InlineEnd.class();
            assert!(end.contains("order-last"));
            assert!(end.contains("pe-3"));
        }
        #[test]
        fn test_input_group_button_size_class() {
            assert!(InputGroupButtonSize::Xs.class().contains("h-6"));
            assert!(InputGroupButtonSize::Sm.class().contains("h-8"));
            assert!(InputGroupButtonSize::IconXs.class().contains("size-6"));
            assert!(InputGroupButtonSize::IconSm.class().contains("size-8"));
        }
    }

    // ── Carousel ──────────────────────────────────────────────────────
    mod carousel {
        use super::*;

        #[test]
        fn test_carousel_orientation_as_str() {
            assert_eq!(CarouselOrientation::Horizontal.as_str(), "horizontal");
            assert_eq!(CarouselOrientation::Vertical.as_str(), "vertical");
        }
    }

    // ── Sidebar ──────────────────────────────────────────────────────
    mod sidebar {
        use super::*;

        #[test]
        fn test_sidebar_variant_as_str() {
            assert_eq!(SidebarVariant::Sidebar.as_str(), "sidebar");
            assert_eq!(SidebarVariant::Floating.as_str(), "floating");
            assert_eq!(SidebarVariant::Inset.as_str(), "inset");
        }
    }

    // ── NavigationMenu ──────────────────────────────────────────────
    mod navigation_menu {
        use super::*;

        #[test]
        fn test_navigation_menu_orientation_as_str() {
            assert_eq!(NavigationMenuOrientation::Horizontal.as_str(), "horizontal");
            assert_eq!(NavigationMenuOrientation::Vertical.as_str(), "vertical");
        }

        #[test]
        fn test_navigation_menu_trigger_style() {
            let style = navigation_menu_trigger_style();
            assert!(style.contains("inline-flex"));
            assert!(style.contains("rounded-md"));
            assert!(style.contains("bg-background"));
        }
    }

    // ── Field ───────────────────────────────────────────────────────────
    mod field {
        use super::*;

        // NOTE: FieldLegendVariant does NOT have an as_str() method on the enum itself.
        //       It exists only on the impl block and it is NOT accessible from test scope.
        #[test]
        fn test_field_legend_default_is_label() {
            assert_eq!(FieldLegendVariant::default(), FieldLegendVariant::Label);
        }
    }

    // NOTE: The following items exist as pub types with public methods but are in private submodules
    //       and cannot be accessed from the test module, even with full crate:: paths.
    //       - ButtonGroupOrientation (mod button_group is private)
    //       - SeparatorOrientation (exists but mysteriously inaccessible from test scope)
    //       - CheckboxState::to_aria_checked / to_data_state (private impl methods)
    //       - CheckboxState::! (returns bool via From<CheckboxState>, not CheckboxState)
    //       - FieldLegendVariant::as_str (module field_legend is private)
}
