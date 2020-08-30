mod youtube_parser;

use std::collections::HashSet;
use youtube_parser::YoutubeParser;
use log::debug;

#[derive(Debug)]
pub struct ParseResult {
    pub category: String,
    pub count: u64
}

pub struct MessageParser {
    youtube_parser: YoutubeParser
}

impl MessageParser {
    pub fn new(youtube_api_key: String) -> MessageParser {
        MessageParser {
            youtube_parser: YoutubeParser::new(youtube_api_key)
        }
    }

    pub async fn parse(&self, msg: &String, categories: HashSet<String>)
            -> Option<Vec<ParseResult>> {

        let space_split = msg.split(' ');

        let mut result_vec = Vec::new();

        for s in space_split {
            if msg.contains("youtube.com") {
                match self.parse_youtube_video(
                        &String::from(s),
                        &categories).await {
                    Some(result) => {
                        result_vec.push(result);
                    },
                    None => continue
                }
            } else {
                let results = self.parse_normal_message(
                        &String::from(s),
                        &categories);
                match results {
                    Some(mut r) => {
                        result_vec.append(&mut r);
                        debug!("Normal result: {:?}", result_vec);
                    },
                    _ => (),
                }
            }
        }

        if result_vec.is_empty() {
            None
        } else {
            Some(result_vec)
        }
    }

    fn parse_normal_message(&self, msg: &String, categories: &HashSet<String>)
            -> Option<Vec<ParseResult>> {
        let lowercase_msg = msg.to_lowercase();
        let mut result_vec = Vec::new();

        for category in categories.iter() {
            let split: Vec<&str> = lowercase_msg
                    .split(|c| c == ' ' || c == '.')
                    .collect();

            let mut count: u64 = 0;
            for s in split {
                if s == category {
                    count += 1;
                }
            }

            if count > 0 {
                result_vec.push(ParseResult {
                    category: category.clone(),
                    count: count
                });
            }
        }

        if result_vec.is_empty() {
            None
        } else {
            Some(result_vec)
        }
    }

    async fn parse_youtube_video(&self,
            msg: &String,
            categories: &HashSet<String>) -> Option<ParseResult>{
        let args = msg.split(|c| c == '?' || c == '&');
        let mut video_id: String = String::from("");

        for a in args {
            let a_string = String::from(a);
            let stripped_string = a_string.strip_prefix("v=");

            match stripped_string {
                Some(id) => video_id = String::from(id),
                None => continue
            }
        }

        let channel_name = self.youtube_parser.get_channel_name(video_id)
                .await
                .unwrap_or_default();
        debug!("Channel name: {}", &channel_name);
        debug!("{:?}", &categories);
        if categories.contains(&channel_name) {
            Some(ParseResult {
                category: channel_name,
                count: 1
            })
        } else {
            debug!("Channel not in shill set");
            None
        }
    }
}