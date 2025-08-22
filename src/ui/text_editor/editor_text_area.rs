use leptos::prelude::*;
use web_sys::HtmlTextAreaElement;
use wasm_bindgen::JsCast;
use std::sync::Arc;

// Utility functions for text manipulation
pub fn insert_text_at_cursor(text: &str, insert: &str, cursor_pos: usize) -> (String, usize) {
    let mut result = String::new();
    result.push_str(&text[..cursor_pos]);
    result.push_str(insert);
    result.push_str(&text[cursor_pos..]);
    (result, cursor_pos + insert.len())
}

pub fn wrap_selection(text: &str, start: usize, end: usize, prefix: &str, suffix: &str) -> (String, usize, usize) {
    let mut result = String::new();
    result.push_str(&text[..start]);
    result.push_str(prefix);
    result.push_str(&text[start..end]);
    result.push_str(suffix);
    result.push_str(&text[end..]);
    
    let new_start = start + prefix.len();
    let new_end = end + prefix.len();
    (result, new_start, new_end)
}

pub fn insert_at_line_start(text: &str, cursor_pos: usize, prefix: &str) -> (String, usize) {
    let lines: Vec<&str> = text.lines().collect();
    let mut char_count = 0;
    let mut line_index = 0;
    
    // Find which line the cursor is on
    for (i, line) in lines.iter().enumerate() {
        if char_count + line.len() >= cursor_pos {
            line_index = i;
            break;
        }
        char_count += line.len() + 1; // +1 for newline
    }
    
    let mut result_lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
    if line_index < result_lines.len() {
        result_lines[line_index] = format!("{}{}", prefix, result_lines[line_index]);
    }
    
    let result = result_lines.join("\n");
    let new_cursor_pos = cursor_pos + prefix.len();
    (result, new_cursor_pos)
}

// Context for exposing markdown editing functions to parent components
#[derive(Clone)]
pub struct MarkdownEditor {
    pub apply_bold: Arc<dyn Fn() + Send + Sync>,
    pub apply_italic: Arc<dyn Fn() + Send + Sync>,
    pub apply_h1: Arc<dyn Fn() + Send + Sync>,
    pub apply_h2: Arc<dyn Fn() + Send + Sync>,
    pub insert_link: Arc<dyn Fn() + Send + Sync>,
    pub apply_quote: Arc<dyn Fn() + Send + Sync>,
    pub insert_image: Arc<dyn Fn() + Send + Sync>,
    pub undo: Arc<dyn Fn() + Send + Sync>,
}

#[component]
pub fn EditorTextArea(
    value: (ReadSignal<String>, WriteSignal<String>),
    children: Children
) -> impl IntoView {
    let textarea_ref = NodeRef::<leptos::html::Textarea>::new();
    
    // Store cursor position and selection
    let cursor_position = RwSignal::new(0usize);
    let selection_start = RwSignal::new(0usize);
    let selection_end = RwSignal::new(0usize);
    
    // Undo history - store up to 10 previous states
    let history = RwSignal::new(Vec::<(String, usize)>::new()); // (text, cursor_position)
    let history_index = RwSignal::new(0usize);
    
    // Add current state to history
    let add_to_history = {
        let value = value.clone();
        move |cursor_pos: usize| {
            let current_text = value.0.get();
            let mut hist = history.get();
            
            // Remove any future history if we're not at the end
            let current_index = history_index.get();
            if current_index < hist.len() {
                hist.truncate(current_index);
            }
            
            // Add new state
            hist.push((current_text, cursor_pos));
            
            // Keep only last 10 states
            if hist.len() > 10 {
                hist.remove(0);
            } else {
                history_index.set(history_index.get() + 1);
            }
            
            history.set(hist);
        }
    };
    
    // Initialize history with current state
    {
        let initial_text = value.0.get();
        let mut hist = Vec::new();
        hist.push((initial_text, 0));
        history.set(hist);
        history_index.set(1);
    }
    
    // Function to get current cursor position and selection
    let update_cursor_info = {
        let textarea_ref = textarea_ref.clone();
        move || {
            if let Some(textarea) = textarea_ref.get() {
                let element = textarea.unchecked_into::<HtmlTextAreaElement>();
                cursor_position.set(element.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize);
                selection_start.set(element.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize);
                selection_end.set(element.selection_end().unwrap_or(Some(0)).unwrap_or(0) as usize);
            }
        }
    };
    
    // Function to set cursor position
    let set_cursor_position = {
        let textarea_ref = textarea_ref.clone();
        move |pos: usize| {
            if let Some(textarea) = textarea_ref.get() {
                let element = textarea.unchecked_into::<HtmlTextAreaElement>();
                let _ = element.set_selection_range(pos as u32, pos as u32);
            }
        }
    };
    
    // Function to set selection
    let set_selection = {
        let textarea_ref = textarea_ref.clone();
        move |start: usize, end: usize| {
            if let Some(textarea) = textarea_ref.get() {
                let element = textarea.unchecked_into::<HtmlTextAreaElement>();
                let _ = element.set_selection_range(start as u32, end as u32);
            }
        }
    };
    
    // Create markdown editing functions and provide them as context
    let markdown_functions = MarkdownEditor {
        apply_bold: {
            let value = value.clone();
            let update_cursor_info = update_cursor_info.clone();
            let set_selection = set_selection.clone();
            let add_to_history = add_to_history.clone();
            Arc::new(move || {
                add_to_history(selection_start.get());
                update_cursor_info();
                let text = value.0.get();
                let start = selection_start.get();
                let end = selection_end.get();
                
                if start == end {
                    // No selection, insert bold placeholder
                    let (new_text, _) = insert_text_at_cursor(&text, "**bold text**", start);
                    value.1.set(new_text);
                    // Set selection to select "bold text"
                    set_selection(start + 2, start + 11);
                } else {
                    // Wrap selection in bold
                    let (new_text, new_start, new_end) = wrap_selection(&text, start, end, "**", "**");
                    value.1.set(new_text);
                    set_selection(new_start, new_end);
                }
            })
        },
        apply_italic: {
            let value = value.clone();
            let update_cursor_info = update_cursor_info.clone();
            let set_selection = set_selection.clone();
            let add_to_history = add_to_history.clone();
            Arc::new(move || {
                add_to_history(selection_start.get());
                update_cursor_info();
                let text = value.0.get();
                let start = selection_start.get();
                let end = selection_end.get();
                
                if start == end {
                    // No selection, insert italic placeholder
                    let (new_text, _) = insert_text_at_cursor(&text, "*italic text*", start);
                    value.1.set(new_text);
                    // Set selection to select "italic text"
                    set_selection(start + 1, start + 12);
                } else {
                    // Wrap selection in italic
                    let (new_text, new_start, new_end) = wrap_selection(&text, start, end, "*", "*");
                    value.1.set(new_text);
                    set_selection(new_start, new_end);
                }
            })
        },
        apply_h1: {
            let value = value.clone();
            let update_cursor_info = update_cursor_info.clone();
            let set_cursor_position = set_cursor_position.clone();
            let add_to_history = add_to_history.clone();
            Arc::new(move || {
                add_to_history(cursor_position.get());
                update_cursor_info();
                let text = value.0.get();
                let cursor = cursor_position.get();
                let (new_text, new_cursor) = insert_at_line_start(&text, cursor, "# ");
                value.1.set(new_text);
                set_cursor_position(new_cursor);
            })
        },
        apply_h2: {
            let value = value.clone();
            let update_cursor_info = update_cursor_info.clone();
            let set_cursor_position = set_cursor_position.clone();
            let add_to_history = add_to_history.clone();
            Arc::new(move || {
                add_to_history(cursor_position.get());
                update_cursor_info();
                let text = value.0.get();
                let cursor = cursor_position.get();
                let (new_text, new_cursor) = insert_at_line_start(&text, cursor, "## ");
                value.1.set(new_text);
                set_cursor_position(new_cursor);
            })
        },
        insert_link: {
            let value = value.clone();
            let update_cursor_info = update_cursor_info.clone();
            let set_selection = set_selection.clone();
            let add_to_history = add_to_history.clone();
            Arc::new(move || {
                add_to_history(selection_start.get());
                update_cursor_info();
                let text = value.0.get();
                let start = selection_start.get();
                let end = selection_end.get();
                
                if start == end {
                    // No selection, insert link placeholder
                    let (new_text, _) = insert_text_at_cursor(&text, "[link text](https://example.com)", start);
                    value.1.set(new_text);
                    // Select "link text"
                    set_selection(start + 1, start + 10);
                } else {
                    // Use selection as link text
                    let selected_text = &text[start..end];
                    let link_markdown = format!("[{}](https://example.com)", selected_text);
                    let (new_text, _, _) = wrap_selection(&text, start, end, "", "");
                    let (final_text, _) = insert_text_at_cursor(&new_text, &link_markdown, start);
                    value.1.set(final_text);
                    // Select the URL part
                    set_selection(start + selected_text.len() + 3, start + selected_text.len() + 22);
                }
            })
        },
        apply_quote: {
            let value = value.clone();
            let update_cursor_info = update_cursor_info.clone();
            let set_cursor_position = set_cursor_position.clone();
            let add_to_history = add_to_history.clone();
            Arc::new(move || {
                add_to_history(cursor_position.get());
                update_cursor_info();
                let text = value.0.get();
                let cursor = cursor_position.get();
                let (new_text, new_cursor) = insert_at_line_start(&text, cursor, "> ");
                value.1.set(new_text);
                set_cursor_position(new_cursor);
            })
        },
        insert_image: {
            let value = value.clone();
            let update_cursor_info = update_cursor_info.clone();
            let set_selection = set_selection.clone();
            let add_to_history = add_to_history.clone();
            Arc::new(move || {
                add_to_history(selection_start.get());
                update_cursor_info();
                let text = value.0.get();
                let start = selection_start.get();
                let (new_text, _) = insert_text_at_cursor(&text, "![alt text](image-url)", start);
                value.1.set(new_text);
                // Select "alt text"
                set_selection(start + 2, start + 10);
            })
        },
        undo: {
            let value = value.clone();
            let set_cursor_position = set_cursor_position.clone();
            Arc::new(move || {
                let current_index = history_index.get();
                if current_index > 1 {
                    // Move back in history
                    let new_index = current_index - 1;
                    history_index.set(new_index);
                    
                    let hist = history.get();
                    if let Some((text, cursor_pos)) = hist.get(new_index - 1) {
                        value.1.set(text.clone());
                        set_cursor_position(*cursor_pos);
                    }
                }
            })
        },
    };
    
    provide_context(markdown_functions);
    
    view! {
        <div class="w-full flex flex-col h-screen">
            {children()}
            <div class="p-1 rounded-[4px] w-full flex flex-1 border border-gray-300">
                <textarea
                    class="p-1 border-none w-full h-full resize-none"
                    node_ref=textarea_ref
                    prop:value=move || value.0.get()
                    on:input:target=move |ev| {
                        value.1.set(ev.target().value());
                    }
                    on:keydown={
                        let add_to_history = add_to_history.clone();
                        let update_cursor_info = update_cursor_info.clone();
                        let undo_fn = {
                            let value = value.clone();
                            let set_cursor_position = set_cursor_position.clone();
                            move || {
                                let current_index = history_index.get();
                                if current_index > 1 {
                                    // Move back in history
                                    let new_index = current_index - 1;
                                    history_index.set(new_index);
                                    
                                    let hist = history.get();
                                    if let Some((text, cursor_pos)) = hist.get(new_index - 1) {
                                        value.1.set(text.clone());
                                        set_cursor_position(*cursor_pos);
                                    }
                                }
                            }
                        };
                        move |ev: web_sys::KeyboardEvent| {
                            let key = ev.key();
                            
                            // Handle Ctrl+Z for undo
                            if ev.ctrl_key() && key == "z" {
                                ev.prevent_default();
                                undo_fn();
                                return;
                            }
                            
                            // Save state before significant changes
                            if key == "Enter" || key == "Backspace" || key == "Delete" || 
                               (ev.ctrl_key() && (key == "v" || key == "x")) {
                                update_cursor_info();
                                add_to_history(cursor_position.get());
                            }
                        }
                    }
                    on:click=move |_| update_cursor_info()
                    on:keyup=move |_| update_cursor_info()
                    on:select=move |_| update_cursor_info()
                >
                    {value.0.get()}
                </textarea>
            </div>
        </div>
    }
}
