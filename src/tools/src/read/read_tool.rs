use super::ReadRequest;
use kraf_core::filesystem::FileSystemService;
use kraf_core::transport::{TransportHeaders, TransportResponse, TransportResponseStatus};

pub fn read(
    service: &FileSystemService,
    request: ReadRequest,
) -> TransportResponse<Vec<(String, String)>> {
    let Some(params) = request.params else {
        return TransportResponse {
            content: "missing params".to_string(),
            content_type: "text/plain".to_string(),
            status: TransportResponseStatus::InternalServerError,
            headers: None,
        };
    };

    let uri = &params.values.uri;

    let provider = match service.resolve(uri) {
        Ok(provider) => provider,
        Err(error) => {
            return TransportResponse {
                content: error.message,
                content_type: "text/plain".to_string(),
                status: TransportResponseStatus::InternalServerError,
                headers: None,
            };
        }
    };

    let bytes = match provider.read_file(uri) {
        Ok(bytes) => bytes,
        Err(error) => {
            return TransportResponse {
                content: error.message,
                content_type: "text/plain".to_string(),
                status: TransportResponseStatus::NotFound,
                headers: None,
            };
        }
    };

    let content = match String::from_utf8(bytes) {
        Ok(content) => content,
        Err(error) => {
            return TransportResponse {
                content: error.to_string(),
                content_type: "text/plain".to_string(),
                status: TransportResponseStatus::InternalServerError,
                headers: None,
            };
        }
    };

    let digest = match provider.digest(uri) {
        Ok(digest) => digest,
        Err(error) => {
            return TransportResponse {
                content: error.message,
                content_type: "text/plain".to_string(),
                status: TransportResponseStatus::InternalServerError,
                headers: None,
            };
        }
    };

    let mut headers = TransportHeaders { values: Vec::new() };
    headers.set("digest", digest);

    TransportResponse {
        content,
        content_type: "text/plain".to_string(),
        status: TransportResponseStatus::Ok,
        headers: Some(headers),
    }
}
