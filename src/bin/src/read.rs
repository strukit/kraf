use std::io::{self, BufRead, Write};

use kraf_core::filesystem::{
    DigestAlgorithm, FileSystemLocal, FileSystemProvider, FileSystemService,
};
use kraf_core::{KrafError, Uri};
use kraf_tools::read::{read_lines, read_stream};

// A line range is 1-based and inclusive. Without one, the whole file is printed.
#[derive(Clone, Copy)]
struct LineRange {
    start: usize,
    end: usize,
}

// Usage: kraf-read [--digest auto|sha2|sha3|none] [uri [start [end]]]
// Without a uri, reads uris from stdin. `auto` picks the algorithm from the hardware.
struct Options {
    algorithm: DigestAlgorithm,
    uri: Option<String>,
    range: Option<LineRange>,
}

fn main() {
    let options = parse_options(std::env::args().skip(1).collect());

    match &options.uri {
        Some(uri) => read_once(uri, options.range, options.algorithm),
        None => run_loop(options.algorithm),
    }
}

fn parse_options(args: Vec<String>) -> Options {
    let mut algorithm = DigestAlgorithm::detect();
    let mut positional = Vec::new();
    let mut args = args.into_iter();

    while let Some(arg) = args.next() {
        if arg == "--digest" {
            let name = args
                .next()
                .unwrap_or_else(|| panic!("--digest needs a value"));
            algorithm = parse_algorithm(&name);
        } else {
            positional.push(arg);
        }
    }

    let uri = positional.first().cloned();
    let range = uri.as_ref().map(|_| parse_range(&positional[1..]));

    Options {
        algorithm,
        uri,
        range: range.flatten(),
    }
}

fn parse_algorithm(name: &str) -> DigestAlgorithm {
    match name {
        "auto" => DigestAlgorithm::detect(),
        "sha2" => DigestAlgorithm::Sha2,
        "sha3" => DigestAlgorithm::Sha3,
        "none" => DigestAlgorithm::None,
        other => panic!("unknown digest: {other}"),
    }
}

fn parse_range(args: &[String]) -> Option<LineRange> {
    let start = args
        .first()?
        .parse()
        .unwrap_or_else(|error| panic!("invalid start line: {error}"));
    let end = args.get(1).map_or(usize::MAX, |end| {
        end.parse()
            .unwrap_or_else(|error| panic!("invalid end line: {error}"))
    });

    Some(LineRange { start, end })
}

fn service() -> FileSystemService {
    let mut service = FileSystemService::new();
    service.register("file", Box::new(FileSystemLocal));
    service
}

fn read_once(uri: &str, range: Option<LineRange>, algorithm: DigestAlgorithm) {
    let uri = Uri::parse(uri).unwrap_or_else(|error| panic!("invalid uri: {error}"));

    print_stream(&service(), &uri, range, algorithm);
}

fn run_loop(algorithm: DigestAlgorithm) {
    let service = service();
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

        print_stream(&service, &uri, None, algorithm);
    }
}

fn print_stream(
    service: &FileSystemService,
    uri: &Uri,
    range: Option<LineRange>,
    algorithm: DigestAlgorithm,
) {
    let provider = match service.resolve(uri) {
        Ok(provider) => provider,
        Err(error) => {
            eprintln!("{}", error.message);
            return;
        }
    };

    let result = match range {
        Some(range) => print_range(provider, uri, range, algorithm),
        None => print_all(provider, uri, algorithm),
    };

    match result {
        Ok(digest) => eprintln!("digest: {digest}"),
        Err(error) => eprintln!("{}", error.message),
    }
}

fn print_all(
    provider: &dyn FileSystemProvider,
    uri: &Uri,
    algorithm: DigestAlgorithm,
) -> Result<String, KrafError> {
    let mut stdout = io::stdout().lock();

    read_stream(provider, uri, algorithm, &mut |chunk| {
        write_chunk(&mut stdout, chunk)
    })
}

fn print_range(
    provider: &dyn FileSystemProvider,
    uri: &Uri,
    range: LineRange,
    algorithm: DigestAlgorithm,
) -> Result<String, KrafError> {
    let mut stdout = io::stdout().lock();

    read_lines(
        provider,
        uri,
        algorithm,
        range.start,
        range.end,
        &mut |chunk| write_chunk(&mut stdout, chunk),
    )
}

fn write_chunk(stdout: &mut io::StdoutLock<'_>, chunk: &[u8]) -> Result<(), KrafError> {
    stdout.write_all(chunk).map_err(|error| KrafError {
        message: error.to_string(),
    })
}
