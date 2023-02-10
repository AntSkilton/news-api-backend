mod theme;

use std::{error::Error};
use newsapi::{self, NewsApi, Article, Endpoint, Country};
use dotenv::dotenv;

fn render_articles(_articles: &Vec<Article>) {
    let _theme = theme::default();
    _theme.print_text("# Top Headlines\n");

    for i in _articles{
        _theme.print_text(&format!("{}/{}/{}\n", &i.get_published_at()[8..10], &i.get_published_at()[5..7], &i.get_published_at()[0..4]));
        _theme.print_text(&format!("`{}`\n", i.get_title()));
        _theme.print_text(&format!("*{}* \n", i.gert_url()));
        _theme.print_text("--- \n\n")
    }
}

#[tokio::main] // This allows main() to be async
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?; // Loads the environment variable.
    let _api_key = std::env::var("API_KEY")?; //For local testing only

    let mut newsapi: NewsApi = NewsApi::new(&_api_key);
    newsapi.endpoint(Endpoint::TopHeadlines).country(Country::Us); // Setting from enums

    let newsapi_response = newsapi.fetch_async().await?;
    render_articles(&newsapi_response.get_articles());

    Ok(())
}