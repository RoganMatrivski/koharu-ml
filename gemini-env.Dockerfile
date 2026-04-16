FROM debian:bookworm-slim

ARG NODE_VERSION=22.14.0
ARG PYTHON_VERSION=3.12

ENV PATH=/root/.local/bin:/root/.cargo/bin:/root/.local/share/mise/shims:$PATH

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
  && rm -rf /var/lib/apt/lists/*

# install mise
RUN curl https://mise.run | MISE_INSTALL_PATH=/usr/local/bin/mise sh

# install runtimes + uv
RUN mise use -g rust python@${PYTHON_VERSION} node@${NODE_VERSION} uv \
  && mise reshim

# install gemini-cli
RUN npm install -g @google/gemini-cli

WORKDIR /workspace
CMD ["/bin/bash"]
