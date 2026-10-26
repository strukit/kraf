FROM setup-environment AS setup-workspace

ARG WORKSPACE

WORKDIR $WORKSPACE

COPY rust-toolchain.toml ./

RUN rustup install

COPY --parents **/.cargo .
COPY --parents **/Cargo.toml .
COPY --parents **/Cargo.lock .

RUN cargo fetch --locked

COPY .. .

FROM setup-workspace AS build

RUN cargo build --release

FROM build AS tester

RUN cargo test --workspace

FROM build AS linter

RUN cargo fmt --check
RUN cargo lint
