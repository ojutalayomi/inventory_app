use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    Manager,
    User,
    Viewer,
}

impl UserRole {
    pub fn can_view(&self) -> bool {
        true // All roles can view
    }

    pub fn can_create(&self) -> bool {
        matches!(self, UserRole::Admin | UserRole::Manager | UserRole::User)
    }

    pub fn can_edit(&self) -> bool {
        matches!(self, UserRole::Admin | UserRole::Manager | UserRole::User)
    }

    pub fn can_delete(&self) -> bool {
        matches!(self, UserRole::Admin | UserRole::Manager)
    }

    pub fn can_manage_users(&self) -> bool {
        matches!(self, UserRole::Admin)
    }

    pub fn can_view_audit(&self) -> bool {
        matches!(self, UserRole::Admin | UserRole::Manager)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing, default = "default_password_hash")]
    pub password_hash: String,
    pub role: UserRole,
    pub created_at: i64,
    pub last_login: Option<i64>,
    pub active: bool,
}

fn default_password_hash() -> String {
    // Default hash that will never match any password
    // Users imported without password_hash will need to reset their password
    "$2b$12$XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
        .to_string()
}

impl User {
    pub fn new(
        username: String,
        password: &str,
        role: UserRole,
    ) -> Result<Self, bcrypt::BcryptError> {
        let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
        let now = chrono::Utc::now().timestamp();

        Ok(Self {
            id: uuid::Uuid::new_v4().to_string(),
            username,
            password_hash,
            role,
            created_at: now,
            last_login: None,
            active: true,
        })
    }

    pub fn verify_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password_hash).unwrap_or(false)
    }

    pub fn update_last_login(&mut self) {
        self.last_login = Some(chrono::Utc::now().timestamp());
    }

    pub fn change_password(&mut self, new_password: &str) -> Result<(), bcrypt::BcryptError> {
        self.password_hash = bcrypt::hash(new_password, bcrypt::DEFAULT_COST)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub user_id: String,
    pub username: String,
    pub role: UserRole,
    pub login_time: i64,
}

impl Session {
    pub fn new(user: &User) -> Self {
        Self {
            user_id: user.id.clone(),
            username: user.username.clone(),
            role: user.role,
            login_time: chrono::Utc::now().timestamp(),
        }
    }
}
