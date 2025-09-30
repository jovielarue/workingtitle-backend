FROM rust:slim-trixie as builder
WORKDIR /usr/src/workingtitle-backend
COPY . .
RUN cargo install --path .

CMD ["workingtitle-backend"]
