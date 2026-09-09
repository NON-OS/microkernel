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
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.
"""Which syscalls the kernel dispatches that no userland code can reach.

The failure this catches has one shape: a capability that is implemented,
sits in the dispatch table, and is referenced by nothing. No compiler warns
about it and no test exercises it, so it survives until someone notices the
feature it was meant to serve does not work. In one day that shape was
MkCapsuleVerify, MkAttestDoc and MkFutexWait, plus a filesystem mount that
had never been called.

A syscall counts as reached if any file under userland/ names its tag,
either as tag4(b"XXXX") or as the equivalent little-endian hex literal, since
libc writes one of them that way. The std PAL lives in the pinned rust-src
outside this tree and is not scanned; a syscall reached only from there is
listed in the notes file beside the baseline so nobody chases it.

Prints the unreachable set and its size. The size is what the baseline gate
compares, and it may only go down.
"""

import argparse
import pathlib
import re
import sys

KERNEL_NUMBERS = pathlib.Path("src/syscall/microkernel/numbers.rs")
USERLAND = pathlib.Path("userland")
SKIP = ("/target/", "/vendor/", "/third_party/")

TAG_RE = re.compile(r'pub const (SYS_\w+): u64 = tag4\(b"(....)"\);')


def tag_hex(tag: str) -> str:
    """The little-endian i64 a tag4 call produces, as libc writes it by hand."""
    value = 0
    for shift, byte in enumerate(tag.encode()):
        value |= byte << (8 * shift)
    hex_digits = f"{value:08X}"
    return f"{hex_digits[:4]}_{hex_digits[4:]}"


def kernel_syscalls() -> dict[str, str]:
    text = KERNEL_NUMBERS.read_text()
    return {tag: name for name, tag in TAG_RE.findall(text)}


def userland_text() -> str:
    parts = []
    for path in USERLAND.rglob("*.rs"):
        s = str(path)
        if any(k in s for k in SKIP):
            continue
        parts.append(path.read_text(errors="replace"))
    return "\n".join(parts)


def unreachable(syscalls: dict[str, str], corpus: str) -> list[tuple[str, str]]:
    out = []
    for tag, name in syscalls.items():
        by_tag = f'b"{tag}"' in corpus
        by_hex = tag_hex(tag) in corpus or tag_hex(tag).replace("_", "") in corpus
        if not (by_tag or by_hex):
            out.append((name, tag))
    return sorted(out)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--count-only", action="store_true", help="print the number alone")
    ap.add_argument(
        "--baseline",
        type=pathlib.Path,
        help="fail if the count exceeds the integer in this file",
    )
    args = ap.parse_args()

    if not KERNEL_NUMBERS.exists():
        print("run from the repository root", file=sys.stderr)
        return 2

    dark = unreachable(kernel_syscalls(), userland_text())
    if args.count_only:
        print(len(dark))
        return 0
    for name, tag in dark:
        print(f"{name:<26} {tag}")
    print(f"unreachable from userland: {len(dark)}")
    if args.baseline is None:
        return 0
    allowed = int(args.baseline.read_text().strip())
    if len(dark) > allowed:
        print(
            f"::error::{len(dark)} syscalls are unreachable, baseline allows {allowed}. "
            "A syscall was added or a caller removed without anything reaching it; "
            "wire it, or explain it in scripts/baselines/syscall-unreachable.notes.md "
            "and raise the count in the same commit."
        )
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
