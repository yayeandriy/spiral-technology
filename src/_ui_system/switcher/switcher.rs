use js_sys::Math::sign;
use leptos::{logging, prelude::*};
use web_sys::MouseEvent;


pub fn Switcher<T: Send + Sync + Clone + PartialEq + ToString + 'static, F: Fn(T) + Clone + 'static>(
    items: Vec<T>,
    selected_item: Signal<T>,    
    mut on_click: F
) -> impl IntoView {
    view! {
        <div class="rounded-md p-2 bg-slate-700 w-full flex flex-col gap-[2px]">
            { 
                items
                .into_iter()
                .map(move |item| {
                    let item_inner = item.clone();
                    let item_class = item_inner.clone();
                    let item_onclick = item_inner.clone();
                    let item_onclick_copy = item_inner.clone();
                    let on_click = on_click.clone();
                    
                    // let is_item_selected = move || v == item_inner;
                    view! {
                        <div class="p-1 px-2 rounded cursor-pointer tracking-wider "
                            // class:hover:bg-slate-900= move || selected_item() != Some(item_inner)
                            class:bg-indigo-600= move || selected_item.get() == item_onclick
                            class:hover:bg-slate-900= move || selected_item.get() != item_onclick_copy      

                            on:click=move |_| {
                                on_click(item_class.clone())
                            }
                        >
                            {item_onclick.to_string()}
                        </div>
                    }
                })
                .collect_view()
            }
            
        </div>
    }
}
