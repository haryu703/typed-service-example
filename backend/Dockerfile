FROM rust:1.43 as builder

WORKDIR /work

RUN rustup component add rustfmt

ADD . .

RUN cargo install --path .


FROM gcr.io/distroless/cc

WORKDIR /app

COPY --from=builder /usr/local/cargo/bin/backend .

CMD [ "./backend" ]
