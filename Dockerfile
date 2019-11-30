FROM rust

RUN cargo install flamegraph
RUN cargo install cargo-cov
RUN rustup component add rust-src

WORKDIR /usr/src/waterpouring
COPY . .

CMD ["cargo", "flamegraph", "--bin", "waterpouring-rec"]
