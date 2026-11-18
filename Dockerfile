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

FROM setup-workspace AS builder

RUN cargo build --release

FROM builder AS tester

RUN cargo test --workspace

FROM builder AS linter

RUN cargo fmt --check
RUN cargo lint

FROM builder AS benchmark

ENV KRAF_READ=.dist/cargo/release/kraf-read

RUN microdnf install -y time \
    && microdnf clean all \
    && sh scripts/bench-read.sh
