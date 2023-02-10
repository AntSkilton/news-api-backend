use serde::Deserialize;
use url::Url;

#[cfg(feature = "async")]
use reqwest::Method;

const BASE_URL: &str = "https://newsapi.org/v2";

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Failed fetching articles")]
    RequestFailed(#[from] ureq::Error),

    #[error("Failed converting response to string")]
    FailedResponseToString(#[from] std::io::Error),

    #[error("Article parsing failed")]
    ArticleParseFailed(serde_json::Error),
    
    #[error("URL parsing failed")]
    UrlParsing(#[from] url::ParseError), // The annotation here automatically assigns the error.

    #[error("Request failed: {0}.")]
    BadRequest(&'static str),

    #[cfg(feature = "async")]
    #[error("Async request failed")]
    AsyncRequestFailed(#[from] reqwest::Error),
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)] // As per JSON field from newsapi
pub struct Article {
    title: String,
    url: String,
    publishedAt: String
}

impl Article {
    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn gert_url(&self) -> &str {
        &self.url
    }

    pub fn get_published_at(&self) -> &str {
        &self.publishedAt
    }
}

#[derive(Deserialize)]
pub struct NewsApiResponse {
    status: String,
    pub articles: Vec<Article>,
    code: Option<String>,
}

impl NewsApiResponse {
    pub fn get_articles(&self) -> &Vec<Article> {
        &self.articles
    }
}

pub struct NewsApi {
    api_key: String,
    endpoint: Endpoint,
    country: Country,
}

pub enum Endpoint {
    TopHeadlines,
}

impl ToString for Endpoint {
    // Encapsulation
    fn to_string(&self) -> String {
        match self { // If / Else or Switch on steroids
            Self::TopHeadlines => "top-headlines".to_string()
        }
    }
}

pub enum Country {
    Us,
}

impl ToString for Country {
    // Encapsulation
    fn to_string(&self) -> String {
        match self { // If / Else or Switch on steroids
            Self::Us => "us".to_string()
        }
    }
}

impl NewsApi {
        pub fn new(api_key: &str) -> NewsApi {
            NewsApi {
                api_key: api_key.to_string(),
                endpoint: Endpoint::TopHeadlines,
                country: Country::Us,
            }
        }

        pub fn endpoint(&mut self, endpoint: Endpoint) -> &mut NewsApi {
            self.endpoint = endpoint;
            self
        }

        pub fn country(&mut self, country: Country) -> &mut NewsApi {
            self.country = country;
            self
        }

        fn prepare_url(&self) -> Result<String, NewsApiError> {
            let mut url = Url::parse(BASE_URL)?;
            url.path_segments_mut()
                .unwrap()
                .push(&self.endpoint.to_string());

            let country = format!("country={}", self.country.to_string());
            url.set_query(Some(&country));

            Ok(url.to_string())
        }

        pub fn fetch(&self) -> Result<NewsApiResponse, NewsApiError> {
            let url = self.prepare_url()?;
            let req = ureq::get(&url).set("Authorization", &self.api_key);
            let response: NewsApiResponse = req.call()?.into_json()?; //Serde "derive" and ureq "json" is needed to use into_json()

            match response.status.as_str() {
                "ok" => return Ok(response),
                _ => return Err(map_response_err(response.code)) // Underscore means "else / all other options".
            }
        }

        #[cfg(feature = "async")]
        pub async fn fetch_async(&self) -> Result<NewsApiResponse, NewsApiError> {
            let url = self.prepare_url()?;

            let client = reqwest::Client::new();
            let request = client
                .request(Method::GET, url)
                .header("Authorization", &self.api_key)
                .header("User-Agent", "clinews")
                .build()
                .map_err(|e |NewsApiError::AsyncRequestFailed(e))?;
            
            let response: NewsApiResponse = client
                .execute(request)
                .await?
                .json()
                .await
                .map_err(|e |NewsApiError::AsyncRequestFailed(e))?;      
            
            match response.status.as_str() {
                "ok" => return Ok(response),
                _ => return Err(map_response_err(response.code)) // Underscore means "else / all other options".
            }
        }
    }

fn map_response_err(code: Option<String>) -> NewsApiError {
    if let Some(code) = code {
        match code.as_str() {
            "apiKeyDisabled" => NewsApiError::BadRequest("Your API key has been disabled."),
            _ => NewsApiError::BadRequest("Unknown error")
        }
    }
    else {
        NewsApiError::BadRequest("Unknown error")
    }
}