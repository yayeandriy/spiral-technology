# UI Components Module

This module contains reusable UI components for the application, providing a single source of truth for all UI elements with consistent styling and behavior.

## Components

### Button Components

#### `Button`
Base button component with variant and size support.

```rust
use crate::ui::*;

view! {
    <Button 
        variant=ButtonVariant::Primary 
        size=ButtonSize::Medium
        on_click=Box::new(move |_| { /* handle click */ })
    >
        "Click me"
    </Button>
}
```

#### Convenience Components
- `PrimaryButton` - Blue primary button
- `SecondaryButton` - Light blue secondary button  
- `DangerButton` - Red danger button
- `CancelButton` - Gray cancel button

```rust
view! {
    <PrimaryButton on_click=Box::new(move |_| save())>
        "Save"
    </PrimaryButton>
    
    <CancelButton on_click=Box::new(move |_| cancel())>
        "Cancel"
    </CancelButton>
}
```

### Input Components

#### `TextInput`
Standard text input with validation states.

```rust
view! {
    <TextInput
        value=title_signal
        placeholder="Enter title...".to_string()
        state=if has_error { InputState::Error } else { InputState::Normal }
        on_input=Box::new(move |ev| set_title(event_target_value(&ev)))
        required=true
    />
}
```

#### Input Types
- `TextInput` - Standard text input
- `EmailInput` - Email input with email validation
- `PasswordInput` - Password input with masked text

### TextArea Components

#### `TextArea`
Multi-line text input with validation support.

```rust
view! {
    <TextArea
        value=description_signal
        rows=4
        placeholder="Enter description...".to_string()
        max_length=Some(500)
        on_input=Box::new(move |ev| set_description(event_target_value(&ev)))
    />
}
```

#### `TextAreaWithCounter`
TextArea with built-in character counter.

```rust
view! {
    <TextAreaWithCounter
        value=description_signal
        max_length=500
        placeholder="Enter description...".to_string()
        rows=4
        on_input=Box::new(move |ev| set_description(event_target_value(&ev)))
    />
}
```

### Select Components

#### `Select`
Dropdown select with customizable options.

```rust
let options = vec![
    SelectOption::new("value1", "Label 1"),
    SelectOption::new("value2", "Label 2"),
    SelectOption::new_disabled("disabled", "Disabled Option"),
];

view! {
    <Select
        value=selected_signal
        options=options
        placeholder=Some("Choose an option...".to_string())
        on_change=Box::new(move |ev| set_selected(event_target_value(&ev)))
    />
}
```

#### `CategorySelect`
Specialized select for category selection with "Add new category" option.

```rust
view! {
    <CategorySelect
        value=category_signal
        categories=available_categories
        on_change=Box::new(move |ev| handle_category_change(event_target_value(&ev)))
    />
}
```

### Checkbox Components

#### `Checkbox`
Standard checkbox input.

```rust
view! {
    <Checkbox
        checked=checked_signal
        on_change=Box::new(move |_| toggle_checked())
    />
}
```

#### `CheckboxWithLabel`
Checkbox with associated label.

```rust
view! {
    <CheckboxWithLabel
        checked=agreed_signal
        on_change=Box::new(move |_| toggle_agreement())
    >
        "I agree to the terms and conditions"
    </CheckboxWithLabel>
}
```

#### `AreaCheckbox`
Specialized checkbox for area selection with title and description.

```rust
view! {
    <AreaCheckbox
        area_id=area.id
        title=area.title.clone()
        description=area.description.clone()
        selected=is_selected_signal
        on_change=Box::new(move |_| toggle_area_selection(area.id))
    />
}
```

### Label Components

#### `Label`
Form label with required field indicator.

```rust
view! {
    <Label for_="title".to_string() required=true>
        "Project Title"
    </Label>
}
```

#### `FieldLabel`
Convenience label component for form fields.

```rust
view! {
    <FieldLabel
        text="Email Address".to_string()
        for_="email".to_string()
        required=true
    />
}
```

### Error and Message Components

#### `ErrorMessage`
Display error messages.

```rust
view! {
    <ErrorMessage message="This field is required".to_string() />
}
```

#### `FieldError`
Conditional error display for form fields.

```rust
view! {
    <FieldError error=validation_error />
}
```

#### `ValidationErrors`
Display multiple validation errors.

```rust
view! {
    <ValidationErrors errors=vec![
        "Title is required".to_string(),
        "Email is invalid".to_string(),
    ] />
}
```

#### Message Components
- `InfoMessage` - Gray info text
- `SuccessMessage` - Green success text
- `ErrorMessage` - Red error text

### Form Layout Components

#### `FormField`
Complete form field with label, input, and error display.

```rust
view! {
    <FormField
        label="Project Title".to_string()
        required=true
        error=title_error
        help=Some("Enter a descriptive title".to_string())
    >
        <TextInput
            value=title_signal
            placeholder="Enter title...".to_string()
            on_input=Box::new(move |ev| set_title(event_target_value(&ev)))
        />
    </FormField>
}
```

#### `TextFormField`
Convenient combination of FormField + TextInput.

```rust
view! {
    <TextFormField
        value=title_signal
        label="Project Title".to_string()
        placeholder="Enter title...".to_string()
        required=true
        error=title_error
        on_input=Box::new(move |ev| set_title(event_target_value(&ev)))
    />
}
```

#### `TextAreaFormField`
Convenient combination of FormField + TextArea.

```rust
view! {
    <TextAreaFormField
        value=description_signal
        label="Description".to_string()
        placeholder="Enter description...".to_string()
        max_length=Some(500)
        show_counter=true
        error=description_error
        on_input=Box::new(move |ev| set_description(event_target_value(&ev)))
    />
}
```

## Migration Examples

### Before (Raw HTML)
```rust
view! {
    <div class="space-y-2">
        <label for="title" class="block text-sm font-medium text-gray-700">
            "Project Title" <span class="text-red-500">*</span>
        </label>
        <input
            type="text"
            id="title"
            class="w-full px-3 py-2 text-black border rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 border-gray-300 focus:border-blue-500"
            placeholder="Enter project title..."
            prop:value=title
            on:input=move |ev| set_title.set(event_target_value(&ev))
            disabled=is_submitting
        />
        <p class="text-sm text-red-600">"Title is required"</p>
    </div>
}
```

### After (UI Components)
```rust
view! {
    <TextFormField
        value=title.into()
        label="Project Title".to_string()
        placeholder="Enter project title...".to_string()
        required=true
        disabled=is_submitting.get()
        error=title_error()
        on_input=Box::new(move |ev| set_title.set(event_target_value(&ev)))
    />
}
```

## Benefits

1. **Consistency**: All UI elements follow the same design patterns and styling
2. **Maintainability**: Changes to styling can be made in one place
3. **Reusability**: Components can be easily used across different parts of the application
4. **Type Safety**: Props are strongly typed with clear documentation
5. **Accessibility**: Built-in ARIA attributes and proper semantic HTML
6. **DX (Developer Experience)**: Less boilerplate code and clearer component APIs

## Styling

All components use Tailwind CSS classes and follow the application's design system. The components are designed to be:

- **Responsive**: Work well on different screen sizes
- **Accessible**: Include proper ARIA attributes and keyboard navigation
- **Consistent**: Follow the same visual patterns throughout the app
- **Customizable**: Accept additional CSS classes for specific use cases
