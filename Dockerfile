# Prepare project within builder image and compile
FROM rust:latest AS builder

RUN USER=root cargo new --bin minerva
WORKDIR ./minerva
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.docker.toml ./Cargo.toml
RUN cargo build --release
RUN rm -r src/

ADD . ./

# Cleanup and build Minerva
RUN rm ./target/release/deps/Minerva*
RUN cargo build --release

# Copy built project and run actual binary
FROM debian:bullseye-slim
ARG APP=/usr/src/app
RUN apt update \
    && apt install -y ca-certificates tzdata libpq5 \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

# Configure timezone, user and certificates
ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

# Copy built application
COPY --from=builder /minerva/target/release/minerva-server ${APP}/minerva
COPY --from=builder /minerva/migrations ${APP}/migrations
COPY --from=builder /minerva/Rocket.toml ${APP}/Rocket.toml
RUN chown -R $APP_USER:$APP_USER ${APP}

# Execute program
USER $APP_USER
WORKDIR ${APP}
ENV ROCKET_PORT=8000

CMD ["./minerva"]


