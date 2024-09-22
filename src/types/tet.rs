//! Tets
//! 
//! Tets are "posts", only worthy to be seen by the eyes of Kasane Teto.
//! 
//! Tets are the main content of the site, and are the primary way to interact with the site.
//! 
//! Tets can be rated, replied to, and saved.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::{deserialize_ts_from_opt_i64, serialize_ts_to_opt_i64, user::User, ApiResult};

use crate::{call_api, Client};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComposePost {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<uuid::Uuid>,
}

impl ComposePost {
    pub async fn compose(
        client: &Client,
        content: &str,
        parent: Option<uuid::Uuid>,
    ) -> ApiResult<Tet> {
        let data = ComposePost {
            content: content.to_string(),
            parent,
        };

        let res = call_api(
            client,
            reqwest::Method::POST,
            "accounts/composePost",
            Some(serde_json::to_value(&data)?),
            None,
        )
        .await?;
        let tet = res.json::<Tet>().await?;
        Ok(tet)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A JSON schema for rating a post
pub struct RatePost {
    pub post_id: uuid::Uuid,
    pub rating: Rating,
}

impl RatePost {
    pub async fn rate(client: &Client, post_id: uuid::Uuid, rating: Rating) -> ApiResult<()> {
        let data = RatePost { post_id, rating };
        call_api(
            client,
            reqwest::Method::POST,
            "accounts/ratePost",
            Some(serde_json::to_value(&data)?),
            None,
        )
        .await?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
// rename camelcase
#[serde(rename_all = "camelCase")]
/// A post on Tetter, a "Tet" to be exact,
///
/// A Tet is a post that's worthy in the name of Kasane Teto.
pub struct Tet {
    pub id: uuid::Uuid,
    pub content: String,
    // Chrono timedate, data should be serialized from UNIX timestamp
    /// The timestamp of when this post was created.
    /// This should be a UNIX timestamp.
    #[serde(deserialize_with = "chrono::serde::ts_milliseconds::deserialize")]
    #[serde(serialize_with = "chrono::serde::ts_milliseconds::serialize")]
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// The number of Yeah!'s this post has received
    pub likes_count: i32,
    /// The number of Tomatoes thrown at this post
    pub dislikes_count: i32,
    /// Ratio of Yeah!'s to Tomatoes, similar to Reddit's upvote ratio
    pub rating: i32,

    /// The number of replies to this post
    pub replies_count: i32,
    /// The number of times this post has been saved
    pub saves_count: i32,

    /// The client's rating of this post
    pub my_rating: Option<Rating>,

    /// Author of the post
    pub author: Option<User>,

    /// The post's parent, if it has one.
    ///
    /// Should be pointed to a post's ID
    pub parent: Option<Uuid>,

    /// Quoting of the post
    pub quoting: Option<Box<Tet>>,
}

impl Tet {
    /// Get a single post by its ID
    pub async fn get(client: &Client, id: &str) -> ApiResult<Self> {
        let res = call_api(
            client,
            reqwest::Method::GET,
            &format!("posts/post/{id}"),
            None,
            None,
        )
        .await?;
        let tet = res.json::<Tet>().await?;
        Ok(tet)
    }

    /// Get the thread (replies) of a post
    pub async fn get_thread(&self, client: &Client) -> ApiResult<Vec<Thread>> {
        Thread::get(client, &self.id.to_string()).await
    }

    /// Get the author of the post
    ///
    /// While the author field is already present, this method attempts to refetch the full author data using
    /// the author's name.
    pub async fn get_author(&self, client: &Client) -> ApiResult<User> {
        User::get_by_id(client, &self.author.as_ref().unwrap().id).await
    }

    /// Get the parent post of this post, if it had one.
    pub async fn get_parent(&self, client: &Client) -> ApiResult<Option<Tet>> {
        if let Some(parent) = &self.parent {
            let parent = Tet::get(client, &parent.to_string()).await?;
            Ok(Some(parent))
        } else {
            Ok(None)
        }
    }

    /// Creates a new post,
    /// alias to ComposePost:: compose
    pub async fn compose(client: &Client, content: &str) -> ApiResult<Self> {
        ComposePost::compose(client, content, None).await
    }

    /// Reply to this post
    pub async fn reply(&self, client: &Client, content: &str) -> ApiResult<Tet> {
        ComposePost::compose(client, content, Some(self.id)).await
    }
    
    /// Rate this post
    pub async fn rate(&self, client: &Client, rating: Rating) -> ApiResult<()> {
        RatePost::rate(client, self.id, rating).await
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct Thread {
    pub post: Tet,
    pub replies: Vec<Box<Self>>,
}

impl Thread {
    pub async fn get(client: &Client, id: &str) -> ApiResult<Vec<Self>> {
        let res = call_api(
            client,
            reqwest::Method::GET,
            &format!("posts/thread/{id}"),
            None,
            None,
        )
        .await?
        .json::<Vec<Thread>>()
        .await?;

        Ok(res)
    }
}

#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug)]
#[repr(i8)]
pub enum Rating {
    None = 0,
    Yeah = 1,
    Tomato = -1,
}
