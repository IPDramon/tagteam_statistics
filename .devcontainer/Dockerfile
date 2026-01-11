FROM purtontech/rust-on-nails-devcontainer:1.3.37 AS development

ARG POSTGRES_CLIENT_VERSION=18

COPY ps1.bash .
RUN cat ps1.bash >> ~/.bashrc && sudo rm ps1.bash

COPY .bash_aliases /home/vscode/.bash_aliases

# Install OpenJDK
RUN sudo apt-get update && \
    sudo apt-get install -y openjdk-17-jdk postgresql-client-${POSTGRES_CLIENT_VERSION} && \
    sudo apt-get clean

# Enable our git hooks and set the permisisons on docker sock.
RUN echo 'git config core.hooksPath /workspace/.devcontainer/.githooks' >> ~/.bashrc

# all the volumes configured in the docker-compose.yml
ARG USERNAME=vscode
RUN sudo mkdir -p /workspace/target && sudo chown $USERNAME:$USERNAME /workspace/target
#RUN sudo mkdir -p /workspace/crates/asset-pipeline/node_modules && sudo chown $USERNAME:$USERNAME /workspace/crates/asset-pipeline/node_modules

