#!/bin/bash

# UI Migration Helper Script
# This script helps migrate from old UI components to new signal-based ones

echo "🚀 UI Migration Helper"
echo "======================"

# Function to show migration suggestions for a file
suggest_migrations() {
    local file="$1"
    echo "📄 Analyzing $file..."
    
    # Check for old button patterns
    if grep -q "PrimaryButton" "$file"; then
        echo "  ✏️  Found PrimaryButton usage - consider migrating to SignalPrimaryButton"
    fi
    
    if grep -q "SecondaryButton" "$file"; then
        echo "  ✏️  Found SecondaryButton usage - consider migrating to SignalSecondaryButton"
    fi
    
    if grep -q "CancelButton" "$file"; then
        echo "  ✏️  Found CancelButton usage - consider migrating to SignalCancelButton"
    fi
    
    if grep -q "DangerButton" "$file"; then
        echo "  ✏️  Found DangerButton usage - consider migrating to SignalDangerButton"
    fi
    
    # Check for Box<dyn Fn> patterns
    if grep -q "Box::new(move" "$file"; then
        echo "  ⚠️  Found Box::new(move |_|) pattern - can be simplified with signal components"
    fi
    
    # Check for manual form handling
    if grep -q "on:input=move" "$file" && grep -q "event_target_value" "$file"; then
        echo "  📝 Found manual form input handling - consider using SignalFormField"
    fi
    
    echo ""
}

# Function to backup a file before migration
backup_file() {
    local file="$1"
    cp "$file" "$file.backup.$(date +%Y%m%d_%H%M%S)"
    echo "📦 Backed up $file"
}

# Main migration analysis
echo "🔍 Scanning for migration opportunities..."
echo ""

# Find all Rust files in src directory
find src -name "*.rs" -type f | while read -r file; do
    suggest_migrations "$file"
done

echo "💡 Migration Tips:"
echo "=================="
echo "1. Start with isolated components (new features first)"
echo "2. Backup files before migrating (this script can help)"
echo "3. Test each component after migration"
echo "4. Use the UI Migration Demo page for reference"
echo ""

echo "📚 Available Migration Resources:"
echo "================================"
echo "• SIGNAL_UI_MIGRATION.md - Complete migration guide"
echo "• src/pages/ui_migration_demo.rs - Live demo page"
echo "• src/ui/signal_*.rs - New signal-based components"
echo "• src/ui/working_signal_demo.rs - Working examples"
echo ""

# Interactive migration helper
if [ "$1" = "--interactive" ]; then
    echo "🛠️  Interactive Migration Mode"
    echo "==============================="
    
    echo "Select a migration option:"
    echo "1. Backup all source files"
    echo "2. Show detailed analysis for specific file"
    echo "3. Generate component replacement patterns"
    echo "4. Exit"
    
    read -p "Enter your choice (1-4): " choice
    
    case $choice in
        1)
            echo "📦 Backing up all source files..."
            find src -name "*.rs" -type f | while read -r file; do
                backup_file "$file"
            done
            echo "✅ All files backed up!"
            ;;
        2)
            read -p "Enter file path: " filepath
            if [ -f "$filepath" ]; then
                suggest_migrations "$filepath"
                echo "Detailed suggestions for $filepath:"
                echo "• Check SIGNAL_UI_MIGRATION.md for specific patterns"
                echo "• Use the migration demo for reference"
                echo "• Test components individually after migration"
            else
                echo "❌ File not found: $filepath"
            fi
            ;;
        3)
            echo "📋 Common Migration Patterns:"
            echo ""
            echo "Button Migration:"
            echo "  Before: <PrimaryButton on_click=Box::new(move |_| action())>"
            echo "  After:  <SignalPrimaryButton on_click=move |_| action()>"
            echo ""
            echo "Form Field Migration:"
            echo "  Before: <input on:input=move |ev| set_value(event_target_value(&ev)) />"
            echo "  After:  <SignalFormField on_input=move |value| set_value(value) />"
            echo ""
            echo "Import Changes:"
            echo "  Add: use crate::ui::{SignalPrimaryButton, SignalCancelButton};"
            echo "  Add: use crate::ui::{SignalForm, SignalFormField};"
            ;;
        4)
            echo "👋 Goodbye!"
            exit 0
            ;;
        *)
            echo "❌ Invalid choice"
            ;;
    esac
fi

echo "🎯 Next Steps:"
echo "============="
echo "1. Review the migration guide: SIGNAL_UI_MIGRATION.md"
echo "2. Visit the demo page to see examples side-by-side"
echo "3. Start migrating isolated components first"
echo "4. Run './migrate_ui.sh --interactive' for guided assistance"
echo ""
echo "✨ Happy migrating!"
