use leptos::prelude::*;
use web_sys::MouseEvent;

#[derive(Clone, Debug)]
pub enum ToolbarAction {
    Bold,
    Italic,
    Underline,
    Code,
    Link,
    List,
    Quote,
    Undo,
    Redo,
    Find,
    Replace,
    FullScreen,
}

#[component]
pub fn EditorToolbar(
    /// Whether to show the toolbar
    #[prop(default = false)]
    show_toolbar: bool,
    /// Callback for toolbar actions
    #[prop(default = Callback::new(|_| {}))]
    on_action: Callback<ToolbarAction>,
    /// Whether the editor is in full screen mode
    #[prop(default = false)]
    is_fullscreen: bool,
) -> impl IntoView {
    let handle_action = move |action: ToolbarAction| {
        move |_: MouseEvent| {
            on_action.run(action.clone());
        }
    };

    view! {
        {move || {
            if show_toolbar {
                view! {
                    <div class="border-b border-gray-200 p-2 flex items-center gap-1 flex-wrap">
                        // Formatting buttons
                        <div class="flex items-center gap-1 border-r border-gray-200 pr-2 mr-2">
                            <button
                                type="button"
                                class="p-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded font-bold"
                                title="Bold"
                                on:click=handle_action(ToolbarAction::Bold)
                            >
                                "𝐁"
                            </button>
                            <button
                                type="button"
                                class="p-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded italic"
                                title="Italic"
                                on:click=handle_action(ToolbarAction::Italic)
                            >
                                "𝐼"
                            </button>
                            <button
                                type="button"
                                class="p-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded font-mono"
                                title="Code"
                                on:click=handle_action(ToolbarAction::Code)
                            >
                                "</>"
                            </button>
                        </div>

                        // List buttons
                        <div class="flex items-center gap-1 border-r border-gray-200 pr-2 mr-2">
                            <button
                                type="button"
                                class="p-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded"
                                title="List"
                                on:click=handle_action(ToolbarAction::List)
                            >
                                "• "
                            </button>
                            <button
                                type="button"
                                class="p-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded"
                                title="Quote"
                                on:click=handle_action(ToolbarAction::Quote)
                            >
                                "❝"
                            </button>
                        </div>

                        // Undo/Redo buttons
                        <div class="flex items-center gap-1 border-r border-gray-200 pr-2 mr-2">
                            <button
                                type="button"
                                class="p-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded"
                                title="Undo"
                                on:click=handle_action(ToolbarAction::Undo)
                            >
                                "↶"
                            </button>
                            <button
                                type="button"
                                class="p-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded"
                                title="Redo"
                                on:click=handle_action(ToolbarAction::Redo)
                            >
                                "↷"
                            </button>
                        </div>

                        // Find/Replace buttons
                        <div class="flex items-center gap-1 border-r border-gray-200 pr-2 mr-2">
                            <button
                                type="button"
                                class="p-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded"
                                title="Find"
                                on:click=handle_action(ToolbarAction::Find)
                            >
                                "🔍"
                            </button>
                        </div>

                        // Fullscreen toggle
                        <div class="ml-auto">
                            <button
                                type="button"
                                class="p-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded"
                                title=if is_fullscreen { "Exit Fullscreen" } else { "Fullscreen" }
                                on:click=handle_action(ToolbarAction::FullScreen)
                            >
                                {if is_fullscreen {
                                    "⤌"
                                } else {
                                    "⛶"
                                }}
                            </button>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }
        }}
    }
}
