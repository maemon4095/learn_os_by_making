FROM debian:trixie

ARG DEV_USER_NAME=dev
ARG DEV_USER_UID=1000
ARG DEV_USER_GID=${DEV_USER_UID}
ENV HOME="/home/${DEV_USER_NAME}"
ENV RUSTUP_HOME="${HOME}/.cargo"
ENV CARGO_HOME=${RUSTUP_HOME}

# create user
RUN groupadd --gid ${DEV_USER_GID} ${DEV_USER_NAME} \
  && useradd --uid ${DEV_USER_UID} --gid ${DEV_USER_GID} -m ${DEV_USER_NAME}

# install qemu
RUN apt update -y && apt upgrade -y \
  && apt install -y qemu-system-x86 build-essential curl git \
  && apt clean

# install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path \ 
  && echo 'source ~/.cargo/env' >> $HOME/.bashrc

# set default user
USER ${DEV_USER_NAME}