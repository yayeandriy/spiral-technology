use leptos::prelude::*;

#[component]
pub fn ContentInfo(
    /// Whether to show content info section
    #[prop(default = false)]
    show_content_info: bool,
    /// Content info to display (e.g., ID, created date)
    #[prop(default = Signal::derive(|| vec![]))]
    content_info: Signal<Vec<(String, String)>>,
) -> impl IntoView {
    view! {
        {move || {
            if show_content_info {
                let info_items = content_info.get();
                if !info_items.is_empty() {
                    view! {
                        <div class="pt-4 border-t border-gray-200">
                            <div class="text-xs text-gray-500 space-y-1">
                                {info_items.into_iter().map(|(label, value)| {
                                    view! {
                                        <div>{label}: " " {value}</div>
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            } else {
                view! {}.into_any()
            }
        }}
    }
}
