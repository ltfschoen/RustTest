FROM rust:1.65 AS builder
# https://github.com/phusion/baseimage-docker/issues/319#issuecomment-272568689

ARG PROJECT_NAME=${PROJECT_NAME}
RUN apt-get update && apt-get install -y --no-install-recommends apt-utils vim git
WORKDIR /app

RUN mkdir -p /app/projects
COPY ./projects/${PROJECT_NAME} /app/projects/${PROJECT_NAME}

# RUN mkdir -p /app/projects \
#     && cd /app/projects \
#     && cargo new --bin --name=${PROJECT_NAME} /app/projects/${PROJECT_NAME} \
#     && ls -al /app/projects \
#     && ls -al /app/projects/${PROJECT_NAME}/ \
#     && cat /app/projects/${PROJECT_NAME}/Cargo.toml \
#     && cat /app/projects/${PROJECT_NAME}/.git/config \
#     && ls -al /app/projects/${PROJECT_NAME}/src \
#     && cat /app/projects/${PROJECT_NAME}/src/main.rs
# FIXME - it removes the intermediate container so i lose /app/projects/zk

RUN echo 'fn main() { panic!("dummy image called!") }' > /app/projects/${PROJECT_NAME}/src/main.rs
#only for existing projects, but we don't want to have to install cargo on host machine
# COPY ./projects/${PROJECT_NAME}/Cargo.toml /app/projects/${PROJECT_NAME}
# COPY ./projects/${PROJECT_NAME}/Cargo.lock /app/projects/${PROJECT_NAME}
RUN cargo build --manifest-path=/app/projects/${PROJECT_NAME}/Cargo.toml
# COPY ./projects/${PROJECT_NAME}/src /app/projects/${PROJECT_NAME}/src

#need to break the cargo cache
RUN touch /app/projects/${PROJECT_NAME}/src/main.rs
RUN cargo build --manifest-path=/app/projects/${PROJECT_NAME}/Cargo.toml

EXPOSE 8080

# ------------------------------------
FROM debian:buster-slim

ARG PROJECT_NAME=${PROJECT_NAME}

COPY --from=builder /app/projects/${PROJECT_NAME}/target/debug/${PROJECT_NAME} /usr/local/${PROJECT_NAME}

CMD tail -f /dev/null
# CMD ["/usr/local/${PROJECT_NAME}"]
# https://github.com/phusion/baseimage-docker/issues/319
