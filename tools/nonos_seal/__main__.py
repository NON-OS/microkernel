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

"""nix run .#seal [-- --profile P] [--release]: seal the build into a
bootable image with ek's keys. See __init__.py for the phases."""

import argparse
import json
import os
import subprocess
import sys

from . import capsules, chain, keys, market, media, verify
from .run import Step, nix_build, nix_json, say


class Tools:
    """The host tools the flake built, from NONOS_HOST_TOOLS (the app sets it)."""

    def __init__(self, root):
        bin_ = os.path.join(root, "bin")
        self.sign = os.path.join(bin_, "capsule-sign")
        self.enroll = os.path.join(bin_, "nonos-stark-enroll")
        self.sign_kernel = os.path.join(bin_, "sign-kernel")
        self.embed = os.path.join(bin_, "embed-trailer")
        self.market_index = os.path.join(bin_, "marketplace-index")


def preflight(release):
    if not os.path.isfile("flake.nix") or not os.environ.get("NONOS_HOST_TOOLS"):
        sys.exit("run the seal as `nix run .#seal` from the NONOS checkout")
    dirty = subprocess.run(["git", "status", "--porcelain", "--untracked-files=no"],
                           capture_output=True, text=True, check=True).stdout.splitlines()
    outside = [l for l in dirty if not l[3:].startswith(("nonos-data/trust/", "nonos-data/market/", "nonos-data/models/"))]
    if outside and release:
        sys.exit("  a release seals a committed tree; these files differ from HEAD:\n    " + "\n    ".join(outside))
    keys.guard()


# What the capsules are built from (tools/nix/src.nix, userland): their code
# and the data they embed. Documentation is left out: it changes no byte. The
# Nix recipes are left out too: a recipe change usually changes no capsule
# byte, and the trailer check below is what decides when one did.
CAPSULE_SOURCES = [
    "userland", "toolchain", "third_party/minimp3", "nonos-data", "nonos-bootloader/firmware",
    "nonos-bench", "nonos-device-attest", "nonos-attest-path", "nonos-boot-measure", "nonos-sign",
    "tools/nonos-pack", "LICENSE", "VERSION", "rust-toolchain.toml", ".cargo",
    ":(exclude)nonos-data/trust", ":(exclude,glob)**/*.md",
    # The host proof crates read capsule code; no capsule reads them.
    ":(exclude,glob)userland/*_proofs/**",
    # Phase 1 writes these two and they are committed after the seal, so they
    # always differ from the enrolled commit. Whether the capsules that embed
    # them kept their bytes is what the enrollment check below decides.
    ":(exclude)nonos-data/market", ":(exclude)nonos-data/models",
]


def unchanged_since(commit):
    """Whether no capsule source differs between `commit` and the tree as it
    stands; None when git cannot say (a shallow history)."""
    r = subprocess.run(["git", "diff", "--quiet", commit, "--", *CAPSULE_SOURCES], capture_output=True)
    return {0: True, 1: False}.get(r.returncode)


def current_capsules(step, tools):
    """The capsules to seal. Every capsule build reads nonos-data/trust/COMMIT,
    the commit the set was enrolled at, so writing a new one changes their
    bytes and costs a whole enrollment. When git shows no capsule source has
    changed since that commit, COMMIT stays, and the enrollment in the tree is
    checked against what was built. When a source did change, COMMIT becomes
    HEAD before the one build. Only a recipe change that did change their
    bytes builds them twice."""
    commit = f"{keys.TRUST}/COMMIT"
    recorded = open(commit).read().strip() if os.path.isfile(commit) else ""
    if recorded and unchanged_since(recorded):
        say(f"  no capsule source changed since {recorded[:12]}, their enrolled commit")
        built = nix_build(step, "capsules")
        if capsules.enrolled(step, tools, capsules.specs(built)):
            say("  kept: the enrollment in the tree proves these capsules")
            return built, True
    head = subprocess.run(["git", "rev-parse", "HEAD"], capture_output=True, text=True, check=True).stdout
    with open(commit, "w") as f:
        f.write(head)
    keys.stage(commit)
    say(f"  the capsules changed: enrolling them at {head.strip()[:12]}")
    return nix_build(step, "capsules"), False


def main():
    ap = argparse.ArgumentParser(prog="nonos-seal", description=__doc__)
    ap.add_argument("--profile", help="seal this profile instead of nonos.toml's (nix build .#<profile>)")
    ap.add_argument("--release", action="store_true",
                    help="a release: refuse a dirty tree, a development loader and a missing Secure Boot key")
    ap.add_argument("--out", default="target/release", help="where the sealed image goes")
    ap.add_argument("--mirror", default="", help="the NONOS model repository the model catalogue names")
    args = ap.parse_args()

    attr = args.profile or "default"
    preflight(args.release)
    tools = Tools(os.environ["NONOS_HOST_TOOLS"])
    cfg = nix_json(Step(os.path.join(args.out, "logs")), f"{attr}.cfg")
    out = os.path.abspath(os.path.join(args.out, cfg["name"]))
    work = os.path.join(out, "work")
    os.makedirs(work, exist_ok=True)
    step = Step(os.path.join(out, "logs"))
    if cfg.get("blocked"):
        sys.exit("  " + cfg["blocked"])
    if args.release and not cfg["release"]:
        sys.exit(f"  profile {cfg['profile']} uses the development loader and is never sealed for release")
    serial = market.serial()
    say(f"sealing {cfg['name']}: {cfg['about']}")
    # A development image (nonos-dev-attest): every trailer is the path alone,
    # which only that image's gates take, so it is enrolled in seconds. Its
    # loader is the development one, which a release refuses above.
    path_only = "nonos-dev-attest" in cfg["features"]
    capsules.DEVELOPMENT = not cfg["release"]
    if path_only:
        os.environ["NONOS_ENROLL_PATHS"] = "1"
        say("  a development image: path-only attestation, no STARK proofs, never a release")

    say("[1/6] inputs")
    linux_out = nix_build(step, "linux-userland")
    market.seal(step, tools, linux_out, serial, args.mirror, cfg.get("linux_packages", ""),
                os.path.join(out, "packages"), required=cfg["release"] and not path_only)

    say("[2/6] capsules")
    capsules_out, kept = current_capsules(step, tools)
    capsules.seal(step, tools, capsules_out, args.release, kept)

    say("[3/6] kernel")
    elf, kernel_trailer = chain.kernel(step, tools, attr, work)

    say("[4/6] bootloader")
    efi, loader_trailer = chain.loader(step, tools, attr, work)

    say("[5/6] image")
    kernel_bin = chain.sign_kernel(step, tools, elf, kernel_trailer, cfg["rollback_index"], work, path_only)
    boot_root, approval = chain.records(step, cfg["rollback_index"], work)
    boot_efi = chain.secure_boot(step, efi, args.release and cfg["loader"] == "production")
    esp = media.esp(out, boot_efi, kernel_bin, loader_trailer, boot_root, approval)
    entries = media.store_entries(cfg["store"], capsules_out, linux_out, capsules.catalogue(),
                                  cfg["enabled"], out)
    img = media.usb(step, out, esp, entries)
    iso = media.iso(step, out, esp)

    say("[6/6] verify")
    verify.ledger()
    verify.checks(step, tools, capsules_out, elf, kernel_trailer, efi, loader_trailer)
    step.run("python3", "scripts/build_receipt.py", "--policy-root", capsules.CAPSULE_ROOT,
             "--kernel-attest-root", chain.KERNEL_ROOT, "--bootloader", efi, "--enrolled-elf", elf,
             "--kernel", kernel_bin, "--artifact", iso, "--artifact", img, "--out", os.path.join(out, "build-receipt.json"))
    verify.reproduced(step, attr, elf, efi, capsules_out)
    with open(os.path.join(nix_build(step, attr), "nonos-build.json")) as f:
        build_manifest = json.load(f)
    verify.record(out, cfg, build_manifest, {"esp": esp, "usb": img, "iso": iso, "kernel": kernel_bin,
                                            "bootloader": boot_efi, "kernel.approval": approval})
    say(f"\nsealed {cfg['name']} in {out}")
    say(f"  boot it:   make boot PROFILE={cfg['name']}   (nix run .#qemu -- --profile {cfg['name']} --tpm)")
    say("  commit:    the staged files under nonos-data/ (git status shows them)")


if __name__ == "__main__":
    main()
