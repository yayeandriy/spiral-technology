use leptos::prelude::*;
use crate::projects::model::Project;
use crate::ui::button::{CancelButton, PrimaryButton};
use crate::projects::projects_context::use_project;

/// Working signal-based project form that fixes all compilation issues
#[component]
pub fn WorkingProjectForm() -> impl IntoView {
    let project_context = use_project();
    
    // Form signals
    let (title, set_title) = signal(String::new());
    let (desc, set_desc) = signal(String::new());
    
    // Form validation
    let is_valid = move || !title.get().trim().is_empty();
    let is_submitting = project_context.is_loading.0;

    let handle_save = move |_| {
        if is_valid() && !is_submitting.get() {
            let new_project = Project {
                id: 0, // This would be set by the backend
                title: title.get(),
                desc: if desc.get().trim().is_empty() { None } else { Some(desc.get()) },
                created_at: None,
            };
            
            leptos::logging::log!("Saving project: {:?}", new_project);
            // Here you would call the actual save function
        }
    };

    let handle_cancel = move |_| {
        set_title.set(String::new());
        set_desc.set(String::new());
        leptos::logging::log!("Form cancelled");
    };

    view! {
        <div class="max-w-2xl mx-auto bg-white p-6 rounded-lg shadow-lg">
            <h2 class="text-2xl font-bold text-gray-900 mb-6">
                "Create New Project"
            </h2>

            <div class="space-y-6">
                <div>
                    <label for="title" class="block text-sm font-medium text-gray-700 mb-2">
                        "Project Title"
                    </label>
                    <input
                        id="title"
                        type="text"
                        class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                        placeholder="Enter project title..."
                        value=move || title.get()
                        on:input=move |ev| set_title.set(event_target_value(&ev))
                        required
                    />
                </div>

                <div>
                    <label for="description" class="block text-sm font-medium text-gray-700 mb-2">
                        "Description"
                    </label>
                    <textarea
                        id="description"
                        class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                        rows="4"
                        prop:value=move || desc.get()
                        placeholder="Enter project description..."
                        on:input=move |ev| set_desc.set(event_target_value(&ev))
                    ></textarea>
                </div>
            </div>

            <div class="flex justify-end space-x-3 mt-8">
                <CancelButton
                    on_click=handle_cancel
                    disabled=is_submitting.into()
                >
                    "Cancel"
                </CancelButton>
                
                <PrimaryButton
                    on_click=handle_save
                    disabled=Signal::derive(move || !is_valid() || is_submitting.get())
                >
                    {move || if is_submitting.get() { "Saving..." } else { "Save Project" }}
                </PrimaryButton>
            </div>
        </div>
    }
}
