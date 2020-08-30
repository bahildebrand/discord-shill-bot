use log::{error};


pub struct YoutubeParser {
    youtube_api_key: String
}

impl YoutubeParser {
    pub fn new(youtube_api_key: String) -> YoutubeParser {
        YoutubeParser {
            youtube_api_key: youtube_api_key
        }
    }

    pub async fn get_channel_name(&self, video_id: String) -> Option<String> {
        let video_url = format!("https://www.googleapis.com/youtube/v3/\
                videos?part=snippet&id={}&key={}",
                video_id,
                self.youtube_api_key);

        let res = reqwest::get(&video_url).await;
        match res {
            Err(e) => {
                error!("Failed to get video by id: {}", e);
                None
            }, Ok(resp) => {
                let json_val = json::parse(&resp.
                        text().
                        await.
                        unwrap_or_default()[..]).unwrap();
                let channel = json_val["items"][0]["snippet"]["channelTitle"].to_string();
                Some(channel.to_lowercase())
            }
        }
    }
}