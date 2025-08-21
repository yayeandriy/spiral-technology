use leptos::prelude::*;

use crate::{areas::{model::ProjectArea, views::area_form::AreaForm}, ui::button::{ButtonSize, SecondaryButton}};


#[component]
pub fn AreaEditor(
    area:impl Fn() -> Option<ProjectArea> + Clone + Copy + Send + 'static,
    category: String,
) -> impl IntoView {
    // If an area is provided, automatically open the form for editing
    let open_form = signal(area().is_some());
    let area = signal(area());
    let category_clone = category.clone();
    let open_area_editor = move || {
        area.1.set(None);
       open_form.1.set(true);
    };
    let area_clone = area.clone();
    // let area_clone = move || {
    //     if let Some(area) = area_clone {
    //         Some(area)
    //     } else {
    //         None
    //     }
    // };
    view! {
        <div class="text-sm w-full flex-col">
            <div>
            {
                move || if let Some(area) = area_clone.0.get() {
                    
                    view! {
                        <h2 class="text-lg font-semibold mb-4">"Edit Area" {area.title}</h2>
                    }.into_any()
                } else {
                    view! {
                        <h2 class="text-lg font-semibold mb-4">"Create New Area"</h2>
                    }.into_any()    
                }
            }
            </div>
            {
                move || {
                    let area = area.clone();
                    let category = category_clone.clone();
                    let open_area_editor = open_area_editor.clone();
                    if open_form.0.get() {
                        if let Some(area) = area.0.get() {
                            view! {
                                <AreaForm area = signal(Some(area)).0 category=category.clone() is_open=open_form.1 />
                            }.into_any()
                        }else{
                            view! {
                                <AreaForm area = signal(None).0 category=category.clone() is_open=open_form.1 />
                            }.into_any()
                        }
                    } else {
                        view! {
                            <div class="w-full flex justify-end">
                            <SecondaryButton
                                on_click=move |_| {
                                    open_area_editor();
                                }
                                size=ButtonSize::Small
                            >
                                "➕"
                            </SecondaryButton>
                            </div>
                        }.into_any()
                    }
                }
            }
        </div>
    }
}

