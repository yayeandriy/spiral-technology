use leptos::prelude::*;

use crate::areas::model::ProjectArea;
use crate::areas::views::{area_form::AreaForm, areas_list::AreasList};

#[component]
pub fn AreasEditor() -> impl IntoView {
    // State for editing
    let (editing_area, set_editing_area) = signal::<Option<ProjectArea>>(None);
    
    // Handle edit
    let handle_edit = Callback::new(move |area: ProjectArea| {
        set_editing_area.set(Some(area));
    });
    
    // Handle form success (clear editing state)
    let on_form_success = Callback::new(move |_: ()| {
        set_editing_area.set(None);
    });

    view! {
        <div class="flex flex-col">
                // Form Column
                {move || {
                    let current_editing = editing_area.get();
                    view! {
                        <AreaForm 
                            editing_area=current_editing
                            on_success=on_form_success
                        />
                    }
                }}                                
                // List Column
                <AreasList on_edit=handle_edit />
        </div>
    }
}
