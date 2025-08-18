use leptos::prelude::*;

use crate::{areas::model::ProjectArea, projects::views::project_edit_page::area_form::AreaForm, ui::signal_button::{ButtonSize, SSecondaryButton}};


#[component]
pub fn AreaEditor(
     #[prop(optional)]
    area: Option<ProjectArea>,
    category: String,
) -> impl IntoView {
    // If an area is provided, automatically open the form for editing
    let open_form = signal(area.is_some());
    let category_clone = category.clone();
    let open_area_editor = move || {
       open_form.1.set(true);
    };

    view! {
        <div class="text-sm w-full flex-col">
            {
                move || {
                    let area = area.clone();
                    let category = category_clone.clone();
                    let open_area_editor = open_area_editor.clone();
                    if open_form.0.get() {
                        if let Some(area) = area {  
                            view! {
                                <AreaForm area = area category=category.clone()  />
                            }.into_any()
                        }else{
                            view! {
                                <AreaForm category=category.clone()  />
                            }.into_any()
                        }
                    } else {
                        view! {
                            <div class="w-full flex justify-end">
                            <SSecondaryButton
                                on_click=move |_| {
                                    open_area_editor();
                                }
                                size=ButtonSize::Small
                            >
                                "➕"
                            </SSecondaryButton>
                            </div>
                        }.into_any()
                    }
                }
            }
        </div>
    }
}

