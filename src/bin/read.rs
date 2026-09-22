use std::io::{self, BufRead, Write};

use kraf::core::Uri;
use kraf::core::filesystem::{FileSystemLocal, FileSystemService};
use kraf::core::transport::{TransportParams, TransportRequest, TransportRequestMethod};
use kraf::tools::read::{ReadOptions, ReadParams, read};

fn main() {
    match std::env::args().nth(1) {
        Some(uri) => read_once(&uri),
        None => run_loop(),
    }
}

fn read_once(uri: &str) {
    let uri = Uri::parse(uri).unwrap_or_else(|error| panic!("invalid uri: {error}"));

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
    print_response(&response);
}

fn run_loop() {
    let mut service = FileSystemService::new();
    service.register("file", Box::new(FileSystemLocal));

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().ok();

        let mut line = String::new();
        let bytes_read = stdin.lock().read_line(&mut line).unwrap_or(0);
        if bytes_read == 0 {
            break;
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let uri = match Uri::parse(line) {
            Ok(uri) => uri,
            Err(error) => {
                eprintln!("invalid uri: {error}");
                continue;
            }
        };

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
        print_response(&response);
    }
}

fn print_response(response: &kraf::core::transport::TransportResponse<Vec<(String, String)>>) {
    println!("status: {:?}", response.status);
    if let Some(headers) = &response.headers {
        if let Some(digest) = headers.get("digest") {
            println!("digest: {digest}");
        }
    }
    println!("{}", response.content);
}
