FROM rust:1.60

WORKDIR /app

COPY /app .

RUN cargo build --release

ENTRYPOINT [ "./target/release/port_scanner" ]

CMD [ "-t", "2000", "192.168.0.1" ]
