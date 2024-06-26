FROM rust:buster as base

ENV USER=root

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch
COPY . /code

FROM base as development
EXPOSE 8000
CMD [ "cargo", "run", "--offline"]

FROM base as builder 
RUN cargo build --release --offline

FROM debian:buster-slim as production
EXPOSE 8000
COPY --from=builder /code/target/release/icp-grader-be /icp-grader-be
CMD ["/icp-grader-be"]
