# Assets Directory

This directory contains all visual assets for the Inventory Manager application.

## Directory Structure

```
assets/
├── icons/
│   ├── nav/          # Navigation icons (24x24px recommended)
│   ├── actions/      # Action button icons (20x20px)
│   ├── status/       # Status indicator icons (16x16px)
│   └── misc/         # Other UI elements
├── illustrations/    # Empty state illustrations (200-300px)
└── logos/           # Application branding
```

## Icon Naming Convention

### Navigation Icons (`icons/nav/`)
- `inventory.svg` - Inventory management view
- `notes.svg` - Notes editor view
- `alerts.svg` - Alerts/notifications view
- `settings.svg` - Settings view
- `users.svg` - User management view
- `audit-log.svg` - Audit log view

### Action Icons (`icons/actions/`)
- `add.svg` or `plus.svg` - Add new items
- `edit.svg` or `pencil.svg` - Edit actions
- `delete.svg` or `trash.svg` - Delete actions
- `search.svg` - Search functionality
- `save.svg` - Save actions
- `check.svg` - Confirm/success
- `close.svg` or `x.svg` - Close/cancel
- `filter.svg` - Filter panel toggle
- `logout.svg` - Logout action

### Status Icons (`icons/status/`)
- `check-circle.svg` - In stock / success
- `alert-triangle.svg` - Low stock warning
- `alert-circle.svg` - Critical alert
- `x-circle.svg` - Out of stock / error
- `info.svg` - Information

### Misc Icons (`icons/misc/`)
- `user.svg` - User profile
- `calculator.svg` - Calculator widget
- `chart.svg` or `stats.svg` - Statistics
- `dollar.svg` - Currency/value
- `box.svg` - Items/quantity

### Illustrations (`illustrations/`)
- `empty-inventory.svg` - No items in inventory
- `no-results.svg` - No search results
- `no-alerts.svg` - No active alerts
- `no-notes.svg` - No notes created

### Logos (`logos/`)
- `app-icon.svg` - Main application icon
- `logo.svg` - Full logo with text
- `logo-small.svg` - Compact logo variant

## SVG Guidelines

### For Icons:
- Use a consistent viewBox (e.g., 0 0 24 24)
- Keep paths simple and clean
- Use single color (will be styled by theme)
- Optimize file size
- Remove unnecessary metadata

### For Illustrations:
- Can use multiple colors
- Keep file size reasonable (<50KB)
- Consider using CSS variables for theme colors
- Ensure proper viewBox for scaling

## Adding New Assets

1. Place SVG files in the appropriate directory
2. Follow the naming convention (kebab-case)
3. Ensure SVGs are optimized
4. Update the icon enum in `src/icons.rs` if adding new icons
5. Test in both light and dark themes

## Current Status

⚠️ **Assets are currently placeholder emoji**

To complete the SVG integration:
1. Add SVG files following the naming convention above
2. The application will automatically load them
3. Icons will respect the current theme colors

