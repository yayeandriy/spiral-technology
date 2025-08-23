use leptos::{prelude::*};


pub fn Select<T: Send + Sync + Clone + PartialEq + ToString + 'static, F: Fn(T) + Clone + 'static>(
    options: impl Fn() -> Vec<T>,
    selected: Signal<Vec<T>>,    
    on_click: F
) -> impl IntoView {
    view! {
        <div class="rounded-[6px] p-2 bg-gray-100 w-full flex text-black flex-col gap-[2px] ">
            { 
                options()
                .into_iter()
                .map(move |item| {
                    let item_inner = item.clone();
                    let item_class = item_inner.clone();
                    let item_onclick = item_inner.clone();
                    let on_click = on_click.clone();
                    let item_selected = move || selected.get().contains(&item_inner);
                    let class = move || if item_selected() {
                        "p-1 px-2 rounded-[6px] cursor-pointer tracking-wider bg-black text-white text-[10px] uppercase  "
                    } else {
                         "p-1 px-2 rounded-[6px] cursor-pointer tracking-wider text-[10px] uppercase text-black"
                    };
                    view! {
                        <div class=class()
                            
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


