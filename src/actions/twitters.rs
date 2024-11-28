use serde::{Deserialize, Serialize};
use std::str::FromStr;
use twitter_v2::{authorization::Oauth1aToken, TwitterApi};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TwitterAuth {
    #[serde(rename = "ConsumerKey")]
    pub consumer_key: String,
    #[serde(rename = "ConsumerSecret")]
    pub consumer_secret: String,
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    #[serde(rename = "AccessTokenSecret")]
    pub access_token_secret: String,
}

impl ToString for TwitterAuth {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl FromStr for TwitterAuth {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let auth: TwitterAuth = serde_json::from_str(s)?;
        Ok(auth)
    }
}

pub async fn post_tweet(auth: &str, text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let twitter_auth = TwitterAuth::from_str(auth)?;
    let oauth1a_token = Oauth1aToken::new(
        twitter_auth.consumer_key,
        twitter_auth.consumer_secret,
        twitter_auth.access_token,
        twitter_auth.access_token_secret,
    );
    let tweet_info = TwitterApi::new(oauth1a_token)
        .post_tweet()
        .text(text.to_string())
        .send()
        .await?;

    Ok(format!(
        "Post tweets {:} success",
        tweet_info.data().unwrap().id
    ))
}
