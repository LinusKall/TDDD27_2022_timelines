use entity::async_graphql::{self, Context, InputObject, Object, Result, SimpleObject};
use entity::users;
use sea_orm::entity::prelude::*;
use sea_orm::{query::*, ActiveModelTrait, EntityTrait, FromQueryResult, Set};

use crate::db::Database;

#[derive(InputObject)]
pub struct CreateUserInput {
    pub username: String,
    pub email: String,
    pub hashed_password: String,
}

#[derive(SimpleObject)]
pub struct DeleteUserResult {
    pub success: bool,
    pub rows_affected: u64,
}

#[derive(Debug, FromQueryResult, SimpleObject)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Default)]
pub struct UsersMutation;

#[Object]
impl UsersMutation {
    pub async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: CreateUserInput,
    ) -> Result<users::Model> {
        let db = ctx.data::<Database>().unwrap();

        let user = users::ActiveModel {
            username: Set(input.username),
            email: Set(input.email),
            hashed_password: Set(input.hashed_password),
            ..Default::default()
        };

        Ok(user.insert(db.get_connection()).await?)
    }

    pub async fn delete_user(&self, ctx: &Context<'_>, id: i32) -> Result<DeleteUserResult> {
        let db = ctx.data::<Database>().unwrap();

        let res = users::Entity::delete_by_id(id)
            .exec(db.get_connection())
            .await?;

        if res.rows_affected <= 1 {
            Ok(DeleteUserResult {
                success: true,
                rows_affected: res.rows_affected,
            })
        } else {
            unimplemented!()
        }
    }

    pub async fn get_user(&self, ctx: &Context<'_>, user_id: i32) -> Result<Option<UserInfo>> {
        let db = ctx.data::<Database>().unwrap();

        Ok(users::Entity::find()
            .having(users::Column::Id.eq(user_id))
            .select_only()
            .column(users::Column::Id)
            .column(users::Column::Username)
            .column(users::Column::Email)
            .into_model::<UserInfo>()
            .one(db.get_connection())
            .await?)
    }
}
