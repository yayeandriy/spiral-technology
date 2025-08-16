#[derive(Clone, Debug)]
pub enum EditorMode {
    Create,
    Update,
}

#[derive(Clone, Debug)]
pub enum EditorStatus {
    Idle,
    Saving,
    Saved,
    HasUnsavedChanges,
}
