FROM rust:slim-trixie AS builder
WORKDIR /usr/src/workingtitle-backend
COPY . .
RUN cargo install --path .

CMD ["workingtitle-backend"]
