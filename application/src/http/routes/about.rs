use crate::http::Route;
use askama::Template;
use async_trait::async_trait;
use domain::contracts::StaticRepository;
use hyper::{Body, Request, Response};
use infrastructure::repositories::MarkdownStaticRepository;
use presentation::templates::AboutTemplate;

pub struct About {
    repository: Box<dyn StaticRepository>,
}

unsafe impl Send for About {}
unsafe impl Sync for About {}

impl About {
    pub fn new(repository: Box<dyn StaticRepository>) -> Self {
        Self { repository }
    }
}

impl Default for About {
    fn default() -> Self {
        Self::new(Box::<MarkdownStaticRepository>::default())
    }
}

#[async_trait]
impl Route for About {
    fn method(&self) -> String {
        "GET".to_string()
    }

    fn path(&self) -> String {
        "".to_string()
    }

    async fn handle(&self, _request: Request<Body>) -> Response<Body> {
        let article = self.repository.get("about");
        let template = AboutTemplate::new(article, self.path());

        Response::new(template.render().unwrap().into())
    }
}
