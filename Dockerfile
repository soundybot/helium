FROM alpine:latest AS build
WORKDIR /helium
COPY . .
RUN apk add --no-cache curl alpine-sdk openssl-dev
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN source $HOME/.cargo/env && cargo build --release
RUN chmod +x target/release/helium

FROM scratch
COPY --from=build /helium/target/release/helium .
CMD ["/helium"]