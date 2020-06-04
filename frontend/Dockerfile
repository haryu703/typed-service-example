FROM node:12 as builder

WORKDIR /work

ARG PROTOC_VERSION="3.11.4"
ARG PROTOC_GEN_GRPC_WEB_VERSION="1.0.7"

# Install protoc
RUN curl -L https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/protoc-${PROTOC_VERSION}-linux-x86_64.zip -o protoc.zip && \
    unzip protoc.zip && \
    mv bin/protoc /usr/local/bin && \
    curl -L https://github.com/grpc/grpc-web/releases/download/${PROTOC_GEN_GRPC_WEB_VERSION}/protoc-gen-grpc-web-${PROTOC_GEN_GRPC_WEB_VERSION}-linux-x86_64 -o protoc-gen-grpc-web && \
    chmod +x protoc-gen-grpc-web && \
    mv protoc-gen-grpc-web /usr/local/bin

# Install dependencies
ADD package.json yarn.lock ./
RUN yarn install --frozen-lockfile

# Build app
ADD . .
RUN yarn build

FROM nginx:stable-alpine

COPY --from=builder /work/build /usr/share/nginx/html
