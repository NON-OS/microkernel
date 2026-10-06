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

"""Phase 2: every capsule's certificate and signed manifest, then the one
STARK enrollment of the whole set. The arguments are the ones
nonos-mk/capsule.mk has always passed, read from the catalogue make prints
(tools/nix/capsules.json), so the capsules are signed for exactly what they
declare."""

import json
import os
import sys

from . import keys
from .run import say

# The trust anchor policy every certificate is issued under: one epoch and one
# validity window for the whole set.
EPOCH = "1"
VALID_FROM_MS = "1767225600000"
VALID_UNTIL_MS = "1893456000000"
POLICY = f"{keys.TRUST}/policy/nonos_trust_anchor.policy.bin"
CAPSULE_ROOT = f"{keys.TRUST}/policy/zk_capsule_policy_root.bin"


# Whether this seal is a development image's. The catalogue marks the
# development tests dev_only (CAPSULE_DEV_ONLY): a development image signs and
# enrolls them with the rest, and any other image never sees them.
DEVELOPMENT = False


def catalogue():
    with open("tools/nix/capsules.json") as f:
        return [e for e in json.load(f) if DEVELOPMENT or not e["dev_only"]]


def elf(capsules_out, entry):
    return os.path.join(capsules_out, entry["slug"], entry["bin"])


def names_these_anchors(path):
    """Whether the policy file carries this machine's trust anchor keys: it
    holds each anchor's raw public key, the tail of its .pub file."""
    body = open(path, "rb").read()
    return all(open(p, "rb").read()[-32:] in body for p in keys.TA_PUBS.values())


def policy(step, tools, release):
    keys.require(list(keys.TA_SEEDS.values()) + list(keys.TA_PUBS.values()), "the trust anchor policy")
    if os.path.isfile(POLICY):
        if names_these_anchors(POLICY):
            return
        if release:
            sys.exit(f"  {POLICY} names other trust anchor keys than {', '.join(keys.TA_PUBS.values())};\n"
                     "  a release never replaces it silently. If the anchor is meant to change, remove the\n"
                     "  policy and seal again: every capsule is then signed under the new anchor.")
        say(f"  {POLICY} names other anchor keys than this machine's: writing one for these (not a release)")
        os.remove(POLICY)
    step.run(tools.sign, "mk-trust-policy", "--epoch", EPOCH,
             "--ta-pub", f"ed25519={keys.TA_PUBS['ed25519']}", "--ta-pub", f"mldsa65={keys.TA_PUBS['mldsa65']}",
             "--valid-from-ms", VALID_FROM_MS, "--valid-until-ms", VALID_UNTIL_MS, "--out", POLICY)


def sign(step, tools, capsules_out, e):
    keys.require([e["seed_ed25519"], e["seed_mldsa65"], e["pub_ed25519"], e["pub_mldsa65"]], e["slug"])
    caps = [e["caps_ceiling"], e["required_caps"], e["optional_caps"]]
    step.run("python3", "scripts/check_device_secret_cap.py", "--slug", e["slug"], "--caps", *caps)
    nonos_id = step.run(tools.sign, "derive-id", "--handle", e["handle"], "--domain", e["domain"],
                        "--recovery", e["recovery"], capture=True)
    step.run(tools.sign, "sign-id-cert", "--serial", e["serial"], "--nonos-id", nonos_id,
             "--ns-glob", e["namespace"], "--caps-ceiling", e["caps_ceiling"], "--epoch", EPOCH,
             "--valid-from-ms", VALID_FROM_MS, "--valid-until-ms", VALID_UNTIL_MS,
             "--pub-key", f"ed25519={e['pub_ed25519']}", "--pub-key", f"mldsa65={e['pub_mldsa65']}",
             "--ta-seed", f"ed25519={keys.TA_SEEDS['ed25519']}", "--ta-seed", f"mldsa65={keys.TA_SEEDS['mldsa65']}",
             "--metadata", e["metadata"], "--out", e["cert"])
    endpoints = [e["service_endpoint"], e["reply_endpoint"]] + e["instance_endpoints"].split()
    step.run(tools.sign, "sign-manifest", "--cert", e["cert"], "--namespace", e["namespace"],
             "--version", e["version"], "--target", e["target"], "--elf", elf(capsules_out, e),
             "--required-caps", e["required_caps"], "--optional-caps", e["optional_caps"],
             *[a for ep in endpoints for a in ("--endpoint", ep)],
             "--pub-seed", f"ed25519={e['seed_ed25519']}", "--pub-seed", f"mldsa65={e['seed_mldsa65']}",
             "--out", e["manifest"])
    step.run(tools.sign, "verify-manifest", "--manifest", e["manifest"], "--cert", e["cert"], "--policy", POLICY)


def specs(capsules_out):
    """Each capsule as the enroll tool names it: CAPS:elf:trailer."""
    return [f"{e['required_caps']}:{elf(capsules_out, e)}:{e['trailer']}" for e in catalogue()]


def enrolled(step, tools, specs):
    """Whether the root and trailers already in the tree prove these capsules,
    by the check the kernel's spawn gate runs at boot: each trailer against
    this capsule's BLAKE3 and capability word. Proving the set takes hours, and
    a seal that stopped after enrolling should not have to do it again; any
    capsule whose bytes or capabilities changed fails the check, and then the
    whole set is enrolled afresh."""
    trailers = [s.rsplit(":", 1)[1] for s in specs]
    if not os.path.isfile(CAPSULE_ROOT) or not all(os.path.isfile(t) for t in trailers):
        return False
    say("  checking the enrollment already in the tree")
    return bool(step.run(tools.enroll, "verify", CAPSULE_ROOT, *specs, may_fail=True))


def seal(step, tools, capsules_out, release, kept=False):
    """Signs every capsule, then enrolls the set unless `kept` (the caller
    already found the enrollment in the tree proves these capsules) or the
    enrollment in the tree passes the check now."""
    entries = catalogue()
    policy(step, tools, release)
    for e in entries:
        say(f"  signing {e['handle']}")
        sign(step, tools, capsules_out, e)
    named = specs(capsules_out)
    if kept or enrolled(step, tools, named):
        say(f"  kept the enrollment of {len(entries)} capsules: every trailer passes the spawn gate's check "
            "for these exact capsules under the root in the tree")
    else:
        say(f"  enrolling {len(entries)} capsules under one STARK policy root")
        step.run(tools.enroll, "capsules", CAPSULE_ROOT, *named)
    keys.stage(f"{keys.TRUST}/capsules", f"{keys.TRUST}/policy")
