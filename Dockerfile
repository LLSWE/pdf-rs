FROM rust:1 AS chef 
RUN cargo install --locked cargo-chef 
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin pdf-rs 

FROM debian:trixie-slim AS runtime
WORKDIR app

ARG USER_ID=1000
ARG GROUP_ID=1000

RUN apt-get update && apt-get install -y \
    chromium \
    ca-certificates \
    libnss3 \
    libx11-6 \
    --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd -g ${GROUP_ID} pdfgroup || true && \
    useradd -l -u ${USER_ID} -g ${GROUP_ID} -m -s /bin/bash pdfuser || truer

RUN mkdir -p /app/storage && chown -R ${USER_ID}:${GROUP_ID} /app

USER pdfuser 

COPY --from=builder /app/target/release/pdf-rs /usr/local/bin

ENV CHROME_BIN=/usr/bin/chromium
ENV CHROME_PATH=/usr/lib/chromium/

ENTRYPOINT ["/usr/local/bin/pdf-rs"]
