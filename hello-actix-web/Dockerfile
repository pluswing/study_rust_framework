FROM rust:1.73-bookworm

WORKDIR /api

RUN apt update && \
  apt install -y default-libmysqlclient-dev

RUN cargo install cargo-watch && \
  cargo install diesel_cli --no-default-features --features mysql

COPY ./ /api
RUN cargo install --path .


EXPOSE 8080

CMD ["cargo" "run"]
