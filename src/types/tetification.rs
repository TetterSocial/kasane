//! Tetifications
//! 
//! Tetifications are notifications that are sent to users when certain events occur. These events include:
//! 
//! - Someone Yeahs or throws tomatoes at your post
//! - Someone replies to your post
//! - Someone shouts on your profile



use super::Tet;
use super::{deserialize_ts_from_opt_i64, serialize_ts_to_opt_i64, user::User, ApiResult};
use crate::call_api;
use crate::Client;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TetificationType {
    PostLike,
    PostReply,
    PostDislike,
    ProfileShout,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tetification {
    /// The ID of the tetification
    /// 
    /// This is an incrementing integer that is unique to each tetification for
    /// every user.
    id: i32,
    
    /// The date this tetification was created
    #[serde(deserialize_with = "deserialize_ts_from_opt_i64")]
    #[serde(serialize_with = "serialize_ts_to_opt_i64")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    /// The type of tetification
    #[serde(rename = "type")]
    tetification_type: TetificationType,

    /// Whether the tetification has been read
    pub is_read: bool,

    #[serde(default)]
    pub target_post: Option<Tet>,

    /// The users responsible for the tetification
    pub authors: Vec<User>,

    /// The reply content if the tetification is a reply to a Tet
    #[serde(default)]
    pub reply: Option<Tet>,
}

impl Tetification {
    /// Get current user's tetifications
    pub async fn get(client: &Client) -> ApiResult<Vec<Self>> {
        let res = call_api(
            client,
            reqwest::Method::GET,
            "accounts/me/notifications",
            None,
            None,
        )
        .await?;
        let tetifications = res.json::<Vec<Tetification>>().await?;
        Ok(tetifications)
    }
    
    pub async fn mark_all_read(client: &Client) -> ApiResult<()> {
        call_api(
            client,
            reqwest::Method::POST,
            "accounts/me/notifications/markAllRead",
            None,
            None,
        )
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
    
    #[test]
    fn parse_tetifications() {
        let notif_str = include_str!("../../test/notif.json");
        
        let notif: Vec<Tetification> = serde_json::from_str(notif_str).unwrap();
        
        println!("{:#?}", notif);
    }
}