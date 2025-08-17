use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlTextAreaElement};
use crate::ui::text_editor::types::{EditorMetrics, MarkdownConfig};

pub struct MarkdownProcessor {
    config: MarkdownConfig,
}

impl MarkdownProcessor {
    pub fn new(config: MarkdownConfig) -> Self {
        Self { config }
    }

    /// Insert Markdown syntax at cursor position
    pub fn insert_markdown(&self, textarea: &HtmlTextAreaElement, syntax: &str, wrap_text: bool) {
        let start = textarea.selection_start().unwrap().unwrap_or(0) as usize;
        let end = textarea.selection_end().unwrap().unwrap_or(0) as usize;
        let value = textarea.value();
        
        let (new_value, new_cursor_pos) = if wrap_text && start != end {
            // Wrap selected text
            let selected = &value[start..end];
            let wrapped = format!("{}{}{}", syntax, selected, syntax);
            let new_val = format!("{}{}{}", &value[..start], wrapped, &value[end..]);
            (new_val, start + syntax.len() + selected.len())
        } else {
            // Insert at cursor
            let new_val = format!("{}{}{}", &value[..start], syntax, &value[start..]);
            (new_val, start + syntax.len())
        };
        
        textarea.set_value(&new_value);
        let _ = textarea.set_selection_range(new_cursor_pos as u32, new_cursor_pos as u32);
        textarea.focus().ok();
    }

    /// Insert Markdown block at current line
    pub fn insert_block(&self, textarea: &HtmlTextAreaElement, block_syntax: &str) {
        let start = textarea.selection_start().unwrap().unwrap_or(0) as usize;
        let value = textarea.value();
        
        // Find start of current line
        let line_start = value[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
        
        let new_value = format!("{}{}\n{}", &value[..line_start], block_syntax, &value[line_start..]);
        let new_cursor_pos = line_start + block_syntax.len() + 1;
        
        textarea.set_value(&new_value);
        let _ = textarea.set_selection_range(new_cursor_pos as u32, new_cursor_pos as u32);
        textarea.focus().ok();
    }

    /// Get Markdown templates for various elements
    pub fn get_template(&self, element: &str) -> String {
        match element {
            "header1" => "# ".to_string(),
            "header2" => "## ".to_string(),
            "header3" => "### ".to_string(),
            "header4" => "#### ".to_string(),
            "header5" => "##### ".to_string(),
            "header6" => "###### ".to_string(),
            "bold" => "**text**".to_string(),
            "italic" => "*text*".to_string(),
            "strikethrough" => "~~text~~".to_string(),
            "code" => "`code`".to_string(),
            "code_block" => "```\ncode\n```".to_string(),
            "bullet_list" => "- ".to_string(),
            "numbered_list" => "1. ".to_string(),
            "todo_list" => "- [ ] ".to_string(),
            "quote" => "> ".to_string(),
            "horizontal_rule" => "---".to_string(),
            "link" => "[text](url)".to_string(),
            "image" => "![alt](url)".to_string(),
            "table" => "| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |".to_string(),
            "math" => "$$ math $$".to_string(),
            "mermaid" => "```mermaid\ngraph TD;\n    A-->B;\n    A-->C;\n    B-->D;\n    C-->D;\n```".to_string(),
            _ => String::new(),
        }
    }

    /// Calculate text metrics
    pub fn calculate_metrics(&self, text: &str) -> EditorMetrics {
        let character_count = text.len();
        let word_count = text.split_whitespace().count();
        let line_count = text.lines().count().max(1);
        
        // Estimate reading time (average 200 words per minute)
        let reading_time_minutes = (word_count as f64 / 200.0).ceil() as usize;
        
        EditorMetrics {
            word_count,
            character_count,
            line_count,
            reading_time_minutes,
        }
    }

    /// Convert Markdown to HTML for preview
    pub fn to_html(&self, markdown: &str) -> String {
        // Basic Markdown to HTML conversion
        // In a real implementation, you'd use a proper Markdown parser like pulldown-cmark
        let mut html = markdown.to_string();
        
        // Headers (simple string replacement for WASM compatibility)
        html = html.lines().map(|line| {
            if line.starts_with("# ") {
                format!("<h1>{}</h1>", &line[2..])
            } else if line.starts_with("## ") {
                format!("<h2>{}</h2>", &line[3..])
            } else if line.starts_with("### ") {
                format!("<h3>{}</h3>", &line[4..])
            } else if line.starts_with("#### ") {
                format!("<h4>{}</h4>", &line[5..])
            } else if line.starts_with("##### ") {
                format!("<h5>{}</h5>", &line[6..])
            } else if line.starts_with("###### ") {
                format!("<h6>{}</h6>", &line[7..])
            } else {
                line.to_string()
            }
        }).collect::<Vec<_>>().join("<br>");
        
        // Simple replacements for common markdown patterns
        html = html.replace("**", "<strong>").replace("**", "</strong>");
        html = html.replace("*", "<em>").replace("*", "</em>");
        html = html.replace("~~", "<del>").replace("~~", "</del>");
        html = html.replace("`", "<code>").replace("`", "</code>");
        
        // Convert line breaks
        html = html.replace("\n", "<br>");
        
        html
    }

    /// Validate Markdown syntax
    pub fn validate(&self, markdown: &str) -> Vec<String> {
        let mut errors = Vec::new();
        
        // Check for unmatched brackets
        let mut bracket_count = 0;
        for char in markdown.chars() {
            match char {
                '[' => bracket_count += 1,
                ']' => bracket_count -= 1,
                _ => {}
            }
        }
        if bracket_count != 0 {
            errors.push("Unmatched square brackets".to_string());
        }
        
        // Check for unmatched parentheses
        let mut paren_count = 0;
        for char in markdown.chars() {
            match char {
                '(' => paren_count += 1,
                ')' => paren_count -= 1,
                _ => {}
            }
        }
        if paren_count != 0 {
            errors.push("Unmatched parentheses".to_string());
        }
        
        errors
    }
}

#[component]
pub fn MarkdownPreview(
    /// The Markdown content to preview
    content: String,
    /// Configuration for Markdown processing
    #[prop(default = MarkdownConfig::default())]
    config: MarkdownConfig,
) -> impl IntoView {
    let processor = MarkdownProcessor::new(config);
    let html_content = processor.to_html(&content);
    
    view! {
        <div class="markdown-preview p-4 bg-white border border-gray-200 rounded prose max-w-none">
            <div inner_html=html_content></div>
        </div>
    }
}
