use std::io::Write;

use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::Pg;
use diesel::serialize;
use diesel::{prelude::*, serialize::ToSql};

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromSqlRow, AsExpression)]
#[diesel(sql_type=crate::schema::sql_types::Role)]
pub enum Role {
    Admin,
    Teacher,
    Student,
}

impl ToSql<crate::schema::sql_types::Role, Pg> for Role {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match *self {
            Role::Admin => out.write_all(b"admin")?,
            Role::Teacher => out.write_all(b"teacher")?,
            Role::Student => out.write_all(b"student")?,
        }
        Ok(serialize::IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::Role, Pg> for Role {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"admin" => Ok(Role::Admin),
            b"teacher" => Ok(Role::Teacher),
            b"student" => Ok(Role::Student),
            _ => Err("Unrecognized Roles Enum variant".into()),
        }
    }
}

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub role: Role,
}

#[derive(Insertable, Clone, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub email: String,
    pub role: Role,
}
