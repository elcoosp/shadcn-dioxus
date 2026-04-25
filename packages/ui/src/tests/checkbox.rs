use crate::CheckboxState;

#[test]
fn test_checkbox_default_is_unchecked() {
    assert_eq!(CheckboxState::default(), CheckboxState::Unchecked);
}
