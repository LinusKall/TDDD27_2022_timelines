use entity::async_graphql::{self, Context, InputObject, Object, Result, SimpleObject};
use entity::users;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

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
}
