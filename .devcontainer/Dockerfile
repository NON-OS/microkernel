# The NONOS build environment for any machine that runs containers: VS Code,
# Codespaces, or `docker run` on Windows without WSL2, macOS or Linux. It holds
# Nix and nothing else; every tool the build uses comes from flake.lock, so a
# build in here gives the same artifacts as `nix build` on a workstation.

FROM debian:bookworm-slim@sha256:3783cc01769c7b2b1b83a5c5ad96c815348e28ed7da68e2e3687004faa906251

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates curl xz-utils git sudo make \
 && rm -rf /var/lib/apt/lists/*

# A user of its own, which the single-user Nix install wants. Builds run in
# Nix's sandbox, which needs namespaces a default container does not grant, so
# the container runs privileged (devcontainer.json, and `docker run
# --privileged` by hand).
RUN useradd --create-home --shell /bin/bash nonos \
 && echo "nonos ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/nonos \
 && mkdir -m 0755 /nix && chown nonos /nix
USER nonos
WORKDIR /home/nonos

# Nix 2.34.6, the installer held to its sha256; the installer holds the
# release tarball it unpacks to the sha256 inside it.
ARG NIX_INSTALLER_SHA256=bf6d12da4aeaae38ab509dc736df648597bcdee051046ea76d3d53d5b18fa54a
RUN curl -fsSL -o install https://releases.nixos.org/nix/nix-2.34.6/install \
 && echo "${NIX_INSTALLER_SHA256}  install" | sha256sum -c - \
 && sh install --no-daemon \
 && rm install \
 && mkdir -p ~/.config/nix \
 && printf 'experimental-features = nix-command flakes\nsandbox = true\n' > ~/.config/nix/nix.conf

ENV PATH=/home/nonos/.nix-profile/bin:$PATH \
    USER=nonos
