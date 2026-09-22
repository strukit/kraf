use super::TransportHeaders;
use super::TransportParams;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportRequestMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Query,
}

pub struct TransportRequest<Params, Headers> {
    pub method: TransportRequestMethod,
    pub params: Option<TransportParams<Params>>,
    pub headers: Option<TransportHeaders<Headers>>,
}
