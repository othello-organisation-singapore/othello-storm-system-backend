use diesel::PgConnection;
use serde_json::{Map, Value};

use crate::database_models::UserRowModel;
use crate::errors::ErrorType;

use super::MetaGenerator;

pub struct UserMetaGenerator {
    user: UserRowModel,
}

impl UserMetaGenerator {
    pub fn from_username(
        username: &String,
        connection: &PgConnection,
    ) -> Result<UserMetaGenerator, ErrorType> {
        let user = UserRowModel::get(username, connection)?;
        Ok(UserMetaGenerator::from_user(user))
    }

    pub fn from_user(user: UserRowModel) -> UserMetaGenerator {
        UserMetaGenerator { user }
    }
}

impl MetaGenerator for UserMetaGenerator {
    fn generate_meta(&self) -> Map<String, Value> {
        let mut meta = Map::new();
        meta.insert(
            String::from("username"),
            Value::from(self.user.username.clone()),
        );
        meta.insert(
            String::from("display_name"),
            Value::from(self.user.display_name.clone()),
        );
        meta.insert(String::from("role"), Value::from(self.user.role.clone()));
        meta
    }
}
