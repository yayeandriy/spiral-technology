use std::sync::Arc;

use leptos::{logging, prelude::*, reactive::spawn_local};

use crate::{areas::{areas_context::{self, use_areas}, model::{ProjectArea, ProjectAreaDto}}, catalog::{self, catalog_context::{self, use_catalog}}, projects::{model::Project, views::project_edit_page::area_form::AreaForm}, ui::{s_selector::s_selector::SSelector, signal_button::{ButtonSize, SSecondaryButton}}};


#[component]
pub fn AreaEditor(
     #[prop(optional)]
    area: Option<ProjectArea>,
    category: String,
) -> impl IntoView {
    let open_form = signal(false);
    let category_clone = category.clone();
    let open_area_editor = move || {
       open_form.1.set(true);
    };


    view! {
        <div class="text-sm">
            {
                move || {
                    let area = area.clone();
                    let category = category_clone.clone();
                    let open_area_editor = open_area_editor.clone();
                    if open_form.0.get() {
                        // let area_signal = signal(area);
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
                            <SSecondaryButton
                                on_click=move |_| {
                                    open_area_editor();
                                }
                                size=ButtonSize::Small
                            >
                                "Add Area"
                            </SSecondaryButton>
                        }.into_any()
                    }
                }
            }
            
        </div>
                           
    }
}

