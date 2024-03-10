use crate::schema::*;
use uuid::Uuid;

use diesel::{query_builder::AsChangeset, Insertable, Queryable, Selectable};

type Slug = String;

#[derive(Selectable,Insertable,Queryable,Debug,AsChangeset)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub age: i32
}

#[derive(Selectable,Insertable,Queryable,Debug,AsChangeset)]
pub struct Role {
    pub slug: Slug,
    pub description: Option<String>,
    pub perms: i16
}

#[derive(Selectable,Insertable,Queryable,Debug,AsChangeset)]
pub struct UserRole {
    pub user: Uuid,
    pub role: Slug
}