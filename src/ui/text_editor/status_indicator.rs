use leptos::prelude::*;
use super::types::EditorStatus;

#[component]
pub fn StatusIndicator(
    /// Current status of the editor
    #[prop(into)]
    status: Signal<EditorStatus>,
    /// Whether there are unsaved changes
    #[prop(into)]
    has_unsaved_changes: Signal<bool>,
) -> impl IntoView {
    // Determine the current effective status
    let effective_status = Signal::derive({
        let status = status.clone();
        let has_unsaved_changes = has_unsaved_changes.clone();
        move || {
            match status.get() {
                EditorStatus::Saving => EditorStatus::Saving,
                EditorStatus::Saved => {
                    if has_unsaved_changes.get() {
                        EditorStatus::HasUnsavedChanges
                    } else {
                        EditorStatus::Saved
                    }
                },
                EditorStatus::Idle => {
                    if has_unsaved_changes.get() {
                        EditorStatus::HasUnsavedChanges
                    } else {
                        EditorStatus::Idle
                    }
                },
                EditorStatus::HasUnsavedChanges => EditorStatus::HasUnsavedChanges,
            }
        }
    });

    view! {
        <div class="flex items-center gap-2">
            {move || {
                match effective_status.get() {
                    EditorStatus::Saving => view! {
                        <div class="flex items-center gap-2 text-sm text-gray-500">
                            <div class="w-4 h-4 border-2 border-blue-600 border-t-transparent rounded-full animate-spin"></div>
                            "Saving..."
                        </div>
                    }.into_any(),
                    EditorStatus::HasUnsavedChanges => view! {
                        <div class="flex items-center gap-1 text-sm text-orange-600">
                            <div class="w-2 h-2 bg-orange-600 rounded-full"></div>
                            "Unsaved changes"
                        </div>
                    }.into_any(),
                    EditorStatus::Saved => view! {
                        <div class="flex items-center gap-1 text-sm text-green-600">
                            <div class="w-2 h-2 bg-green-600 rounded-full"></div>
                            "Saved"
                        </div>
                    }.into_any(),
                    EditorStatus::Idle => view! {}.into_any(),
                }
            }}
        </div>
    }
}
