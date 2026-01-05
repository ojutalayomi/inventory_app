use crate::user::{Session, User, UserRole};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthStore {
    users: HashMap<String, User>,
}

impl Default for AuthStore {
    fn default() -> Self {
        let mut users = HashMap::new();

        // Create default admin user
        if let Ok(admin) = User::new("admin".to_string(), "admin123", UserRole::Admin) {
            users.insert(admin.id.clone(), admin);
        }

        Self { users }
    }
}

impl AuthStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn authenticate(&mut self, username: &str, password: &str) -> Option<Session> {
        let user = self
            .users
            .values_mut()
            .find(|u| u.username == username && u.active)?;

        if user.verify_password(password) {
            user.update_last_login();
            Some(Session::new(user))
        } else {
            None
        }
    }

    pub fn add_user(
        &mut self,
        username: String,
        password: &str,
        role: UserRole,
    ) -> Result<User, String> {
        // Check if username already exists
        if self.users.values().any(|u| u.username == username) {
            return Err("Username already exists".to_string());
        }

        let user = User::new(username, password, role)
            .map_err(|e| format!("Failed to create user: {}", e))?;

        self.users.insert(user.id.clone(), user.clone());
        Ok(user)
    }

    pub fn get_user(&self, user_id: &str) -> Option<&User> {
        self.users.get(user_id)
    }

    pub fn get_all_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }

    pub fn update_user(
        &mut self,
        user_id: &str,
        role: UserRole,
        active: bool,
    ) -> Result<(), String> {
        let user = self.users.get_mut(user_id).ok_or("User not found")?;

        user.role = role;
        user.active = active;
        Ok(())
    }

    pub fn change_password(&mut self, user_id: &str, new_password: &str) -> Result<(), String> {
        let user = self.users.get_mut(user_id).ok_or("User not found")?;

        user.change_password(new_password)
            .map_err(|e| format!("Failed to change password: {}", e))
    }

    pub fn delete_user(&mut self, user_id: &str) -> Result<(), String> {
        // Prevent deleting the last admin
        let admins: Vec<_> = self
            .users
            .values()
            .filter(|u| matches!(u.role, UserRole::Admin) && u.active)
            .collect();

        if admins.len() == 1 && admins[0].id == user_id {
            return Err("Cannot delete the last active admin".to_string());
        }

        self.users.remove(user_id).ok_or("User not found")?;
        Ok(())
    }
}
