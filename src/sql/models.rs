use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Row};
use sqlx::mysql::{MySqlPool, MySqlRow};
use anyhow::Result;

// this struct will use to receive user input

#[derive(Serialize, Deserialize,Debug)]
pub struct Info {
    pub user_id: String,
    pub group_id: String,
}

// this struct will be used to represent database record
#[derive(Serialize, FromRow)]
pub struct GroupID {
    pub group_id: Vec<String>,

}

// implementation of Actix Responder for Todo struct so we can return Todo from action handler
// impl Responder for GroupID {
//     type Error = Error;
//     type Future = Ready<Result<HttpResponse, Error>>;
//
//     fn respond_to(self, _req: &HttpRequest) -> Self::Future {
//         let body = serde_json::to_string(&self).unwrap();
//         // create response and set content type
//         ready(Ok(
//             HttpResponse::Ok()
//                 .content_type("application/json")
//                 .body(body)
//         ))
//     }
// }

// Implementation for Todo struct, functions for read/write/update and delete todo from database
impl GroupID {
    pub async fn find_all_group_id(pool: &MySqlPool) -> Result<GroupID, sqlx::Error> {

        let recs: Vec<MySqlRow> = sqlx::query(
            r#"
                SELECT distinct group_id  FROM qqmessage
                ORDER BY group_id
            "#
        )
            .fetch_all(pool)
            .await?;

        let mut group_ = GroupID{
            group_id: vec![]
        };

        for rec in recs {
            let group: String = rec.try_get("group_id")?;
            group_.group_id.push(group)
        }

        Ok(group_)
    }
}


#[derive(Serialize, FromRow)]
pub struct UserTalk {
    pub user_id: String,
    pub messages: Vec<Vec<String>>,


}

// impl Responder for UserTalk {
//     type Error = Error;
//     type Future = Ready<Result<HttpResponse, Error>>;
//
//     fn respond_to(self, _req: &HttpRequest) -> Self::Future {
//         let body = serde_json::to_string(&self).unwrap();
//         // create response and set content type
//         ready(Ok(
//             HttpResponse::Ok()
//                 .content_type("application/json")
//                 .body(body)
//         ))
//     }
// }

impl UserTalk {
    pub async fn find_all_talk(user_id: String, group_id: String, pool: &MySqlPool) -> Result<UserTalk, sqlx::Error> {
        let sql = match group_id.as_str() {
            "not" => format!("
                SELECT content,format_time,group_id FROM qqmessage
                where user_id ={} order by time desc
            ", user_id),
            _ => format!("
                SELECT content,format_time,group_id FROM qqmessage
                where user_id ={} and group_id ={} order by time desc
            ", user_id, group_id),
        };


        let recs: Vec<MySqlRow> = sqlx::query(sql.as_str()).fetch_all(pool).await?;

        let mut talk = UserTalk {
            user_id: user_id,
            messages: vec![],
        };
        let iter = recs.iter().map(|row| {
            let content: String = row.try_get("content").unwrap();
            let gid: String = row.try_get("group_id").unwrap();
            let time: String = row.try_get("format_time").unwrap();
            let mut h =Vec::new();
            h.push(content);
            h.push(gid);
            h.push(time);

            h
        }).collect::<Vec<Vec<String>>>();

        talk.messages.extend(iter);

        Ok(talk)
    }
}