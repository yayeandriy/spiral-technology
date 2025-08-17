use std::collections::HashSet;

use leptos::{prelude::*, reactive::spawn_local};

#[derive(Clone, Debug, PartialEq)]
pub enum AutoSaveStatus {
    Idle,
    Pending,
    Saving,
    Saved,
    Error(String),
}


#[component]
pub fn ProjectFormNotifications(
    status: ReadSignal<AutoSaveStatus>,
) -> impl IntoView {
    let status = move || status.get().clone();
    
    view! {
            // Autosave status indicator
            {
               
                    move || match status() {
                        AutoSaveStatus::Idle => view! { <div></div> }.into_any(),
                        AutoSaveStatus::Pending => view! {
                            <div class="flex items-center justify-end mb-2 text-sm">
                                <span class="text-yellow-600 flex items-center">
                                  
                                    "Changes pending..."
                                </span>
                            </div>
                        }.into_any(),
                        AutoSaveStatus::Saving => view! {
                            <div class="flex items-center justify-end mb-2 text-sm">
                                <span class="text-blue-600 flex items-center">
                                    
                                    "Saving..."
                                </span>
                            </div>
                        }.into_any(),
                        AutoSaveStatus::Saved => view! {
                            <div class="flex items-center justify-end mb-2 text-sm">
                                <span class="text-green-600 flex items-center">
                                    
                                    "Changes saved"
                                </span>
                            </div>
                        }.into_any(),
                        AutoSaveStatus::Error(err) => view! {
                            <div class="flex items-center justify-end mb-2 text-sm">
                                <span class="text-red-600 flex items-center">
                                   
                                    {format!("Save failed: {}", err)}
                                </span>
                            </div>
                        }.into_any(),
                    }
                    }
           
    }
}

