# Inventory Manager

A modern, feature-rich desktop application for managing inventory, taking notes, and performing quick calculations.

Built with Rust 🦀 and Iced GUI framework.

## ✨ Features

### 📦 Enhanced Inventory Management
- **Comprehensive Item Details**: Track name, SKU, category, supplier, quantity, and price
- **Timestamp Tracking**: Automatic created_at and updated_at timestamps for each item
- **Unique IDs**: UUID-based identification for reliable item tracking
- **Dialog-Based Forms**: Clean modal dialogs for adding and editing items
- **Rich Table View**: Display all item details in an organized, sortable table
- **Automatic Calculations**: Real-time calculation of item totals and overall inventory value
- **Edit & Delete**: Easily modify or remove inventory items

### 📝 Multi-Note System with CRUD Operations
- **Create Multiple Notes**: Organize your thoughts with unlimited notes
- **Sidebar Navigation**: Browse all your notes in a dedicated sidebar
- **Full-Featured Editor**: Multi-line text editor for each note
- **Individual Note Titles**: Name each note for easy identification
- **Timestamp Tracking**: Each note tracks creation and update times
- **Delete with Confirmation**: Safely remove notes with confirmation dialog
- **Auto-Save**: All changes are automatically saved as you type
- **Live Statistics**: View line count and character count
- **Keyboard Shortcut**: Create new notes with `Ctrl/Cmd + N`

### 🧮 Floating Calculator with Position Memory
- **Draggable Interface**: Click and drag the calculator anywhere on screen
- **Position Persistence**: Calculator remembers its location between sessions
- **Basic Operations**: Addition, subtraction, multiplication, division
- **Decimal Support**: Work with floating-point numbers
- **Clear Function**: Reset with one click
- **Always Accessible**: Toggle visibility without losing calculations
- **Keyboard Shortcut**: Show/hide with `Ctrl/Cmd + K`

### 💾 Advanced Data Persistence
- **Auto-Save**: All data automatically saved as you work
- **Comprehensive State**: Saves inventory items, all notes, and calculator position
- **Cross-Platform Storage**:
  - macOS: `~/Library/Application Support/com.inventory.app/`
  - Linux: `~/.local/share/inventory/app/`
  - Windows: `%APPDATA%\inventory\app\`
- **Graceful Migration**: Handles old data formats smoothly
- **First-Run Experience**: Creates welcome note on first launch

### ⌨️ Comprehensive Keyboard Shortcuts
- `Ctrl/Cmd + 1` - Switch to Inventory view
- `Ctrl/Cmd + 2` - Switch to Notes view
- `Ctrl/Cmd + N` - Create new note
- `Ctrl/Cmd + K` - Toggle Calculator
- `Ctrl/Cmd + S` - Manual Save (auto-save active)
- `Ctrl/Cmd + I` - Show About dialog
- `Escape` - Close dialogs/overlays

### 🪟 Professional Desktop Experience
- **Loading Screen**: Smooth startup with 3-second splash screen
- **Optimized Window**:
  - Default size: 1200×800
  - Minimum size: 900×600
  - Fully resizable
- **Dark Theme**: Professional dark interface
- **Modal Dialogs**: Clean overlay system for forms and confirmations
- **Responsive Layout**: Adapts to window resizing
- **Multiple Views**: Seamless switching between Inventory and Notes
- **About Dialog**: View app information and credits

## 🚀 Installation & Running

### Prerequisites
- Rust (latest stable version)
- Cargo (comes with Rust)

### Building from Source
```bash
# Clone or navigate to the project directory
cd inventory_app

# Build the application
cargo build --release

# Run the application
cargo run --release
```

### Development Mode
```bash
cargo run
```

## 📖 Usage Guide

### Managing Inventory

1. Click "Add Item" button to open the item dialog
2. Fill in all fields:
   - Item Name (required)
   - SKU (required)
   - Category (e.g., Electronics, Food)
   - Supplier name
   - Quantity
   - Price
3. Click "Submit" to add or "Cancel" to discard
4. Click "Edit" on any item to modify it
5. Click "Delete" to remove an item
6. View totals at the top: total items and total value

### Working with Multiple Notes

1. Click "Notes" in the header or press `Ctrl+2`
2. Use "+ New Note" button or press `Ctrl+N` to create a note
3. Click any note in the sidebar to view/edit it
4. Edit the note title at the top
5. Type your content in the editor below
6. Click the × button on a note to delete it (with confirmation)
7. All changes auto-save as you work
8. View statistics at the bottom

### Using the Calculator

1. Click "Show Calc" or press `Ctrl+K`
2. Click number buttons or use operations
3. Press `=` to calculate result
4. Press "Clear" to reset
5. **Drag the calculator**: Click and hold on the calculator window to move it
6. Position persists between sessions
7. Toggle visibility anytime with `Ctrl+K`

## 🏗️ Project Structure

```
src/
├── main.rs              # Application entry & orchestration
├── calculator.rs        # Calculator state & operations
├── inventory.rs         # InventoryItem struct with timestamps
├── note.rs              # Note struct with CRUD operations
├── messages.rs          # Message types & enums
├── persistence.rs       # Data save/load functionality
└── views/
    ├── mod.rs           # Views module declaration
    ├── inventory.rs     # Inventory table view
    ├── editor.rs        # Multi-note editor with sidebar
    ├── calculator.rs    # Calculator UI
    ├── item_dialog.rs   # Add/Edit item modal dialog
    ├── loading.rs       # Loading screen
    └── about.rs         # About dialog
```

## 🔧 Technologies Used

- **Rust** - Systems programming language
- **Iced** - Cross-platform GUI framework
- **Serde & Serde JSON** - Serialization/deserialization
- **UUID** - Unique identifier generation
- **Chrono** - Date and time handling
- **Tokio** - Async runtime for file I/O
- **Directories** - Cross-platform app data directories

## 📝 Data Format

Data is stored as JSON with the following structure:

```json
{
  "items": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "Product Name",
      "sku": "SKU-12345",
      "category": "Electronics",
      "supplier": "Supplier Inc",
      "quantity": 10,
      "price": 29.99,
      "created_at": 1704067200,
      "updated_at": 1704153600
    }
  ],
  "notes": [
    {
      "id": "660e8400-e29b-41d4-a716-446655440001",
      "title": "Meeting Notes",
      "content": "Discussion points...",
      "created_at": 1704067200,
      "updated_at": 1704153600
    }
  ],
  "calculator_position": [450.0, 200.0]
}
```

## 🎯 Key Improvements from v0.1.0

### Inventory System
- ✅ Added comprehensive fields (SKU, category, supplier)
- ✅ UUID-based identification
- ✅ Timestamp tracking (created/updated)
- ✅ Dialog-based forms (no inline clutter)
- ✅ Enhanced table view with all details

### Notes System
- ✅ Multi-note support (was single note)
- ✅ Full CRUD operations
- ✅ Sidebar navigation
- ✅ Individual note titles
- ✅ Delete confirmations
- ✅ Timestamp tracking per note

### Calculator
- ✅ Draggable positioning
- ✅ Position persistence
- ✅ Improved UX

### Data Persistence
- ✅ Backward compatible loading
- ✅ Comprehensive state saving
- ✅ Calculator position memory

## 🎨 User Experience Highlights

- **Modal Dialogs**: Clean overlays for forms prevent UI clutter
- **Auto-Save**: Never lose work with automatic persistence
- **Keyboard-First**: Full keyboard navigation support
- **Visual Feedback**: Clear indicators for selected notes, active views
- **Confirmation Dialogs**: Prevent accidental data loss
- **Loading States**: Professional 3-second splash screen
- **Responsive Design**: Adapts to any window size

## 📄 License

This project is open source. Feel free to use and modify as needed.

## 🤝 Contributing

Contributions are welcome! Feel free to submit issues or pull requests.

## 🔮 Future Enhancements

Potential features for future versions:
- Drag & drop for calculator (full implementation)
- Export inventory to CSV/PDF
- Import data from files
- Search and filter functionality
- Multiple inventory categories with filtering
- Note folders/tags
- Rich text formatting in notes
- Data backup and restore
- Custom themes and color schemes
- Multi-language support
- Charts and analytics dashboard
- Barcode scanning support

---

**Version**: 0.2.0  
**Built with**: Rust & Iced GUI Framework  
**Author**: Inventory Manager Team
