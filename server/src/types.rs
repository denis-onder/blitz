extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

pub struct Post {
    pub title: String,
    pub link: String,
    pub author: String,
    pub thumbnail: String,
}

#[derive(Serialize, Deserialize)]
pub struct Feed {
    pub name: String,
    pub posts: Vec<Post>,
}

// Source: where feeds and posts originate from (Currently only Reddit)
pub enum Source {
    Reddit,
}

impl Source {
    pub fn as_str(&self) -> &'static str {
        match self {
            Source::Reddit => "reddit",
        }
    }
}
