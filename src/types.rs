use warp::http::{HeaderMap, Method, Version};
use bytes::Bytes;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: Method,
    pub headers: HeaderMap,
    pub cookies: Option<String>,
    pub path: String,
    pub query: String,
    pub body: Bytes,
    pub remote_addr: Option<SocketAddr>,
}
