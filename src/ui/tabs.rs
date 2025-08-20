use leptos::prelude::*;

#[component]
pub fn Tabs(
    children: ChildrenFragment,
    tabs_titles: Vec<String>,
) -> impl IntoView {
    let selected_tab_index = signal(0 as usize);
    let fragment = children();
    let tabs: Vec<_> = fragment.nodes.into_iter().enumerate().collect();
    let n_tabs = tabs.len();

    view! {
        <div class="text-sm">
            <div class="flex mb-1">
                {
                    (0..n_tabs).map(|index| {
                        let tab_title = if index < tabs_titles.len() {
                            tabs_titles[index].clone()
                        } else {
                            format!("Tab {}", index + 1)
                        };
                        let is_selected = Signal::derive(move || selected_tab_index.0.get() == index);
                        view! {
                            <div 
                                class=move || if is_selected.get() {
                                    "tab-selected px-2 py-1 bg-blue-500 border border-transparent text-white first:rounded-l-[6px] last:rounded-r-[6px] cursor-pointer"
                                } else {
                                    "tab cursor-pointer px-2 py-1 border border-gray-300 first:rounded-l-[6px] last:rounded-r-[6px] hover:bg-gray-100"
                                }
                                on:click=move |_| selected_tab_index.1.set(index)
                            >
                                {tab_title}
                            </div>
                        }
                    }).collect_view()
                }
            </div>
            <div class="tab-content">
                {
                    tabs.into_iter().map(|(index, tab)| {
                        view! {
                            <div 
                                class="tab-panel"
                                style:display=move || if selected_tab_index.0.get() == index { "block" } else { "none" }
                            >
                                {tab}
                            </div>
                        }
                    }).collect_view()
                }
            </div>
        </div>
    }
}