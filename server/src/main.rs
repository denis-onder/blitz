#[macro_use]
extern crate rocket;

mod translators;
mod types;

use translators::translate_reddit_post_to_post;
use types::{Feed, Post, Source};

#[get("/fetch/<source>/<target>")]
async fn fetch_feed(source: String, target: String) -> String {
    if source != Source::Reddit.as_str() {
        return "".to_string();
    }

    let url = format!("https://www.reddit.com/r/{}.json", target);
    let resp = reqwest::get(&url).await.unwrap();
    let body = resp.text().await.unwrap();

    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    let reddit_posts = json["data"]["children"].as_array().unwrap().to_vec();

    let mut posts: Vec<Post> = Vec::new();

    for reddit_post in reddit_posts {
        let post = translate_reddit_post_to_post(&reddit_post);
        posts.push(post);
    }

    let feed = Feed {
        name: format!("r/{}", target),
        posts: posts,
    };

    return serde_json::to_string(&feed).unwrap();
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![fetch_feed])
}
