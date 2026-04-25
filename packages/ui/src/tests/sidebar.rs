use crate::SidebarVariant;

#[test]
fn test_sidebar_variant_as_str() {
    assert_eq!(SidebarVariant::Sidebar.as_str(), "sidebar");
    assert_eq!(SidebarVariant::Floating.as_str(), "floating");
    assert_eq!(SidebarVariant::Inset.as_str(), "inset");
}
