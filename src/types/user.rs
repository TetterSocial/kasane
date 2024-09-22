//! User-related API endpoints and types.
//!
//! Users are the main entities on Tetter. They can post, reply, and interact with other users.
//!
//! Users can also be rated, replied to, and saved.
//!
//! To interact with Tetter, you need a user account, and get the token from the user account.

use super::{deserialize_ts_from_opt_i64, serialize_ts_to_opt_i64};
use serde::{Deserialize, Serialize};

use crate::{call_api, Client};

use super::ApiResult;

/// A user on Tetter
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// User ID,
    /// is a Firebase UID
    #[serde(default)]
    pub id: String,
    /// User's display name
    #[serde(default)]
    pub username: String,

    /// Creation of user account
    #[serde(deserialize_with = "deserialize_ts_from_opt_i64")]
    #[serde(serialize_with = "serialize_ts_to_opt_i64")]
    // skip if not present
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    /// User's URL to their avatar
    #[serde(default)]
    pub avatar_url: Option<String>,

    /// User's role
    #[serde(default)]
    pub role: Option<Role>,

    /// User's friend count
    #[serde(default)]
    pub friends_count: Option<i32>,

    /// User's post count
    #[serde(default)]
    pub posts_count: Option<i32>,
}

impl User {
    /// Get a user by their username.
    pub async fn get_by_username(client: &Client, username: &str) -> ApiResult<Self> {
        let res = call_api(
            client,
            reqwest::Method::GET,
            &format!("users/byUsername/{username}"),
            None,
            None,
        )
        .await?;
        let user = res.json::<User>().await?;
        Ok(user)
    }

    /// Get a user by their ID.
    pub async fn get_by_id(client: &Client, id: &str) -> ApiResult<Self> {
        let res = call_api(
            client,
            reqwest::Method::GET,
            &format!("users/byId/{id}"),
            None,
            None,
        )
        .await?;
        let user = res.json::<User>().await?;
        Ok(user)
    }

    /// Get the current user.
    /// Quirk: This uses /accounts/me instead of /users/{username}
    pub async fn get_current_user(client: &Client) -> ApiResult<Self> {
        let res = call_api(client, reqwest::Method::GET, "accounts/me", None, None).await?;
        let user = res.json::<User>().await?;
        Ok(user)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// Members of the site
    Member,
    /// Tetter moderators
    Moderator,
    /// Site administrators
    Admin,
    /// The owner of the site (Ucrash!)
    Owner,
}
