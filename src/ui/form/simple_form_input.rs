use leptos::prelude::*;

#[component]
pub fn SimpleFormInput(
    id: String,
    label: String,
    input_type: String,
    value: ReadSignal<String>,
    on_input: Callback<String>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] required: Option<bool>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let required = required.unwrap_or(false);
    let placeholder = placeholder.unwrap_or_default();
    let class = class.unwrap_or_default();

    view! {
        <div class=format!("space-y-2 {}", class)>
            <label for=id.clone() class="block text-sm font-medium text-gray-700">
                {label}
                {move || if required { " *" } else { "" }}
            </label>
            <input
                type=input_type
                id=id.clone()
                name=id.clone()
                value=move || value.get()
                on:input=move |e| {
                    let val = event_target_value(&e);
                    on_input.run(val);
                }
                placeholder=placeholder
                required=required
                class="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            />
        </div>
    }
}
