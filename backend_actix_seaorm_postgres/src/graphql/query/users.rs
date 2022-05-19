use async_graphql::{Context, Object, Result};
use entity::{async_graphql, users};
use sea_orm::EntityTrait;
use sea_orm::{entity::*, query::*};

use crate::db::Database;

#[derive(Default)]
pub struct UsersQuery;

#[Object]
impl UsersQuery {
    async fn get_users(&self, ctx: &Context<'_>) -> Result<Vec<users::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(users::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_user_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<users::Model>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(users::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_user_id(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> Result<Option<i32>> {
        let db = ctx.data::<Database>().unwrap();

        if let Some(user) = users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .filter(users::Column::HashedPassword.eq(password))
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?
        {
            Ok(Some(user.id))
        } else {
            Ok(None)
        }
    }
}