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
"""A run's receipt, read back from its serial log and checked on this host.

The headless Terminal writes it as `[AGENTPROOF-RECEIPT]` rows and acks it
only once they are written (receipt_log.rs). Here it is taken only whole, as
long as its head line says, then `agentproof-receipt verify` checks it, and a
copy with one byte flipped must be refused. With no --trust, the trust is
pinned from this receipt on first use: that proves the receipt sound and
signed by its AK, never that anyone vouched for the AK or the release.
"""

import subprocess
from pathlib import Path

from .receipt_take import take

CRATE = Path(__file__).resolve().parents[2] / "userland/agentproof_receipt"
TOOL = CRATE / "target/release/agentproof-receipt"
# Asked again, the same bytes; asked another, EBUSY; after the ack, the record gone.
RULE = {"again": "same", "other": "-16", "after-ack": "-2"}
def tool(*args) -> subprocess.CompletedProcess:
    return subprocess.run([str(TOOL), *map(str, args)], capture_output=True, text=True)


def check(log: str, out: Path, trust=None) -> dict:
    """What came of the run's receipt; `why` is empty when it verified."""
    receipt, said, rule = take(log)
    r = {"said": said, "rule": rule, "verified": False, "flip_refused": None, "trust": None,
         "why": [] if rule == RULE else [f"the job's rule did not hold on the machine: {rule}"]}
    if receipt is None:
        r["why"].append(f"no whole receipt in the log: {said}")
        return r
    if not TOOL.exists():
        subprocess.run(["cargo", "build", "--release", "-q"], cwd=CRATE, check=True)
    (out / "receipt.napr").write_bytes(receipt)
    if trust is None:
        pinned = tool("pin", out / "receipt.napr")
        if pinned.returncode != 0:
            r["why"].append("the receipt was refused: " + pinned.stderr.strip())
            return r
        trust = out / "trust.pinned"
        trust.write_text(pinned.stdout)
        r["trust"] = "pinned on first use from this receipt; no one vouched for it"
    v = tool("verify", out / "receipt.napr", trust)
    r["verified"], r["verdict"] = v.returncode == 0, v.stdout.splitlines()
    flipped = bytearray(receipt)
    flipped[len(flipped) // 2] ^= 1
    (out / "receipt.flipped").write_bytes(bytes(flipped))
    r["flip_refused"] = tool("verify", out / "receipt.flipped", trust).returncode == 1
    if not r["verified"]:
        r["why"].append("the receipt was refused: " + v.stderr.strip())
    if not r["flip_refused"]:
        r["why"].append("a receipt with one byte flipped was not refused")
    return r
