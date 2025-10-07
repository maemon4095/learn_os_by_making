FROM debian:trixie

ARG DEV_USER_NAME=dev
ARG DEV_USER_UID=1000
ARG DEV_USER_GID=${DEV_USER_UID}
ARG WORKSPACE

ENV WORKSPACE=${WORKSPACE}
ENV HOME="/home/${DEV_USER_NAME}"
ENV RUSTUP_HOME="${HOME}/.cargo"
ENV CARGO_HOME=${RUSTUP_HOME}
ENV CACHE_DIR="${WORKSPACE}/.cache"

# create user
RUN groupadd --gid ${DEV_USER_GID} ${DEV_USER_NAME} \
  && useradd --uid ${DEV_USER_UID} --gid ${DEV_USER_GID} -m ${DEV_USER_NAME}

# create directory and set permission
RUN mkdir -p ${CACHE_DIR} && chmod -R 777 "${WORKSPACE}"

# install essentials
RUN apt update -y && apt upgrade -y \
  && apt install -y qemu-system-x86 build-essential curl git  xxd bsdextrautils netcat-openbsd

# change user to dev
USER ${DEV_USER_NAME}

# install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
ENV PATH="${HOME}/.cargo/bin:$PATH"
