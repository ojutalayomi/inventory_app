# Inventory Manager - Testing Guide

## Overview
This guide covers testing of the enterprise features implemented in the Inventory Manager application:
- ✅ **Phase 1**: User Authentication & Role-Based Access Control
- ✅ **Phase 2**: Comprehensive Audit Trail
- ✅ **Phase 4**: Advanced Validation & Error Handling (Partial)

---

## Prerequisites
1. Build and run the application:
   ```bash
   cd /Users/ayomide/rust/inventory_app
   cargo run
   ```

2. **Default Admin Account**:
   - Username: `admin`
   - Password: `admin123`

---

## Phase 1: Authentication & Role-Based Access Control

### Test 1.1: Login Screen
**Expected**: App should show a login screen on startup.

**Steps**:
1. Launch the app
2. Verify you see:
   - "Inventory Manager" title
   - "Please log in to continue" subtitle
   - Username input field
   - Password input field (should hide characters)
   - "Log In" button
   - Default credentials hint at bottom

**Result**: ✅ Login screen displays correctly

---

### Test 1.2: Invalid Login
**Expected**: Login fails with error message for invalid credentials.

**Steps**:
1. Enter username: `wronguser`
2. Enter password: `wrongpass`
3. Click "Log In"

**Expected Result**: Red error message appears: "Invalid username or password"

---

### Test 1.3: Successful Admin Login
**Expected**: Admin logs in and sees all features.

**Steps**:
1. Enter username: `admin`
2. Enter password: `admin123`
3. Click "Log In"

**Expected Result**:
- App transitions to main interface
- Top bar shows: "Logged in as: admin (Admin)"
- Navigation buttons visible:
  - ✅ Inventory
  - ✅ Notes
  - ✅ Settings
  - ✅ Users (Admin only)
  - ✅ Audit Log (Admin/Manager only)
  - ✅ About
- Red "Logout" button on the right

---

### Test 1.4: User Management (Admin Only)
**Expected**: Admin can create, view, and delete users.

**Steps**:
1. Log in as admin
2. Click "Users" button
3. Verify you see:
   - "User Management" title
   - "Add New User" section with fields:
     - Username (text input)
     - Password (text input, should be hidden)
     - Role (dropdown: Admin, Manager, User, Viewer)
     - "Add User" button
   - "Existing Users" section showing the admin user

**Test Create User**:
1. Enter new username: `testmanager`
2. Enter password: `password123`
3. Select role: `Manager`
4. Click "Add User"

**Expected Result**:
- Form clears
- New user appears in "Existing Users" list
- Shows: username, role (Manager), status (Active), last login (Never)
- "Delete" button appears for the new user (but not for admin)

**Test Delete User**:
1. Click "Delete" button next to `testmanager`
2. User is removed from list immediately

**Test Logout**:
1. Click "Logout" button
2. Returns to login screen

**Test Login as New User**:
1. Login with username: `testmanager`, password: `password123`
2. Verify Manager role displays
3. Verify "Users" button is NOT visible (only admins can manage users)
4. Verify "Audit Log" button IS visible (managers can view audit)

---

### Test 1.5: Role-Based Permissions
**Expected**: Different roles have different access levels.

**Permission Matrix**:

| Action | Admin | Manager | User | Viewer |
|--------|-------|---------|------|--------|
| View Items | ✅ | ✅ | ✅ | ✅ |
| Create Items | ✅ | ✅ | ✅ | ❌ |
| Edit Items | ✅ | ✅ | ✅ | ❌ |
| Delete Items | ✅ | ✅ | ❌ | ❌ |
| Manage Users | ✅ | ❌ | ❌ | ❌ |
| View Audit Log | ✅ | ✅ | ❌ | ❌ |

**Test Viewer Permissions**:
1. Create a viewer account: username `viewer1`, password `view123`, role `Viewer`
2. Logout and login as `viewer1`
3. Go to Inventory
4. Verify: "Add Item" button should still appear (permission check happens on submit)
5. Try to add an item - nothing should happen (permission denied silently)
6. Verify: Edit/Delete buttons on existing items won't trigger actions

**Note**: The UI currently doesn't hide disabled buttons - this is a UX improvement opportunity.

---

## Phase 2: Comprehensive Audit Trail

### Test 2.1: View Audit Log
**Expected**: Audit log displays all system activities.

**Steps**:
1. Login as admin
2. Click "Audit Log" button

**Expected Result**:
- Table with columns:
  - Timestamp (formatted date/time)
  - User (username who performed action)
  - Action (color-coded)
  - Entity (type and ID)
  - Details (description)
- "Export to CSV" button at top

**Color Coding**:
- 🔴 **Red** (Delete actions): ItemDeleted, NoteDeleted, UserDeleted, DataCleared
- 🟢 **Green** (Create actions): ItemCreated, NoteCreated, UserCreated
- 🔵 **Blue** (Update actions): ItemUpdated, NoteUpdated, UserUpdated, SettingsChanged
- ⚪ **Gray** (Other): UserLogin, UserLogout, DataExported, DataImported

---

### Test 2.2: Login/Logout Auditing
**Expected**: All login and logout events are logged.

**Steps**:
1. Logout if logged in
2. Login as admin
3. Navigate to Audit Log
4. Verify newest entry shows:
   - Action: "User Login" (gray)
   - User: admin
   - Details: "User logged in successfully"
   
5. Click Logout
6. Login again and check Audit Log
7. Verify second-to-last entry shows:
   - Action: "User Logout" (gray)
   - User: admin
   - Details: "User logged out"

---

### Test 2.3: Inventory Item Auditing
**Expected**: All item create/update/delete operations are logged.

**Steps**:
1. Login as admin
2. Go to Inventory
3. Click "Add Item"
4. Fill form:
   - Name: `Test Product`
   - SKU: `TEST-001`
   - Category: `Test`
   - Supplier: `Test Supplier`
   - Description: `This is a test item`
   - Quantity: `100`
   - Price: `49.99`
5. Click Submit

**Verify in Audit Log**:
- Action: "Item Created" (green)
- Details: "Created item: Test Product (SKU: TEST-001)"
- Entity: "item [item-id]"

**Test Edit**:
1. Click "Edit" on the test product
2. Change quantity to `200`
3. Change price to `59.99`
4. Submit

**Verify in Audit Log**:
- Action: "Item Updated" (blue)
- Details: "Updated item: Test Product"
- Should track old values and new values internally

**Test Delete**:
1. Click "Delete" on the test product
2. Verify in Audit Log:
   - Action: "Item Deleted" (red)
   - Details: "Deleted item: Test Product (SKU: TEST-001)"

---

### Test 2.4: Note Operations Auditing
**Expected**: Note create and delete operations are logged.

**Steps**:
1. Go to Notes Editor
2. Click "New Note" (or use Ctrl/Cmd+N)
3. Check Audit Log:
   - Action: "Note Created" (green)
   - Details: "Created note: Untitled Note"

4. Edit the note title
5. Delete the note
6. Check Audit Log:
   - Action: "Note Deleted" (red)
   - Details shows the note title

**Note**: Note updates (content changes) are auto-saved but not individually logged to avoid spam.

---

### Test 2.5: User Management Auditing
**Expected**: User create and delete operations are logged.

**Steps**:
1. Login as admin
2. Create a new user: `audituser`, password `test123`, role `User`
3. Check Audit Log:
   - Action: "User Created" (green)
   - Details: "Created user: audituser with role: User"

4. Delete the user
5. Check Audit Log:
   - Action: "User Deleted" (red)
   - Details: "Deleted user: audituser"

---

### Test 2.6: Data Operations Auditing
**Expected**: Export and clear data operations are logged.

**Steps**:
1. Go to Settings
2. Scroll to "Data Management" section
3. Click "Export Data"
4. Check Audit Log:
   - Action: "Data Exported"
   - Details: "Exported all data to JSON file"

5. Click "Clear All Data"
6. Click "Yes, Delete Everything" in confirmation
7. Check Audit Log:
   - Action: "Data Cleared" (red)
   - Details: "Cleared X items and Y notes"

---

### Test 2.7: Export Audit Log to CSV
**Expected**: Audit log can be exported to CSV file.

**Steps**:
1. View Audit Log
2. Click "Export to CSV" button
3. Check your Desktop folder
4. Find file: `audit_log_[timestamp].csv`
5. Open in Excel/Numbers/Sheets

**Expected CSV Format**:
```
ID,Timestamp,User,Action,Entity Type,Entity ID,Details
[uuid],2026-01-05 12:34:56,admin,User Login,user,[user-id],User logged in successfully
...
```

---

### Test 2.8: Audit Log Permissions
**Expected**: Only Managers and Admins can view audit log.

**Steps**:
1. Create a User role account (not Manager/Admin)
2. Login as that user
3. Verify "Audit Log" button is NOT visible in navigation
4. Manually try to switch view (not possible via UI)

**Create Manager Account**:
1. Login as admin
2. Create user: username `manager1`, password `mgr123`, role `Manager`
3. Logout and login as `manager1`
4. Verify "Audit Log" button IS visible
5. Click and verify access to audit log
6. Verify "Users" button is NOT visible (only admins)

---

## Phase 4: Advanced Validation & Error Handling

### Test 4.1: Required Field Validation
**Expected**: Empty required fields show error messages.

**Steps**:
1. Go to Inventory
2. Click "Add Item"
3. Leave all fields empty
4. Click Submit

**Expected Result**:
- Red error banner appears at top of dialog
- Shows: "Name is required"
- Form does NOT close
- Can fix error and resubmit

**Test Each Required Field**:
1. Fill Name, leave SKU empty → Error: "SKU is required"
2. Fill Name & SKU, leave Quantity empty → Error: "Quantity is required"
3. Fill Name & SKU & Quantity, leave Price empty → Error: "Price is required"

---

### Test 4.2: SKU Format Validation
**Expected**: SKU must be alphanumeric with optional hyphens/underscores.

**Steps**:
1. Add item with SKU: `TEST 001` (with space)
2. Expected error: "SKU has invalid format. Expected: alphanumeric characters with optional hyphens or underscores"

**Valid SKUs** (should work):
- `TEST001` ✅
- `TEST-001` ✅
- `TEST_001` ✅
- `test-123-abc` ✅

**Invalid SKUs** (should fail):
- `TEST 001` ❌ (space)
- `TEST@001` ❌ (special char)
- `TEST.001` ❌ (period)
- `A-very-long-sku-that-exceeds-fifty-characters-limit-x` ❌ (>50 chars)

---

### Test 4.3: Duplicate SKU Detection
**Expected**: Cannot create two items with the same SKU.

**Steps**:
1. Create item with SKU: `UNIQUE-001`
2. Submit successfully
3. Try to create another item with SKU: `unique-001` (case insensitive)
4. Expected error: "SKU 'unique-001' already exists"

**Test Edit**:
1. Edit an existing item
2. Try changing its SKU to match another item's SKU
3. Should show duplicate error
4. Changing SKU to its own current value should be allowed

---

### Test 4.4: Quantity Validation
**Expected**: Quantity must be a positive integer.

**Steps**:
1. Add item with Quantity: `abc`
2. Expected error: "Quantity must be a valid integer (got 'abc')"

3. Add item with Quantity: `-5`
4. Expected error: "Quantity must be a valid integer" (can't parse negative)

5. Add item with Quantity: `1000001` (over 1 million)
6. Expected error: "Quantity must be between 0 and 1000000"

**Valid Quantities**:
- `0` ✅
- `1` ✅
- `999999` ✅

---

### Test 4.5: Price Validation
**Expected**: Price must be positive with max 2 decimal places.

**Steps**:
1. Add item with Price: `abc`
2. Expected error: "Price must be a valid decimal number (got 'abc')"

3. Add item with Price: `-10.50`
4. Expected error: "Price must be between 0 and 1000000"

5. Add item with Price: `1000001`
6. Expected error: "Price must be between 0 and 1000000"

7. Add item with Price: `10.999` (3 decimal places)
8. Expected error: "Price can have at most 2 decimal places"

**Valid Prices**:
- `0` ✅
- `0.99` ✅
- `10.50` ✅
- `999999.99` ✅

---

### Test 4.6: Similar Items Warning
**Expected**: System warns about potentially duplicate items.

**Steps**:
1. Create item: Name: `Apple iPhone 14`
2. Start creating new item
3. In Name field, type: `Apple iPhone`
4. After typing, a **yellow warning box** should appear below validation errors
5. Warning shows: "Similar items found:" with up to 3 matches
6. Example: "• Apple iPhone 14 (SKU: XXX)"

**Purpose**: Helps prevent accidental duplicates with slightly different names.

---

### Test 4.7: Name Length Validation
**Expected**: Name must be between 1 and 200 characters.

**Steps**:
1. Try to submit with Name: `` (empty)
2. Error: "Name is required"

3. Try name with 201 characters
4. Error: "Name must be between 1 and 200 characters (got 201)"

---

### Test 4.8: Real-time Error Clearing
**Expected**: Error messages clear when user starts editing.

**Steps**:
1. Trigger any validation error
2. Verify red error banner appears
3. Start typing in any field
4. Error banner should disappear immediately
5. Error reappears if you submit with invalid data again

---

## General Features Testing

### Test G.1: Data Persistence
**Expected**: All data persists between app restarts.

**Steps**:
1. Create some inventory items
2. Create some notes
3. Create a user
4. Close the app completely (Cmd+Q / Quit)
5. Restart the app
6. Login
7. Verify all data is still present:
   - Inventory items ✅
   - Notes ✅
   - Users ✅
   - Audit log entries ✅
   - Settings ✅

**Data Location**: `~/Library/Application Support/com.inventory-app/inventory_data.json` (macOS)

---

### Test G.2: Keyboard Shortcuts
**Expected**: Keyboard shortcuts work throughout app.

**Shortcuts**:
- `Ctrl/Cmd + S` → Save (triggers auto-save)
- `Ctrl/Cmd + 1` → Switch to Inventory view
- `Ctrl/Cmd + 2` → Switch to Notes view
- `Ctrl/Cmd + K` → Toggle Calculator
- `Ctrl/Cmd + N` → Create New Note
- `Ctrl/Cmd + I` → Show About dialog
- `Escape` → Close About dialog

---

### Test G.3: Settings
**Expected**: Settings can be modified and persist.

**Steps**:
1. Go to Settings
2. Toggle "Auto-save enabled" off/on
3. Change "Auto-save interval" to different value
4. Change "Default category"
5. Switch theme (Dark/Light)
6. Toggle "Show loading screen"
7. Restart app
8. Verify all settings persist

---

## Known Issues & Limitations

1. **Permission UI**: Buttons don't visually disable for users without permissions (they just don't trigger actions)
2. **Note Updates**: Individual note content edits are not logged in audit (only create/delete to avoid spam)
3. **User Edit**: "Edit User" button shows "coming soon" message
4. **Import Data**: Import feature requires manual file placement in Downloads folder
5. **Similar Items**: Only checks when typing name, shows max 3 matches
6. **Audit Log**: Limited to last 1000 entries (auto-pruned)

---

## Success Criteria

### Phase 1 ✅
- [x] Login/logout functionality
- [x] Default admin account works
- [x] User CRUD operations
- [x] Role-based navigation (Users/Audit Log buttons)
- [x] All roles work: Admin, Manager, User, Viewer
- [x] Session persists during app use

### Phase 2 ✅
- [x] All operations logged to audit trail
- [x] Audit log viewable by Manager/Admin
- [x] Color-coded action types
- [x] CSV export functionality
- [x] Before/after values tracked for updates
- [x] Audit log persists between restarts
- [x] 1000 entry limit enforced

### Phase 4 (Partial) ✅
- [x] Required field validation
- [x] SKU format validation
- [x] Duplicate SKU detection
- [x] Price validation (positive, 2 decimals, range)
- [x] Quantity validation (positive integer, range)
- [x] Real-time error display
- [x] Error clearing on edit
- [x] Similar items warning

---

## Next Steps

Continue with remaining enterprise features:
- **Phase 3**: Database Migration (JSON → SQLite)
- **Phase 5**: Advanced Search & Filtering
- **Phase 6**: Stock Alerts & Notifications
- **Phase 7**: Reporting & Analytics
- **Phase 8**: Enhanced Import/Export (CSV, Excel)
- **Phase 9**: UI Polish & Batch Operations
- **Phase 10**: Automated Backup & Recovery

---

## Support

For issues or questions:
1. Check `cargo build` for compilation errors
2. Check `cargo run` output for runtime errors
3. Check data file location for corruption
4. Reset by deleting data file (backup first!)

---

**Last Updated**: January 5, 2026
**Version**: 1.0 (Phases 1, 2, 4-partial complete)

