use std::collections::HashMap;

use diesel::PgConnection;

use crate::database_models::UserRowModel;
use super::MetaGenerator;

pub struct UserMetaGenerator {
    user: UserRowModel,
}

impl UserMetaGenerator {
    pub fn from_username(
        username: &String, connection: &PgConnection,
    ) -> Result<UserMetaGenerator, String> {
        let user = UserRowModel::get(username, connection)?;
        Ok(UserMetaGenerator::from_user(user))
    }

    pub fn from_user(user: UserRowModel) -> UserMetaGenerator {
        UserMetaGenerator { user }
    }
}

impl MetaGenerator for UserMetaGenerator {
    fn generate_meta(&self) -> HashMap<String, String> {
        let mut meta: HashMap<String, String> = HashMap::new();
        meta.insert(String::from("username"), self.user.username.clone());
        meta.insert(String::from("display_name"), self.user.display_name.clone());
        meta.insert(String::from("role"), self.user.role.clone());
        meta
    }
}
