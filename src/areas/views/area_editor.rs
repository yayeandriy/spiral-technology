use leptos::{ logging, prelude::*};

use crate::{areas::{model::ProjectArea, views::area_form::AreaForm}, ui::button::{ButtonSize, SecondaryButton}};


#[component]
pub fn AreaEditor(
    area:impl Fn() -> Option<ProjectArea> + Clone + Copy + Send + 'static,
    category: String,
    open_form: (ReadSignal<bool>, WriteSignal<bool>),
) -> impl IntoView {
    // If an area is provided, automatically open the form for editing
    let init_area = area();
    // let open_form = signal(false);
    let area = signal(area());
    let category_clone = category.clone();
    let open_area_editor = move || {
        area.1.set(None);
       open_form.1.set(true);
    };
    let area_clone = area.clone();
   
   Effect::new(move || {
       let is_open = open_form.0.get();
       logging::log!("AreaEditor is_open: {}", is_open);
    });

    view! {
        <div class="text-sm w-full flex-col">            
            {
                move || {
                    let area = area.clone();
                    let category = category_clone.clone();
                    let open_area_editor = open_area_editor.clone();
                    move || if open_form.0.get() {
                        if let Some(area) = area.0.get() {
                            view! {
                                <AreaForm area=area category=category.clone() on_close=move |is_open| {
                                    logging::log!("Updated AreaForm closed: {}", is_open);
                                    open_form.1.set(is_open);
                                } />
                            }.into_any()
                        }else{
                            view! {
                                <AreaForm category=category.clone() on_close=move |is_open| {
                                    open_form.1.set(is_open);
                                }   />
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

