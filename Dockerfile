FROM rustlang/rust:nightly

RUN cargo install flamegraph
RUN cargo install cargo-cov

WORKDIR /usr/src/waterpouring
COPY . .

CMD ["cargo", "flamegraph", "--bin", "waterpouring-rec"]
