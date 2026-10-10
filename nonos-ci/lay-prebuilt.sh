#!/usr/bin/env bash
# Lay the flake's prebuilt programs where the make lanes read them by path.
#
# With NONOS_CAPSULE_BINS set, make signs the flake's capsules instead of
# building its own (nonos-mk/capsule.mk), so the rules that build the upstream
# tools and the Linux userland never run. The kernel still embeds the tools
# from target/upstream-<tool>/ (src/userspace/tool_capsules), and the store
# takes the files beside each Linux program (python312.zip, perl5, john.conf,
# tcl8.6) from target/linux-userland/. Without them the kernel does not
# compile, or make compiles CPython and perl again inside a boot timeout.
#
# Each file is the flake's, so the kernel embeds the bytes the seal enrolled.
set -euo pipefail
: "${NONOS_CAPSULE_BINS:?NONOS_CAPSULE_BINS names the flake capsules output}"

python3 - "$NONOS_CAPSULE_BINS" <<'EOF'
import json, os, shutil, sys
bins = sys.argv[1]
for e in json.load(open("tools/nix/capsules.json")):
    dest = e.get("prebuilt") or ""
    if not dest.startswith("target/upstream-"):
        continue
    os.makedirs(os.path.dirname(dest), exist_ok=True)
    shutil.copyfile(os.path.join(bins, e["slug"], e["bin"]), dest)
    os.chmod(dest, 0o755)
EOF

mkdir -p target/linux-userland
cp -rL "$(nix build .#linux-userland --no-link --print-out-paths)"/. target/linux-userland/
chmod -R u+w target/linux-userland target/upstream-*
# Newer than every recipe input, so make takes them as built.
find target/linux-userland target/upstream-* -exec touch {} +
echo "lay-prebuilt: upstream tools and Linux userland from the flake"
