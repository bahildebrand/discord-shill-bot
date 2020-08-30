use std::collections::HashSet;

pub struct MessageParser {
    pub youtube_api_key: String
}

impl MessageParser {
    pub async fn parse(&self, msg: &String, categories: HashSet<String>) {
        let lowercase_msg = msg.to_lowercase();

        for category in categories.iter() {
            let split: Vec<&str> = lowercase_msg
                    .split(|c| c == ' ' || c == '.')
                    .collect();
            for s in split {
                let mut count: u64 = 0;

                if s == category {
                    count += 1;
                }

            }
        }
    }
}