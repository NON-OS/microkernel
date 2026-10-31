#!/usr/bin/env bash
# Free the disk of a stock ubuntu-latest runner, about 25 GB, by removing the
# preinstalled toolchains no NONOS job uses. Run before Nix is installed.

set -euo pipefail

df -h /
sudo rm -rf /usr/share/dotnet /opt/ghc /usr/local/lib/android \
    /usr/local/share/boost /usr/share/swift /usr/local/share/powershell \
    /usr/local/lib/node_modules "${AGENT_TOOLSDIRECTORY:-/nonexistent}" || true
sudo docker image prune --all --force >/dev/null || true
df -h /
