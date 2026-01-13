use iced::Task;
use crate::{InventoryApp, Message};
use crate::user::UserRole;
use crate::audit::{AuditAction, AuditEntry};

impl InventoryApp {
    pub fn handle_new_username_changed(&mut self, value: String) {
        self.new_username_input = value;
        self.user_operation_error = None;
    }

    pub fn handle_new_password_changed(&mut self, value: String) {
        self.new_password_input = value;
        self.user_operation_error = None;
    }

    pub fn handle_new_role_changed(&mut self, role: UserRole) {
        self.new_role_input = Some(role);
        self.user_operation_error = None;
    }

    pub fn handle_create_user(&mut self) -> Task<Message> {
        if let Some(role) = self.new_role_input {
            if !self.new_username_input.is_empty() && !self.new_password_input.is_empty() {
                match self.auth_store.add_user(
                    self.new_username_input.clone(),
                    &self.new_password_input,
                    role,
                ) {
                    Ok(user) => {
                        // Log user creation
                        if let Some(session) = &self.session {
                            let audit_entry = AuditEntry::new(
                                session.user_id.clone(),
                                session.username.clone(),
                                AuditAction::UserCreated,
                                "user".to_string(),
                                Some(user.id.clone()),
                                format!(
                                    "Created user: {} with role: {:?}",
                                    user.username, user.role
                                ),
                            );
                            self.audit_log.add_entry(audit_entry);
                        }
                        
                        self.new_username_input.clear();
                        self.new_password_input.clear();
                        self.new_role_input = None;
                        self.user_operation_error = None;
                        return self.auto_save();
                    }
                    Err(e) => {
                        self.user_operation_error = Some(e);
                    }
                }
            } else {
                self.user_operation_error =
                    Some("Username and password are required".to_string());
            }
        } else {
            self.user_operation_error = Some("Please select a role".to_string());
        }
        Task::none()
    }

    pub fn handle_edit_user(&mut self, _user_id: String) {
        // TODO: Implement user editing dialog
        self.user_operation_error = Some("User editing coming soon".to_string());
    }

    pub fn handle_delete_user(&mut self, user_id: String) -> Task<Message> {
        let username = self
            .auth_store
            .get_user(&user_id)
            .map(|u| u.username.clone());
            
        match self.auth_store.delete_user(&user_id) {
            Ok(_) => {
                // Log user deletion
                if let Some(session) = &self.session {
                    if let Some(uname) = username {
                        let audit_entry = AuditEntry::new(
                            session.user_id.clone(),
                            session.username.clone(),
                            AuditAction::UserDeleted,
                            "user".to_string(),
                            Some(user_id.clone()),
                            format!("Deleted user: {}", uname),
                        );
                        self.audit_log.add_entry(audit_entry);
                    }
                }
                
                self.user_operation_error = None;
                return self.auto_save();
            }
            Err(e) => {
                self.user_operation_error = Some(e);
            }
        }
        Task::none()
    }

    pub fn handle_user_operation_result(&mut self, result: Result<(), String>) {
        if let Err(e) = result {
            self.user_operation_error = Some(e);
        } else {
            self.user_operation_error = None;
        }
    }
}

