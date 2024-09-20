use crate::route::handler::Handler;

#[derive(Debug,Default)]
pub struct Route{
    pub http_method: HttpMethod,
    pub path: String,
    pub handler: Box<dyn Handler>,
}



#[derive(Debug, Default)]
pub enum HttpMethod {
    #[default]
    All,
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Trace,
    Connect,
    Options,
    Before,
    After,
}