pub mod twitters;

use std::{error::Error, str::FromStr};

pub enum Action {
    Unknown,
    PostTweets,
}

impl Action {
    pub async fn execute(&self, auth: &str, input: &str) -> Result<String, Box<dyn Error>> {
        match self {
            Self::PostTweets => twitters::post_tweet(auth, input).await,
            _ => Err("Not in the list of supported actions".into()),
        }
    }
}

impl ToString for Action {
    fn to_string(&self) -> String {
        match self {
            Self::PostTweets => "PostTweets".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PostTweets" => Ok(Action::PostTweets),
            _ => Err("Not in the list of supported actions".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{twitters::TwitterAuth, Action};

    #[tokio::test]
    async fn test_post_tweets() {
        let action = Action::from_str("PostTweets").unwrap();
        let twitter_auth = TwitterAuth {
            consumer_key: "doTulsstaL7Lzgd1JlDHrOUDC".to_string(),
            consumer_secret: "Hh5zJ5ZkbUsw7dKbVnbriD22rhnHikO3PaCXXjuzcaVBkdsHCY".to_string(),
            access_token: "1776074378310217728-XEAnTOScv6bhS4pOv4fepfSNYKCRy8".to_string(),
            access_token_secret: "xmLS8dP4ZFSvm035P3IaKcFsxvajqAr9BTUAtjvGd2zEU".to_string(),
        }
        .to_string();

        match action.execute(twitter_auth.as_str(), "hello,world!").await {
            Ok(response) => println!("Success: {}", response),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
