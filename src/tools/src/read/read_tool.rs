use super::ReadRequest;
use kraf_core::KrafError;
use kraf_core::Uri;
use kraf_core::filesystem::{DigestAlgorithm, FileSystemProvider, FileSystemService, Hasher};
use kraf_core::transport::{TransportHeaders, TransportResponse, TransportResponseStatus};

pub fn read_stream(
    provider: &dyn FileSystemProvider,
    uri: &Uri,
    algorithm: DigestAlgorithm,
    sink: &mut dyn FnMut(&[u8]) -> Result<(), KrafError>,
) -> Result<String, KrafError> {
    let mut hasher = Hasher::new(algorithm);

    provider.read_chunks(uri, &mut |chunk| {
        hasher.update(chunk);
        sink(chunk)
    })?;

    Ok(hasher.finish())
}

pub fn read_lines(
    provider: &dyn FileSystemProvider,
    uri: &Uri,
    algorithm: DigestAlgorithm,
    start: usize,
    end: usize,
    sink: &mut dyn FnMut(&[u8]) -> Result<(), KrafError>,
) -> Result<String, KrafError> {
    let mut line = 1;

    read_stream(provider, uri, algorithm, &mut |chunk| {
        for segment in chunk.split_inclusive(|byte| *byte == b'\n') {
            if line >= start && line <= end {
                sink(segment)?;
            }

            if segment.ends_with(b"\n") {
                line += 1;
            }
        }

        Ok(())
    })
}

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

    let mut bytes = Vec::new();

    let digest = match read_stream(
        provider,
        uri,
        DigestAlgorithm::default_algorithm(),
        &mut |chunk| {
            bytes.extend_from_slice(chunk);
            Ok(())
        },
    ) {
        Ok(digest) => digest,
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

    let mut headers = TransportHeaders { values: Vec::new() };
    headers.set("digest", digest);

    TransportResponse {
        content,
        content_type: "text/plain".to_string(),
        status: TransportResponseStatus::Ok,
        headers: Some(headers),
    }
}
