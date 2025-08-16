use leptos::{logging, prelude::*};


pub fn Selector<T: Send + Sync + Clone + PartialEq + ToString + 'static, F: Fn(T) + Clone + 'static>(
    options: impl Fn() -> Vec<T>,
    selected: Signal<Vec<T>>,    
    mut on_click: F
) -> impl IntoView {
    view! {
        <div class="rounded-md p-2 bg-slate-700  w-full flex flex-col gap-[2px]">
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
                        "p-1 px-2 rounded cursor-pointer tracking-wider bg-indigo-600 text-[9px] uppercase text-indigo-200 "
                    } else {
                         "p-1 px-2 rounded cursor-pointer tracking-wider text-[9px] uppercase text-cyan-500"
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





// #[component]
// pub fn SelectorOld(
//     #[prop(into)]
//     data: Vec<String>,
//     #[prop(into)]
//     selected_items: ReadSignal<Vec<String>>,
//     #[prop(into)]
//     selected_items_set: WriteSignal<Vec<String>>,
// ) -> impl IntoView {
//     // let selected_items = RwSignal::new(Vec::<String>::new());
//     let items = data;
//     let is_item_selected = move |item: &String| selected_items.get().contains(item);
//     // let setter = use_context::<WriteSignal<Vec<String>>>().expect("to have found the setter provided");
//     view! {
//         <div class="rounded-md p-2 bg-slate-700 w-full flex flex-col gap-[2px]">
//             { 
//                 items
//                 .into_iter()
//                 .map(move |item| {
//                     let item_inner = item.clone();
//                     let item_class = item_inner.clone();
//                     let item_onclick = item_inner.clone();
                    
//                     view! {
//                         <div class="p-1 px-2 rounded cursor-pointer tracking-wider"
//                             class:hover:bg-slate-900= move || !is_item_selected(&item_inner)
//                             class:bg-blue-600= move || is_item_selected(&item_class)
//                             class:opacity-40= move || selected_items.get().is_empty()
//                             on:click=move |_| {
//                                 let current_item = item_onclick.to_string();
//                                 selected_items_set.update(|s| {
//                                     if s.contains(&current_item){
//                                         s.retain(|x| x != &current_item);
//                                     } else {
//                                         s.push(current_item);
//                                     }
//                                 });
//                                 logging::log!("SELECTED ITMES: {:?}", selected_items.get())
//                             }
//                         >
//                             {item_inner.to_string()}
//                         </div>
//                     }
//                 })
//                 .collect::<Vec<_>>()
//             }
//         </div>
//     }
// }


// #[component]
// pub fn FilterSwitcher<T>(
//     #[prop(into)]
//     options: Vec<T>,
//     #[prop(optional)]
//     label: &'static str,
//     #[prop(into)]
//     selected: RwSignal<Vec<T>>,
// ) -> impl IntoView
// where
//     T: 'static + Clone + PartialEq + Sync + Send + std::fmt::Debug,
//     Vec<T>: With + Read + Track + ReadUntracked,
// {
//     view! {
//         <div class="filter-switcher">
//             <p>{label}</p>
//             <div class="options flex gap-2">
//                 {
//                     options.iter().map(move |option| {
//                         let option_clone = option.clone();
//                         let option_clone_2 = option.clone();
//                         let option_clone_3 = option.clone();
//                         let option_clone_4 = option.clone();
//                         view! {
//                             <div class="p-1 px-2 rounded cursor-pointer border"
//                                 class:bg-blue-600=move || selected.get().contains(&option_clone)
//                                 on:click=move |_| {
//                                     selected.update(|s| {
//                                         if s.contains(&option_clone_2) {
//                                             s.retain(|x| x != &option_clone_4);
//                                         } else {
//                                             s.push(option_clone_4.clone());
//                                         }
//                                     })
//                                 }
//                             >
//                                 {format!("{:?}", option_clone_3)}
//                             </div>
//                         }
//                     }).collect::<Vec<_>>()
//                 }
//             </div>
//         </div>
//     }
// }