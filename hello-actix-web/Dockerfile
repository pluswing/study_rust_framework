FROM rust:1.73-bookworm

WORKDIR /api

COPY ./ /api
RUN cargo install --path .

EXPOSE 8080

CMD ["cargo" "run"]
