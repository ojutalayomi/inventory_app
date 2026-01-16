use iced::Task;
use crate::{InventoryApp, Message};
use crate::audit::{AuditAction, AuditEntry};

impl InventoryApp {
    pub fn handle_username_changed(&mut self, value: String) {
        self.username_input = value;
        self.login_error = None;
    }

    pub fn handle_password_changed(&mut self, value: String) {
        self.password_input = value;
        self.login_error = None;
    }

    pub fn handle_attempt_login(&mut self) -> Task<Message> {
        self.logging_in = true;
        
        if let Some(session) = self
            .auth_store
            .authenticate(&self.username_input, &self.password_input)
        {
            // Log successful login
            let audit_entry = AuditEntry::new(
                session.user_id.clone(),
                session.username.clone(),
                AuditAction::UserLogin,
                "user".to_string(),
                Some(session.user_id.clone()),
                "User logged in successfully".to_string(),
            );
            self.audit_log.add_entry(audit_entry);
            
            self.session = Some(session);
            self.username_input.clear();
            self.password_input.clear();
            self.login_error = None;
            self.logging_in = false;
            self.state = crate::AppState::Loaded;
            
            // Check for updates on login and auto-save
            let update_checker = self.update_checker.clone();
            let check_update_task = Task::perform(
                async move {
                    update_checker.check_for_updates().await
                },
                Message::UpdateCheckComplete,
            );
            return Task::batch(vec![self.auto_save(), check_update_task]);
        } else {
            self.login_error = Some("Invalid username or password".to_string());
            self.logging_in = false;
        }
        Task::none()
    }

    pub fn handle_login_success(&mut self) {
        self.state = crate::AppState::Loaded;
    }

    pub fn handle_login_failed(&mut self, error: String) {
        self.login_error = Some(error);
    }

    pub fn handle_logout(&mut self) -> Task<Message> {
        use crate::messages::View;
        
        if let Some(session) = &self.session {
            // Log logout
            let audit_entry = AuditEntry::new(
                session.user_id.clone(),
                session.username.clone(),
                AuditAction::UserLogout,
                "user".to_string(),
                Some(session.user_id.clone()),
                "User logged out".to_string(),
            );
            self.audit_log.add_entry(audit_entry);
        }
        self.session = None;
        self.state = crate::AppState::Login;
        self.current_view = View::Inventory;
        self.auto_save()
    }
}

