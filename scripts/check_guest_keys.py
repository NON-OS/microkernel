#!/usr/bin/env python3
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
"""Guests.mk mints its scratch publisher pairs under target/, never .keys/ (B-03).

.keys/ holds the operator's own keys; an untracked .pub beside the committed
ones could be committed. Both halves live in $(LINUX_GUESTS_KEYS), in target/.
"""

import argparse
import re
import sys
from pathlib import Path

DIR = "LINUX_GUESTS_KEYS := $(TARGET_DIR)/"
PREFIX = "$(LINUX_GUESTS_KEYS)/guest_$(1)_publisher"


def code(text: str) -> str:
    return "\n".join(line.split("#", 1)[0] for line in text.splitlines())


def problems(text: str) -> list:
    body, out = code(text), []
    if ".keys" in body:
        out.append("Guests.mk names .keys outside a comment")
    if DIR not in body:
        out.append("LINUX_GUESTS_KEYS is not under $(TARGET_DIR)")
    outs = re.findall(r"keygen\s+--alg\s+\S+\s+--out\s+(\S+)", body)
    if not outs or any(not o.startswith("$(LINUX_GUESTS_KEYS)/") for o in outs):
        out.append("a keygen writes outside $(LINUX_GUESTS_KEYS)")
    guest = body[body.find("define LINUX_GUEST") : body.find("include nonos-mk/capsule.mk")]
    for var in ("CAPSULE_KEY_SEED_PREFIX", "CAPSULE_KEY_PUB_PREFIX"):
        if not re.search(rf"^{var}\s*:=\s*{re.escape(PREFIX)}\s*$", guest, re.M):
            out.append(f"LINUX_GUEST does not set {var} under $(LINUX_GUESTS_KEYS)")
    return out


SEED = "CAPSULE_KEY_SEED_PREFIX  := " + PREFIX
CASES = [("as built", lambda t: t, 0),
         ("mint into .keys", lambda t: t.replace("--out $(LINUX_GUESTS_KEYS)/", "--out .keys/"), 2),
         ("seed prefix dropped", lambda t: t.replace(SEED + "\n", ""), 1),
         ("pub prefix dropped", lambda t: t.replace("CAPSULE_KEY_PUB_PREFIX   := " + PREFIX + "\n", ""), 1),
         ("dir moved out of target", lambda t: t.replace(DIR, "LINUX_GUESTS_KEYS := keys/"), 1)]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--self-test", action="store_true")
    ap.add_argument("--mk", default="userland/linux_guests/Guests.mk")
    a = ap.parse_args()
    text = Path(a.mk).read_text()
    if a.self_test:
        bad = [w for w, f, n in CASES if len(problems(f(text))) != n]
        print(f"guest-keys self-test: {len(CASES)} cases, {len(bad)} wrong {bad if bad else ''}")
        return 1 if bad else 0
    found = problems(text)
    for p in found:
        print(f"guest-keys: {p}")
    print(f"guest-keys: {len(found)} problems")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())
