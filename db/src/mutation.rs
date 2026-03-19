use ::entity::{chatter, chatter::Entity as Chatter};
use ::entity::{chatter_command, chatter_command::Entity as ChatterCommand};
use ::entity::{post, post::Entity as Post};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_post(
        db: &DbConn,
        form_data: post::Model,
    ) -> Result<post::ActiveModel, DbErr> {
        post::ActiveModel {
            title: Set(form_data.title.to_owned()),
            text: Set(form_data.text.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_post_by_id(
        db: &DbConn,
        id: i32,
        form_data: post::Model,
    ) -> Result<post::Model, DbErr> {
        let post: post::ActiveModel = Post::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post::ActiveModel {
            id: post.id,
            title: Set(form_data.title.to_owned()),
            text: Set(form_data.text.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete_post(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let post: post::ActiveModel = Post::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post.delete(db).await
    }

    pub async fn delete_all_posts(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Post::delete_many().exec(db).await
    }

    pub async fn get_or_create_chatter(
        db: &DbConn,
        id: i64,
        name: String,
    ) -> Result<chatter::Model, DbErr> {
        if let Ok(Some(c)) = Chatter::find_by_id(id).one(db).await {
            return Ok(c);
        }

        chatter::ActiveModel {
            id: Set(id),
            name: Set(name),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn set_chatter_tid(db: &DbConn, id: i64, tid: i64) -> Result<chatter::Model, DbErr> {
        let c = Chatter::find_by_id(id).one(db).await?.unwrap();
        let mut c: chatter::ActiveModel = c.into();
        c.tid = Set(tid);
        c.update(db).await
    }

    pub async fn set_chatter_sid(db: &DbConn, id: i64, sid: i64) -> Result<chatter::Model, DbErr> {
        let c = Chatter::find_by_id(id).one(db).await?.unwrap();
        let mut c: chatter::ActiveModel = c.into();
        c.sid = Set(sid);
        c.update(db).await
    }

    pub async fn get_or_create_chatter_command(
        db: &DbConn,
        chatter_id: i64,
        command_id: i64,
    ) -> Result<chatter_command::Model, DbErr> {
        if let Ok(Some(cc)) = ChatterCommand::find_by_id((chatter_id, command_id))
            .one(db)
            .await
        {
            return Ok(cc);
        }

        chatter_command::ActiveModel {
            chatter_id: Set(chatter_id),
            command_id: Set(command_id),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn inc_chatter_command_count(
        db: &DbConn,
        chatter_id: i64,
        command_id: i64,
    ) -> Result<chatter_command::Model, DbErr> {
        let cc = Self::get_or_create_chatter_command(db, chatter_id, command_id).await?;
        let count = cc.count;
        let mut cc: chatter_command::ActiveModel = cc.into();
        cc.count = Set(count + 1);
        cc.update(db).await
    }
}
