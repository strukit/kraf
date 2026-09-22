use kraf_core::Uri;
use kraf_core::transport::TransportRequest;

pub struct ReadOptions {
    pub partial: bool,
}

pub struct ReadParams {
    pub uri: Uri,
    pub options: ReadOptions,
}

pub type ReadRequest = TransportRequest<ReadParams, Vec<(String, String)>>;
