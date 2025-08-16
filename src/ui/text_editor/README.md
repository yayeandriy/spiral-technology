# Text Editor Module

A comprehensive text editor component system for Leptos applications.

## Components

### Core Components

#### `TextEditor`
The basic text editor component with essential functionality:
- Text content editing with reactive signals
- Status indicators (saving, saved, unsaved changes)
- Action buttons (save, cancel)
- Error display
- Content information display

#### `AdvancedTextEditor`
An enhanced version of the text editor with additional features:
- All basic text editor functionality
- Toolbar with formatting options
- Settings panel
- Fullscreen mode
- Auto-save configuration

### Supporting Components

#### `StatusIndicator`
Displays the current status of the editor:
- Idle state
- Saving state with spinner
- Saved state with green indicator
- Unsaved changes with orange indicator

#### `ActionButtons`
Provides save and cancel functionality:
- Save/Update button (disabled when no changes or saving)
- Cancel button (reverts to original content)
- Mode description (Create vs Update)

#### `ContentInfo`
Displays metadata about the content:
- Content ID
- Creation date
- Other custom information

#### `EditorToolbar`
Rich text formatting toolbar:
- Bold, italic, code formatting
- Lists and quotes
- Links
- Undo/redo actions
- Find functionality
- Fullscreen toggle

#### `EditorSettings`
Configuration panel for editor behavior:
- Auto-save toggle
- Word wrap toggle
- Spell check toggle
- Font size adjustment
- Theme selection

## Types

### `EditorMode`
- `Create`: Creating new content
- `Update`: Updating existing content

### `EditorStatus`
- `Idle`: No ongoing operations
- `Saving`: Save operation in progress
- `Saved`: Content has been saved
- `HasUnsavedChanges`: Content has been modified

### `EditorConfig`
Configuration structure for editor settings:
- `auto_save`: Enable automatic saving
- `auto_save_delay_ms`: Delay before auto-save triggers
- `show_line_numbers`: Display line numbers
- `font_size`: Editor font size
- `theme`: Editor theme (Light, Dark, Auto)
- `word_wrap`: Enable word wrapping
- `spell_check`: Enable spell checking

### `ToolbarAction`
Actions available in the toolbar:
- Formatting: `Bold`, `Italic`, `Underline`, `Code`
- Structure: `List`, `Quote`, `Link`
- History: `Undo`, `Redo`
- Utility: `Find`, `Replace`, `FullScreen`

## Usage

### Basic Text Editor

```rust
use crate::ui::text_editor::{TextEditor, EditorMode, EditorStatus};

#[component]
pub fn MyEditor() -> impl IntoView {
    let content = RwSignal::new(String::new());
    let save_trigger = RwSignal::new(false);
    let cancel_trigger = RwSignal::new(false);
    
    let status = Signal::derive(|| EditorStatus::Idle);
    let mode = Signal::derive(|| EditorMode::Create);
    let error = Signal::derive(|| None);
    
    view! {
        <TextEditor
            content=content
            status=status
            error=error
            mode=mode
            save_trigger=save_trigger
            cancel_trigger=cancel_trigger
            title="My Editor".to_string()
            placeholder="Enter your text here...".to_string()
        />
    }
}
```

### Advanced Text Editor with Toolbar

```rust
use crate::ui::text_editor::{AdvancedTextEditor, EditorMode, EditorStatus, EditorConfig};

#[component]
pub fn MyAdvancedEditor() -> impl IntoView {
    let content = RwSignal::new(String::new());
    let save_trigger = RwSignal::new(false);
    let cancel_trigger = RwSignal::new(false);
    let editor_config = RwSignal::new(EditorConfig::default());
    
    view! {
        <AdvancedTextEditor
            content=content
            status=Signal::derive(|| EditorStatus::Idle)
            error=Signal::derive(|| None)
            mode=Signal::derive(|| EditorMode::Create)
            save_trigger=save_trigger
            cancel_trigger=cancel_trigger
            title="Advanced Editor".to_string()
            show_toolbar=true
            show_settings=true
            editor_config=editor_config
        />
    }
}
```

## Integration

The text editor uses reactive signals for all state management, making it easy to integrate with external data sources and contexts. The save and cancel operations are triggered through signals, allowing the parent component to handle the actual persistence logic.

### Example with Context Integration

```rust
// Handle save trigger
Effect::new({
    let save_trigger = save_trigger.clone();
    let content = content.clone();
    let my_context = my_context.clone();
    move || {
        if save_trigger.get() {
            let content_value = content.get();
            spawn_local(async move {
                my_context.save_content(content_value).await;
            });
        }
    }
});
```

## Customization

All components accept additional CSS classes and can be customized through props. The modular structure allows you to use individual components or compose them as needed for your specific use case.
