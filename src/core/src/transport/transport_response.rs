use super::TransportHeaders;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum TransportResponseStatus {
    Ok = 200,
    NotFound = 404,
    NotModified = 304,
    InternalServerError = 500,
}

pub struct TransportResponse<Headers> {
    pub content: String,
    pub content_type: String,
    pub status: TransportResponseStatus,
    pub headers: Option<TransportHeaders<Headers>>,
}
