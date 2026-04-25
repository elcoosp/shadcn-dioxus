use crate::progress::{get_progress_state, ProgressState};

#[test]
fn test_progress_state_as_str() {
    assert_eq!(ProgressState::Indeterminate.as_str(), "indeterminate");
    assert_eq!(ProgressState::Loading.as_str(), "loading");
    assert_eq!(ProgressState::Loaded.as_str(), "loaded");
}

#[test]
fn test_get_progress_state_none() {
    assert_eq!(get_progress_state(None, 100.0), ProgressState::Indeterminate);
}

#[test]
fn test_get_progress_state_below_max() {
    assert_eq!(get_progress_state(Some(50.0), 100.0), ProgressState::Loading);
    assert_eq!(get_progress_state(Some(0.0), 100.0), ProgressState::Loading);
    assert_eq!(get_progress_state(Some(99.9), 100.0), ProgressState::Loading);
}

#[test]
fn test_get_progress_state_at_max() {
    assert_eq!(get_progress_state(Some(100.0), 100.0), ProgressState::Loaded);
}

#[test]
fn test_get_progress_state_above_max() {
    assert_eq!(get_progress_state(Some(150.0), 100.0), ProgressState::Loaded);
}

#[test]
fn test_get_progress_state_zero_max() {
    assert_eq!(get_progress_state(Some(0.0), 0.0), ProgressState::Loaded);
    assert_eq!(get_progress_state(None, 0.0), ProgressState::Indeterminate);
}
