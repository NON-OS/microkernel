# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

"""Phase 6: every check the boot will make, run against what was written,
before the seal calls the image ready. The same five checks the make build
ended with (A to E), and one more the flake makes possible: the kernel and
loader just sealed are byte for byte what `nix build` now gives from the tree.
Then the release record."""

import hashlib
import json
import os
import subprocess

from . import capsules, chain, keys
from .run import nix_build, say


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def ledger():
    """Re-stamps nonos-data/trust/MANIFEST.sha256 over the set just written,
    in a fixed order, so two stamps of one tree are identical."""
    trust = keys.TRUST
    files = sorted(os.path.relpath(os.path.join(d, n), trust) for sub in ("capsules", "keys", "policy")
                   for d, _, ns in os.walk(os.path.join(trust, sub)) for n in ns)
    with open(os.path.join(trust, "MANIFEST.sha256"), "w") as f:
        f.writelines(f"{sha256(os.path.join(trust, p))}  {p}\n" for p in files)
    keys.stage(os.path.join(trust, "MANIFEST.sha256"))


def checks(step, tools, capsules_out, elf, kernel_trailer, efi, loader_trailer):
    entries = capsules.catalogue()
    say("  [A] trust ledger")
    subprocess.run(["sha256sum", "-c", "--quiet", "MANIFEST.sha256"], cwd=keys.TRUST, check=True)
    say("  [B] manifest signatures, Ed25519 and ML-DSA-65, under the trust anchor policy")
    for e in entries:
        step.run(tools.sign, "verify-manifest", "--manifest", e["manifest"], "--cert", e["cert"],
                 "--policy", capsules.POLICY, "--elf", capsules.elf(capsules_out, e))
    say("  [C] each binary's declared capabilities against its signed manifest")
    for e in entries:
        step.run("python3", "scripts/check_declared_caps.py", capsules.elf(capsules_out, e),
                 "--manifest-caps", e["required_caps"], "--allow-missing")
    say("  [D] every STARK membership proof, by the gate the kernel and loader run")
    step.run(tools.enroll, "verify", capsules.CAPSULE_ROOT,
             *[f"{e['required_caps']}:{capsules.elf(capsules_out, e)}:{e['trailer']}" for e in entries])
    step.run(tools.enroll, "verify-kernel", chain.KERNEL_ROOT, elf, kernel_trailer)
    step.run(tools.enroll, "verify-bootloader", chain.LOADER_ROOT, efi, loader_trailer)


def reproduced(step, attr, elf, efi, capsules_out):
    """The tree now holds everything the seal wrote. A fresh build from it must
    give the kernel and loader that were sealed, or the release would not be
    what its source says."""
    say("  [E] the capsules the kernel embeds against the ones signed")
    rebuilt = nix_build(step, "capsules")
    if os.path.realpath(rebuilt) != os.path.realpath(capsules_out):
        moved = [e["slug"] for e in capsules.catalogue()
                 if sha256(capsules.elf(rebuilt, e)) != sha256(capsules.elf(capsules_out, e))]
        if moved:
            raise SystemExit("  these capsules changed under the seal, so the image would carry bytes their "
                             f"signed manifests do not name and the kernel would refuse them: {' '.join(moved)}")
    say("  [F] the sealed kernel and loader against a fresh nix build of the tree")
    for out, name, sealed in ((f"{attr}.kernel", "nonos-kernel", elf), (f"{attr}.loader", "nonos_boot.efi", efi)):
        built = os.path.join(nix_build(step, out), name)
        if sha256(built) != sha256(sealed):
            raise SystemExit(f"  {name} from the tree is not the one sealed: something changed under the seal")


def sealed(rel, path):
    """A sealed file by sha256; a sealed directory (the ESP) by every file in
    it, each by sha256, in a fixed order."""
    if not os.path.isdir(path):
        return {"file": rel, "sha256": sha256(path)}
    files = sorted(os.path.relpath(os.path.join(d, n), path) for d, _, ns in os.walk(path) for n in ns)
    return {"dir": rel, "files": {f: sha256(os.path.join(path, f)) for f in files}}


def record(out, cfg, build_manifest, products):
    rel = {k: os.path.relpath(v, out) for k, v in products.items() if v}
    data = {
        "format": "nonos-release/1",
        "config": cfg,
        "build": build_manifest,
        "sealed": {k: sealed(rel[k], v) for k, v in products.items() if v},
        "commit": "the public trust set the seal staged under nonos-data/; commit it so the "
                  "tree builds these artifacts again",
    }
    with open(os.path.join(out, "nonos-release.json"), "w") as f:
        json.dump(data, f, indent=1, sort_keys=True)
        f.write("\n")
