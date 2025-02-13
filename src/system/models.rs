use std::fmt::Debug;
use std::io::Write;

use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::mysql::Mysql;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Text;
use diesel::{deserialize, serialize};
use serde::{Deserialize, Serialize};

use crate::schema::*;

use crate::system::utils::ModifyUser;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub permissions: UserPermissions,
    pub created: i64,
}

impl User {
    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
    pub fn update(&mut self, update: ModifyUser) {
        if let Some(perms) = update.permissions {
            self.permissions = perms;
        }
        if let Some(name) = update.name {
            self.name = name;
        }
        if let Some(email) = update.email {
            self.email = email;
        }

    }
}

#[derive(AsExpression, Debug, Deserialize, Serialize, FromSqlRow, Clone)]
#[sql_type = "Text"]
pub struct UserPermissions {
    #[serde(default = "default_permission")]
    pub admin: bool,
    #[serde(default = "default_permission")]
    pub deployer: bool,
}

impl UserPermissions {
    pub fn new_owner() -> UserPermissions {
        UserPermissions {
            admin: true,
            deployer: true,
        }
    }
}

fn default_permission() -> bool {
    false
}

impl FromSql<Text, Mysql> for UserPermissions {
    fn from_sql(
        bytes: Option<&<diesel::mysql::Mysql as Backend>::RawValue>,
    ) -> deserialize::Result<UserPermissions> {
        let t = <String as FromSql<Text, Mysql>>::from_sql(bytes)?;
        let result: UserPermissions = serde_json::from_str(t.as_str())?;
        return Ok(result);
    }
}

impl ToSql<Text, Mysql> for UserPermissions {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Mysql>) -> serialize::Result {
        let s = serde_json::to_string(&self)?;
        <String as ToSql<Text, Mysql>>::to_sql(&s, out)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct ForgotPassword {
    pub id: i64,
    pub user: i64,
    pub token: String,
    pub expiration: i64,
    pub created: i64,
}

// Represents a Session of an active user
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct SessionToken {
    pub id: i64,
    pub user: i64,
    pub token: String,
    pub expiration: i64,
    pub created: i64,
}

// Unlike a SessionToken this is a token sent with the users username to be used as a password.
// If the user sets up Google Authentication with their account they will need to generate one of these to do deploys
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct AuthToken {
    pub id: i64,
    pub user: i64,
    pub token: String,
    pub expiration: i64,
    pub created: i64,
}
