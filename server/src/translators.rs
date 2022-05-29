use crate::types::Post;

// Logic that translates the request into a response
pub fn translate_reddit_post_to_post(post: &serde_json::Value) -> Post {
    let title = post["data"]["title"].as_str().unwrap();
    let url = post["data"]["url"].as_str().unwrap();
    let thumbnail = post["data"]["thumbnail"].as_str().unwrap();
    let author = post["data"]["author"].as_str().unwrap();

    Post {
        title: title.to_string(),
        link: url.to_string(),
        author: author.to_string(),
        thumbnail: thumbnail.to_string(),
    }
}
