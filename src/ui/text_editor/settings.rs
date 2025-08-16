use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct EditorConfig {
    pub auto_save: bool,
    pub auto_save_delay_ms: u32,
    pub show_line_numbers: bool,
    pub font_size: u16,
    pub theme: EditorTheme,
    pub word_wrap: bool,
    pub spell_check: bool,
}

#[derive(Clone, Debug)]
pub enum EditorTheme {
    Light,
    Dark,
    Auto,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            auto_save: false,
            auto_save_delay_ms: 2000,
            show_line_numbers: false,
            font_size: 14,
            theme: EditorTheme::Light,
            word_wrap: true,
            spell_check: false,
        }
    }
}

#[component]
pub fn EditorSettings(
    /// Editor configuration
    #[prop(into)]
    config: RwSignal<EditorConfig>,
    /// Whether to show the settings panel
    #[prop(default = false)]
    show_settings: bool,
) -> impl IntoView {
    view! {
        {move || {
            if show_settings {
                view! {
                    <div class="border-t border-gray-200 pt-4 mt-4">
                        <details class="group">
                            <summary class="cursor-pointer text-sm font-medium text-gray-700 hover:text-gray-900">
                                "Editor Settings"
                            </summary>
                            <div class="mt-3 space-y-3 text-sm">
                                // Auto-save toggle
                                <label class="flex items-center space-x-2">
                                    <input
                                        type="checkbox"
                                        class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                        prop:checked=move || config.get().auto_save
                                        on:change=move |ev| {
                                            let checked = event_target_checked(&ev);
                                            config.update(|c| c.auto_save = checked);
                                        }
                                    />
                                    <span>"Auto-save"</span>
                                </label>

                                // Word wrap toggle
                                <label class="flex items-center space-x-2">
                                    <input
                                        type="checkbox"
                                        class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                        prop:checked=move || config.get().word_wrap
                                        on:change=move |ev| {
                                            let checked = event_target_checked(&ev);
                                            config.update(|c| c.word_wrap = checked);
                                        }
                                    />
                                    <span>"Word wrap"</span>
                                </label>

                                // Spell check toggle
                                <label class="flex items-center space-x-2">
                                    <input
                                        type="checkbox"
                                        class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                        prop:checked=move || config.get().spell_check
                                        on:change=move |ev| {
                                            let checked = event_target_checked(&ev);
                                            config.update(|c| c.spell_check = checked);
                                        }
                                    />
                                    <span>"Spell check"</span>
                                </label>

                                // Font size slider
                                <div class="space-y-1">
                                    <label class="block text-xs text-gray-600">"Font size"</label>
                                    <input
                                        type="range"
                                        min="10"
                                        max="24"
                                        class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
                                        prop:value=move || config.get().font_size.to_string()
                                        on:input=move |ev| {
                                            if let Ok(size) = event_target_value(&ev).parse::<u16>() {
                                                config.update(|c| c.font_size = size);
                                            }
                                        }
                                    />
                                    <div class="text-xs text-gray-500">
                                        {move || format!("{}px", config.get().font_size)}
                                    </div>
                                </div>
                            </div>
                        </details>
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }
        }}
    }
}
