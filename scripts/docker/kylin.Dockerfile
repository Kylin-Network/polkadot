# build polkadot-node
FROM paritytech/ci-linux:production as builder

LABEL maintainer="kylin-dev"
ARG RUST_VERSION=1.51.0
ARG PROFILE=release
ARG DOT_GIT_REPO="https://github.com/kylin-Network/polkadot.git"
ARG DOT_BRANCH="release-v0.9.7"

RUN rm -rf /usr/local/rustup/toolchains/
RUN rustup default stable
RUN rustup update nightly
RUN rustup target add wasm32-unknown-unknown --toolchain nightly

#Build Polkadot
WORKDIR /builds/
RUN git clone --recursive ${DOT_GIT_REPO}
WORKDIR /builds/polkadot
RUN git checkout ${DOT_BRANCH}
RUN cargo build --${PROFILE}
RUN cp target/${PROFILE}/polkadot /polkadot

ARG SUBKEY_VERSION=2.0.1

# build subkey
RUN cargo install --force subkey --git https://github.com/paritytech/substrate --version ${SUBKEY_VERSION} --locked
RUN cp /usr/local/cargo/bin/subkey /subkey

# build sample data fetcher
#FROM paritytech/ci-linux:production as data_fetcher_builder
ARG DATA_FETECHER_REPO="https://github.com/Kylin-Network/sample-data-fetcher.git"
ARG DATA_FETECHER_BRANCH="master"
ARG PROFILE="release"

WORKDIR /builds/
RUN git clone --recursive ${DATA_FETECHER_REPO}
WORKDIR /builds/sample-data-fetcher
RUN cargo build --${PROFILE}
RUN cp target/${PROFILE}/data_fetcher /data_fetcher

#
# build Polkadot JS apps
#FROM paritytech/ci-linux:production as frontend_builder
#
LABEL maintainer="kylin-dev"
ARG FRONT_REPO="https://github.com/Kylin-Network/apps.git"
ARG FRONT_BRANCH="master"

WORKDIR /builds/
RUN git clone --recursive ${FRONT_REPO}
WORKDIR /builds/apps

RUN apt update && apt install gnupg2 curl ca-certificates -y
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - && echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list
RUN apt update && apt install yarn -y
RUN apt purge nodejs npm -y

RUN apt-get install -y \
    software-properties-common \
    npm

RUN npm install npm@latest -g && \
    npm install n -g && \
    n latest
RUN YARN_CHECKSUM_BEHAVIOR=update yarn && yarn build:www

ARG PROFILE="release"
ENV DEBIAN_FRONTEND noninteractive
RUN apt-get update && apt-get install curl ca-certificates npm nodejs -y
RUN npm install -g serve


