trait Summary {
    fn summarize(&self) -> String;
}
struct NewsArticle {
    headline: String,
    author: String,
    location: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})",
            self.headline,
            self.author,
            self.location
        )
    }
}



struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}



struct BlogPost {
    title: String,
}

impl Summary for BlogPost {
    fn summarize(&self) -> String {
        format!("ブログ記事: {}", self.title)
    }
}
fn notify(item: &impl Summary) {
    println!("速報: {}", item.summarize());
}
fn main () {
    let article = NewsArticle {
        headline: "Rust 1.75 released".to_string(),
        author: "Rust Team".to_string(),
        location: "Internet".to_string(),
    };

    let tweet = Tweet {
        username: "alice".to_string(),
        content: "Rust楽しい！".to_string(),
    };

    let blog_post = BlogPost {
        title: "トレイト完全理解".to_string(),
    };

    notify(&article);
    notify(&tweet);
    notify(&blog_post);
}