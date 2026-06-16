# This script require build arg declaration (docker build --build-arg ...) :
#     - SYSTEM
#     - VERSION
#     - PROJECT_NAME

FROM rust:1.94.1-trixie AS base

FROM base AS builder

ARG SYSTEM
ARG VERSION
ARG PROJECT_NAME

ENV PROJECT_NAME=${PROJECT_NAME}

WORKDIR /build

COPY . .

RUN apt-get update && \
    apt-get install -y libclang-dev bash musl-tools && \
    bash scripts/set-version.sh "${VERSION}" && \
    bash deployment/build.sh "${SYSTEM}"


FROM alpine:3.20 AS runtime

WORKDIR /app

COPY --from=builder /build/output/app app
COPY deployment/entrypoint.sh .

RUN chmod +x entrypoint.sh

ENTRYPOINT ["/app/entrypoint.sh"]