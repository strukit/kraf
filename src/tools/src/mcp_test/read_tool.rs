use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use kraf_core::Uri;
use kraf_core::filesystem::{FileSystemLocal, FileSystemService};
use kraf_core::transport::{TransportParams, TransportRequest, TransportRequestMethod};

use crate::read::{ReadOptions, ReadParams, read};

#[derive(Deserialize, JsonSchema)]
pub struct ReadArgs {
    pub uri: String,
}

#[derive(Serialize)]
struct ReadResult {
    status: u16,
    digest: Option<String>,
    line_count: usize,
    content: String,
}

#[derive(Clone)]
pub struct ReadTool;

#[tool_router(server_handler)]
impl ReadTool {
    #[tool(
        description = "Read a file through kraf's read tool. Returns a JSON object with \
                        `status` (HTTP-style code), `digest` (sha256 of the content, for \
                        change detection), `line_count`, and `content`."
    )]
    fn read(&self, Parameters(ReadArgs { uri }): Parameters<ReadArgs>) -> String {
        let uri = Uri::parse(&uri).expect("invalid uri");

        let mut service = FileSystemService::new();
        service.register("file", Box::new(FileSystemLocal));

        let request = TransportRequest {
            method: TransportRequestMethod::Get,
            params: Some(TransportParams {
                values: ReadParams {
                    uri,
                    options: ReadOptions { partial: false },
                },
            }),
            headers: None,
        };

        let response = read(&service, request);
        let digest = response
            .headers
            .as_ref()
            .and_then(|headers| headers.get("digest"))
            .cloned();

        let result = ReadResult {
            status: response.status as u16,
            digest,
            line_count: response.content.lines().count(),
            content: response.content,
        };

        serde_json::to_string(&result).expect("ReadResult is always serializable")
    }
}
