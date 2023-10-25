pub trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn call_summarize<T: Summary>(item: &T) -> String {
    item.summarize()
}

pub fn call_boxed_summarize(item: &Box<dyn Summary>) -> String {
    item.summarize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarize_via_bare_struct() {
        let item = NewsArticle {
            headline: String::from("News Headline"),
            location: String::from("Tokyo"),
            author: String::from("yowcow"),
            content: String::from("hoge fuga foo bar"),
        };

        assert_eq!(call_summarize(&item), "News Headline, by yowcow, (Tokyo)",)
    }

    #[test]
    fn summarize_via_boxed_struct() {
        let items: Vec<Box<dyn Summary>> = vec![
            Box::new(NewsArticle {
                headline: String::from("News Headline"),
                location: String::from("Tokyo"),
                author: String::from("yowcow"),
                content: String::from("hoge fuga foo bar"),
            }),
            Box::new(Tweet {
                username: String::from("yowcow"),
                content: String::from("hoge fuga foo bar"),
                reply: false,
                retweet: false,
            }),
        ];

        let item0: &Box<dyn Summary> = items.get(0).unwrap();
        let item1: &Box<dyn Summary> = items.get(1).unwrap();

        assert_eq!(item0.summarize(), "News Headline, by yowcow, (Tokyo)",);
        assert_eq!(
            call_boxed_summarize(item0),
            "News Headline, by yowcow, (Tokyo)",
        );
        assert_eq!(item1.summarize(), "yowcow: hoge fuga foo bar",);
    }
}
